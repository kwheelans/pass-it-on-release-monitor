use pass_it_on::ClientConfigFile;
use serde::Deserialize;
use crate::monitors::rancher_channel_server::RancherChannelServerConfiguration;

#[derive(Deserialize)]
pub struct MonitorConfigFileParser {
    pub monitor: MonitorConfiguration,
    pub client: ClientConfigFile,
}

#[derive(Deserialize)]
pub struct MonitorConfiguration {
    #[serde(rename = "rancher-channel-server")]
    pub channel_server: Vec<RancherChannelServerConfiguration>,
}

impl TryFrom<&str> for MonitorConfigFileParser {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}
