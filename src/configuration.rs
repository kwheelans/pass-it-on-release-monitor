use pass_it_on::ClientConfigFile;
use serde::Deserialize;

use crate::monitors::Monitor;

#[derive(Deserialize)]
pub struct MonitorConfigFileParser {
    pub monitors: MonitorConfiguration,
    pub client: ClientConfigFile,
}

#[derive(Deserialize)]
pub struct MonitorConfiguration {
    pub monitor: Vec<Box<dyn Monitor>>,
}

impl TryFrom<&str> for MonitorConfigFileParser {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}
