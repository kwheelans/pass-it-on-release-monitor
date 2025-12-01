use crate::configuration::GlobalConfiguration;
use crate::database::queries::select_all_monitors;
use crate::database::{MonitorActiveModel, MonitorEntity, MonitorModel, monitors};
use crate::error::Error;
use crate::error::Error::{ModelConversionFailed, NoMonitors};
use crate::monitors::github_release::{
    GithubConfiguration, GithubConfigurationInner, TYPE_NAME_GITHUB,
};
use crate::monitors::rancher_channel_server::{
    RancherChannelServerConfiguration, TYPE_NAME_RANCHER_CHANNEL,
};
use async_trait::async_trait;
use chrono::{DateTime, TimeDelta, Utc};
use pass_it_on::notifications::ClientReadyMessage;
use sea_orm::prelude::ChronoUtc;
use sea_orm::sea_query::OnConflict;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, trace, warn};

pub mod github_release;
pub mod rancher_channel_server;

const MONITOR_SLEEP_DURATION: Duration = Duration::from_secs(60);

#[async_trait]
#[typetag::serde(tag = "type")]
pub trait Monitor: Send + Debug {
    async fn check(&self, global_config: &GlobalConfiguration) -> Result<ReleaseData, Error>;
    fn message(&self, version: ReleaseData) -> ClientReadyMessage;
    fn monitor_type(&self) -> String;
    fn name(&self) -> String;
    fn frequency(&self) -> TimeDelta;
    fn to_json(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Deserialize)]
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

struct ReleaseTracker {
    monitor: Box<dyn Monitor>,
    version: String,
    last_check: DateTime<Utc>,
}

impl ReleaseTracker {
    pub fn new(monitor: Box<dyn Monitor>, version: String) -> Self {
        Self {
            monitor,
            version,
            last_check: ChronoUtc::now(),
        }
    }

    pub fn needs_check(&self) -> bool {
        ChronoUtc::now()
            .signed_duration_since(self.last_check)
            .abs()
            .ge(&self.monitor.frequency().abs())
    }

    pub fn new_version(&mut self, latest: &str) -> bool {
        let update = PartialEq::ne(self.version.as_str(), latest);
        if update {
            self.version = latest.to_string();
            self.last_check = ChronoUtc::now();
        }
        update
    }
}

pub async fn monitor(
    monitors: Vec<Box<dyn Monitor>>,
    interface: mpsc::Sender<ClientReadyMessage>,
    global_configs: GlobalConfiguration,
    db: DatabaseConnection,
) -> Result<(), Error> {
    let mut startup = true;
    let mut update_store = true;
    let conflict_id_update = OnConflict::column(monitors::Column::Id)
        .update_column(monitors::Column::Version)
        .to_owned();

    debug!("Getting monitor_list with create_monitor_list");
    let mut monitor_list = create_monitor_list(monitors, &db, &global_configs).await?;

    while !interface.is_closed() {
        tokio::time::sleep(MONITOR_SLEEP_DURATION).await;

        for (monitor_id, tracker) in &mut monitor_list {
            match tracker.needs_check() || startup {
                true => {
                    trace!("{}: check required", monitor_id.as_str());
                    match tracker.monitor.check(&global_configs).await {
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

        if update_store || startup {
            debug!("Updating in memory values to database",);
            let monitor_store: Vec<_> = monitor_list
                .iter()
                .map(|(id, tracker)| monitor_entity(id, id, tracker.version.as_str()))
                .collect();

            MonitorEntity::insert_many(monitor_store)
                .on_conflict(conflict_id_update.clone())
                .exec(&db)
                .await?;
            update_store = false;
        }
        startup = false;
    }

    Ok(())
}

async fn create_monitor_list(
    monitors: Vec<Box<dyn Monitor>>,
    db: &DatabaseConnection,
    global_configs: &GlobalConfiguration,
) -> Result<HashMap<String, ReleaseTracker>, Error> {
    let mut list = HashMap::with_capacity(monitors.len());
    let stored_versions: HashMap<String, String> = {
        let selected = select_all_monitors(db).await?;
        debug!("selected: {:?}", selected);
        selected.into_iter().map(|x| (x.name, x.version)).collect()
    };

    let selected = select_all_monitors(db).await?;
    for s in selected {
        let x = monitor_from_model(s)?;
    }

    debug!("stored_versions from database: {:?}", stored_versions);

    for monitor in monitors {
        // Initial check of monitors
        match monitor.check(global_configs).await {
            Ok(release_data) => {
                let stored_version = stored_versions.get(&monitor.name());
                let version = match stored_version {
                    None => {
                        debug!(
                            "Initial check got {} for {}",
                            release_data.version,
                            monitor.name()
                        );
                        release_data.version
                    }
                    Some(stored_version) => {
                        debug!(
                            "Using stored version {} for {}",
                            stored_version,
                            monitor.name()
                        );
                        stored_version.to_string()
                    }
                };
                list.insert(monitor.name(), ReleaseTracker::new(monitor, version));
            }
            Err(error) => {
                warn!("Unable to add {} due to: {}", monitor.name(), error)
            }
        }
    }

    match list.is_empty() {
        false => Ok(list),
        true => Err(NoMonitors),
    }
}

fn monitor_from_model(model: MonitorModel) -> Result<Box<dyn Monitor>, Error> {
    match model.monitor_type.as_str() {
        TYPE_NAME_GITHUB => Ok(Box::new(GithubConfiguration {
            name: model.name,
            inner: serde_json::from_str::<GithubConfigurationInner>(model.configuration.as_str())?,
        })),
        TYPE_NAME_RANCHER_CHANNEL => Ok(Box::new(serde_json::from_str::<
            RancherChannelServerConfiguration,
        >(model.configuration.as_str())?)),
        _ => Err(ModelConversionFailed),
    }
}

fn monitor_entity(name: &str, monitor_type: &str, version: &str) -> MonitorActiveModel {
    MonitorActiveModel {
        id: Default::default(),
        name: ActiveValue::Unchanged(name.to_owned()),
        monitor_type: ActiveValue::Unchanged(monitor_type.to_owned()),
        version: ActiveValue::Set(version.to_owned()),
        configuration: Default::default(),
        timestamp: Default::default(),
    }
}

/* TODO: Fix tests after completing changes
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
*/
