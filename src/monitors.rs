use crate::configuration::GlobalConfiguration;
use crate::database::MonitorModel;
use crate::database::queries::{select_all_monitors, update_monitor};
use crate::error::Error;
use crate::error::Error::ModelConversionFailed;
use crate::monitors::github_release::{
    GithubConfiguration, GithubConfigurationInner, TYPE_NAME_GITHUB,
};
use crate::monitors::rancher_channel_server::{
    RancherChannelServerConfiguration, RancherChannelServerConfigurationInner,
    TYPE_NAME_RANCHER_CHANNEL,
};
use async_trait::async_trait;
use chrono::TimeDelta;
use pass_it_on::notifications::ClientReadyMessage;
use sea_orm::prelude::ChronoUtc;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, trace, warn};

pub mod github_release;
pub mod rancher_channel_server;

const MONITOR_SLEEP_DURATION: Duration = Duration::from_secs(60);

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait Monitor: CloneMonitor + Send + Debug {
    async fn check(&self, global_config: &GlobalConfiguration) -> Result<ReleaseData, Error>;
    fn message(&self, version: ReleaseData) -> ClientReadyMessage;
    fn monitor_type(&self) -> String;
    fn name(&self) -> String;
    fn frequency(&self) -> TimeDelta;
    fn inner_to_json(&self) -> String;
}

pub trait CloneMonitor {
    fn box_clone(&self) -> Box<dyn Monitor>;
}

impl<T> CloneMonitor for T
where
    T: Monitor + Clone + 'static,
{
    fn box_clone(&self) -> Box<dyn Monitor> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Monitor> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum FrequencyPeriod {
    #[serde(alias = "minute")]
    Minute,
    #[default]
    #[serde(alias = "hour")]
    Hour,
    #[serde(alias = "day")]
    Day,
    #[serde(alias = "week")]
    Week,
}

impl FrequencyPeriod {
    pub fn to_duration(&self, value: u64) -> TimeDelta {
        let value = value as i64;
        match self {
            FrequencyPeriod::Minute => TimeDelta::minutes(value),
            FrequencyPeriod::Hour => TimeDelta::hours(value),
            FrequencyPeriod::Day => TimeDelta::days(value),
            FrequencyPeriod::Week => TimeDelta::weeks(value),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FrequencyValue(u64);

impl Default for FrequencyValue {
    fn default() -> Self {
        Self(1)
    }
}

pub struct ReleaseData {
    pub version: String,
    pub link: Option<String>,
}

async fn get_model_list(db: &DatabaseConnection) -> Result<HashMap<String, MonitorModel>, Error> {
    let selected = select_all_monitors(db).await?;
    Ok(selected.into_iter().map(|m| (m.name.clone(), m)).collect())
}

pub async fn start_monitoring(
    db: &DatabaseConnection,
    global_configs: GlobalConfiguration,
    interface: mpsc::Sender<ClientReadyMessage>,
) -> Result<(), Error> {
    while !interface.is_closed() {
        debug!("Getting all models from database");
        let monitor_models = get_model_list(db).await?;
        if monitor_models.is_empty() {
            warn!("No monitors present in database")
        }

        for (name, model) in &monitor_models {
            let monitor = monitor_from_model(model)?;
            if needs_check(model, monitor.as_ref()) {
                trace!("Monitor needs to be checked: {}", name);
                match monitor.check(&global_configs).await {
                    Ok(release_data) => {
                        debug!("{:?}", model);
                        debug!(
                            "Checked version for: {} --> old: {} |-| new: {}",
                            name,
                            model.version.as_str(),
                            release_data.version.as_str()
                        );
                        if is_new_version(model.version.as_str(), release_data.version.as_str()) {
                            trace!("Found new version: {}", name);
                            let mut active_model = model.clone().into_active_model();
                            active_model.version = Set(release_data.version.clone());
                            active_model.timestamp = Set(ChronoUtc::now().into());
                            if let Err(error) = update_monitor(db, active_model).await {
                                error!("Database Update failed for: {} --> {}", name, error);
                            } else {
                                debug!("Sending notification: {}", name);
                                if let Err(error) =
                                    interface.send(monitor.message(release_data)).await
                                {
                                    warn!("Error sending notification: {} -> {}", name, error)
                                }
                            }
                        } else {
                            trace!("Both old and new version are equal: {}", name);
                        }
                    }
                    Err(error) => {
                        warn!("Unable to check: {} --> {}", name, error)
                    }
                }
            } else {
                trace!("Monitor does not need to be checked: {}", name)
            }
        }
        // Wait before starting to check again
        tokio::time::sleep(MONITOR_SLEEP_DURATION).await;
    }
    Ok(())
}

fn needs_check(model: &MonitorModel, monitor: &dyn Monitor) -> bool {
    let since_last_check = ChronoUtc::now().signed_duration_since(model.timestamp.to_utc());
    since_last_check.ge(&monitor.frequency())
}

fn is_new_version<S: AsRef<str>>(old: S, new: S) -> bool {
    PartialEq::ne(old.as_ref(), new.as_ref())
}

fn monitor_from_model(model: &MonitorModel) -> Result<Box<dyn Monitor>, Error> {
    match model.monitor_type.as_str() {
        TYPE_NAME_GITHUB => Ok(Box::new(GithubConfiguration {
            name: model.name.clone(),
            inner: serde_json::from_str::<GithubConfigurationInner>(model.configuration.as_str())?,
        })),
        TYPE_NAME_RANCHER_CHANNEL => Ok(Box::new(RancherChannelServerConfiguration {
            name: model.name.clone(),
            inner: serde_json::from_str::<RancherChannelServerConfigurationInner>(
                model.configuration.as_str(),
            )?,
        })),
        _ => Err(ModelConversionFailed),
    }
}
