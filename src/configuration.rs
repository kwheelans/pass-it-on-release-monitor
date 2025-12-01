use crate::monitors::Monitor;
use pass_it_on::ClientConfigFile;
use serde::{Deserialize, Serialize};

const DEFAULT_DATA_PATH: &str = "sqlite://release-monitor.sqlite";

#[derive(Debug, Deserialize)]
pub struct ReleaseMonitorConfiguration {
    #[serde(default)]
    pub global: GlobalConfiguration,
    pub monitors: MonitorConfiguration,
    pub client: ClientConfigFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorConfiguration {
    pub monitor: Vec<Box<dyn Monitor>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalConfiguration {
    pub persist: bool,
    pub uri: String,
    pub github_personal_token: Option<String>,
}

impl Default for GlobalConfiguration {
    fn default() -> Self {
        Self {
            persist: true,
            uri: DEFAULT_DATA_PATH.to_string(),
            github_personal_token: None,
        }
    }
}

impl TryFrom<&str> for ReleaseMonitorConfiguration {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}
