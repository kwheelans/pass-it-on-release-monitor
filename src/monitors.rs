use crate::error::Error;
use crate::error::Error::NoMonitors;
use async_trait::async_trait;
use pass_it_on::notifications::ClientReadyMessage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tracing::{debug, trace, warn};
use crate::configuration::GlobalConfiguration;

pub mod github_release;
pub mod rancher_channel_server;

#[async_trait]
#[typetag::deserialize(tag = "type")]
pub trait Monitor: Send + Debug {
    async fn check(&self) -> Result<ReleaseData, Error>;
    fn message(&self, version: ReleaseData) -> ClientReadyMessage;
    fn monitor_type(&self) -> String;
    fn monitor_id(&self) -> String;
    fn frequency(&self) -> Duration;
    fn set_global_configs(&mut self, configs: &GlobalConfiguration);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MonitorStore {
    pub monitor_id: String,
    pub version: String,
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

    pub fn new_version(&mut self, latest: &str) -> bool {
        let update = self.version.as_str().ne(latest);
        if update {
            self.version = latest.to_string();
            self.last_check = Instant::now();
        }
        update
    }
}

pub struct ReleaseData {
    pub version: String,
    pub link: Option<String>,
}

async fn create_monitor_list(
    monitors: Vec<Box<dyn Monitor>>,
    persist_path: Option<PathBuf>,
    global_configs: GlobalConfiguration,
) -> Result<HashMap<String, ReleaseTracker>, Error> {
    let mut list = HashMap::with_capacity(monitors.len());
    let stored_versions: Option<HashMap<String, String>> = match persist_path {
        Some(path) if path.exists() => {
            let reader = std::io::BufReader::new(File::open(path)?);
            let stored: Vec<MonitorStore> = serde_json::from_reader(reader)?;
            Some(
                stored
                    .into_iter()
                    .map(|x| (x.monitor_id, x.version))
                    .collect(),
            )
        }
        _ => None,
    };

    debug!("{:?}", stored_versions);

    for mut monitor in monitors {
        monitor.set_global_configs(&global_configs);
        match monitor.check().await {
            Ok(release_data) => {
                let version = match &stored_versions {
                    None => {
                        debug!(
                            "Initial check got {} for {}",
                            release_data.version,
                            monitor.monitor_id()
                        );
                        release_data.version
                    }
                    Some(stored) => {
                        let stored_version = stored.get(&monitor.monitor_id());
                        match stored_version {
                            None => release_data.version,
                            Some(stored_version) => {
                                debug!(
                                    "Using stored version {} for {}",
                                    stored_version,
                                    monitor.monitor_id()
                                );
                                stored_version.to_string()
                            }
                        }
                    }
                };
                list.insert(monitor.monitor_id(), ReleaseTracker::new(monitor, version));
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
    global_configs: GlobalConfiguration,
) -> Result<(), Error> {
    let mut startup = true;
    let mut update_store = true;
    let persist_path = match global_configs.persist {
        true => Some(global_configs.data_path.as_str().into()),
        false => None,
    };
    debug!("Persistence settings: {:?}", persist_path);
    let mut monitor_list = create_monitor_list(monitors, persist_path.clone(), global_configs).await?;
    

    while !interface.is_closed() {
        tokio::time::sleep(Duration::from_secs(60)).await;

        for (monitor_id, tracker) in &mut monitor_list {
            match tracker.needs_check() || startup {
                true => {
                    trace!("{}: check required", monitor_id.as_str());
                    match tracker.monitor.check().await {
                        Ok(latest) => {
                            debug!(
                                "{}: Checking previous version: {} |-| latest version: {}",
                                monitor_id.as_str(),
                                tracker.version.as_str(),
                                latest.version.as_str()
                            );
                            match tracker.new_version(latest.version.as_str()) {
                                true => {
                                    update_store = true;
                                    debug!("{}: Sending notification", monitor_id.as_str());
                                    if let Err(error) =
                                        interface.send(tracker.monitor.message(latest)).await
                                    {
                                        warn!(
                                            "{}: Error sending notification -> {}",
                                            monitor_id.as_str(),
                                            error
                                        )
                                    }
                                }

                                false => trace!(
                                    "{}: Both previous and latest version are equal",
                                    monitor_id.as_str()
                                ),
                            }
                        }
                        Err(error) => {
                            warn!("{}: Unable to check -> {}", monitor_id.as_str(), error)
                        }
                    };
                }
                false => trace!("{}: check not required", monitor_id.as_str()),
            }
        }

        if (update_store || startup) && persist_path.is_some() {
            debug!(
                "Updating stored values to file {}",
                persist_path.as_ref().unwrap().to_string_lossy()
            );
            let file = File::options()
                .write(true)
                .truncate(true)
                .create(true)
                .open(persist_path.as_ref().unwrap())?;
            let writer = std::io::BufWriter::new(file);
            let monitor_store: Vec<MonitorStore> = monitor_list
                .iter()
                .map(|(id, tracker)| MonitorStore {
                    monitor_id: id.to_string(),
                    version: tracker.version.to_string(),
                })
                .collect();
            serde_json::to_writer(writer, &monitor_store)?;
            update_store = false;
        }
        startup = false;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::monitors::rancher_channel_server::RancherChannelServerConfiguration;
    use crate::monitors::{FrequencyPeriod, FrequencyValue, Monitor, ReleaseTracker};

    fn create_rancher_channel_monitor() -> Box<dyn Monitor> {
        Box::new(RancherChannelServerConfiguration {
            url: "https://example.com".to_string(),
            channel: "stable".to_string(),
            notification: "test".to_string(),
            frequency: FrequencyValue(1),
            period: FrequencyPeriod::Hour,
        })
    }

    #[test]
    fn is_new_release() {
        let mut tracker =
            ReleaseTracker::new(create_rancher_channel_monitor(), "v1.0.0-rc1".to_string());
        assert!(tracker.new_version("v1.1.0-rc1"))
    }

    #[test]
    fn is_not_new_release() {
        let mut tracker =
            ReleaseTracker::new(create_rancher_channel_monitor(), "v1.0.0-rc1".to_string());
        assert!(!tracker.new_version("v1.0.0-rc1"))
    }

    #[test]
    fn release_tracker_is_updated() {
        const NEW_VERSION: &str = "v1.1.0-rc1";
        let mut tracker =
            ReleaseTracker::new(create_rancher_channel_monitor(), "v1.0.0-rc1".to_string());
        tracker.new_version(NEW_VERSION);
        assert_eq!(tracker.version, NEW_VERSION.to_string())
    }
}
