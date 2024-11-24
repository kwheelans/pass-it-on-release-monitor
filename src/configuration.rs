use pass_it_on::ClientConfigFile;
use serde::Deserialize;

use crate::monitors::Monitor;

const DEFAULT_DATA_PATH: &str = "release-tracking.json";

#[derive(Deserialize)]
pub struct MonitorConfigFileParser {
    pub global: GlobalConfiguration,
    pub monitors: MonitorConfiguration,
    pub client: ClientConfigFile,
}

#[derive(Deserialize)]
pub struct MonitorConfiguration {
    pub monitor: Vec<Box<dyn Monitor>>,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct GlobalConfiguration {
    pub persist: bool,
    pub data_path: String,
}

impl Default for GlobalConfiguration {
    fn default() -> Self {
        Self {
            persist: false,
            data_path: DEFAULT_DATA_PATH.to_string(),
        }
    }
}

impl TryFrom<&str> for MonitorConfigFileParser {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}
