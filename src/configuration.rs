use crate::monitors::Monitor;
use pass_it_on::ClientConfigFile;
use serde::{Deserialize, Serialize};

const DEFAULT_DATA_PATH: &str = "release-monitor.sqlite";

#[derive(Debug, Deserialize)]
pub struct ReleaseMonitorConfiguration {
    #[serde(default)]
    pub global: GlobalConfiguration,
    pub monitors: MonitorConfiguration,
    pub client: ClientConfigFile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorConfiguration {
    pub monitor: Vec<Box<dyn Monitor>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct GlobalConfiguration {
    pub persist: bool,
    pub db_path: String,
    pub web_ui_port: u16,
    pub web_ui_address: String,
    pub github_personal_token: Option<String>,
}

impl Default for GlobalConfiguration {
    fn default() -> Self {
        Self {
            persist: true,
            db_path: DEFAULT_DATA_PATH.to_string(),
            web_ui_port: 8080,
            web_ui_address: "0.0.0.0".to_string(),
            github_personal_token: None,
        }
    }
}

impl GlobalConfiguration {
    pub fn db_uri(&self) -> String {
        format!("{}{}{}", "sqlite://", self.db_path.as_str(), "?mode=rwc")
    }
}

impl TryFrom<&str> for ReleaseMonitorConfiguration {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}
