use crate::error::Error;
use crate::error::Error::NoMonitors;
use async_trait::async_trait;
use log::{debug, trace, warn};
use pass_it_on::notifications::ClientReadyMessage;
use serde::Deserialize;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

pub mod rancher_channel_server;

#[async_trait]
#[typetag::deserialize(tag = "type")]
pub trait Monitor: Send {
    async fn check(&self) -> Result<String, Error>;
    fn message(&self, version: &str) -> ClientReadyMessage;
    fn monitor_type(&self) -> String;
    fn monitor_id(&self) -> String;
    fn frequency(&self) -> Duration;
}

#[derive(Deserialize, Debug)]
pub enum FrequencyPeriod {
    #[serde(alias = "minute")]
    Minute,
    #[serde(alias = "hour")]
    Hour,
    #[serde(alias = "day")]
    Day,
    #[serde(alias = "week")]
    Week,
}

impl Default for FrequencyPeriod {
    fn default() -> Self {
        Self::Hour
    }
}

impl FrequencyPeriod {
    pub fn to_duration(&self, value: u64) -> Duration {
        match self {
            FrequencyPeriod::Minute => Duration::from_secs(60 * value),
            FrequencyPeriod::Hour => Duration::from_secs(3600 * value),
            FrequencyPeriod::Day => Duration::from_secs(86400 * value),
            FrequencyPeriod::Week => Duration::from_secs(604800 * value),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct FrequencyValue(u64);

impl Default for FrequencyValue {
    fn default() -> Self {
        Self(1)
    }
}

struct ReleaseTracker {
    monitor: Box<dyn Monitor>,
    version: String,
    last_check: Instant,
}

impl ReleaseTracker {
    pub fn new(monitor: Box<dyn Monitor>, version: String) -> Self {
        Self {
            monitor,
            version,
            last_check: Instant::now(),
        }
    }

    pub fn needs_check(&self) -> bool {
        Instant::now()
            .duration_since(self.last_check)
            .ge(&self.monitor.frequency())
    }

    pub fn new_version(&mut self, latest: String) -> bool {
        let update = self.version.eq(&latest);
        if update {
            self.version = latest;
            self.last_check = Instant::now();
        }
        update
    }
}

async fn create_monitor_list(
    monitors: Vec<Box<dyn Monitor>>,
) -> Result<Vec<ReleaseTracker>, Error> {
    let mut list = Vec::new();
    for monitor in monitors {
        match monitor.check().await {
            Ok(version) => {
                debug!("Initial check got {} for {}", version, monitor.monitor_id());
                list.push(ReleaseTracker::new(monitor, version));
            }
            Err(error) => {
                warn!("Unable to add {} due to: {}", monitor.monitor_id(), error)
            }
        }
    }

    match list.is_empty() {
        false => Ok(list),
        true => Err(NoMonitors),
    }
}

pub async fn monitor(
    monitors: Vec<Box<dyn Monitor>>,
    interface: mpsc::Sender<ClientReadyMessage>,
) -> Result<(), Error> {
    let mut monitor_list = create_monitor_list(monitors).await?;

    while !interface.is_closed() {
        tokio::time::sleep(Duration::from_secs(60)).await;

        for tracker in monitor_list.as_mut_slice() {
            match tracker.needs_check() {
                true => {
                    trace!("{}: Needs check", tracker.monitor.monitor_id());
                    match tracker.monitor.check().await {
                        Ok(latest) => {
                            debug!(
                                "previous version:{} |-| latest version: {}",
                                tracker.version.as_str(),
                                latest.as_str()
                            );
                            match tracker.new_version(latest) {
                                true => {
                                    if let Err(error) = interface
                                        .send(tracker.monitor.message(tracker.version.as_str()))
                                        .await
                                    {
                                        warn!("Error sending notification: {}", error)
                                    }
                                }

                                false => trace!("Previous version is the same as latest version"),
                            }
                        }
                        Err(error) => warn!(
                            "Unable to check {}: {}",
                            tracker.monitor.monitor_id(),
                            error
                        ),
                    };
                }
                false => trace!(
                    "{} does not need to be checked yet",
                    tracker.monitor.monitor_id()
                ),
            }
        }
    }

    Ok(())
}
