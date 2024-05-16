use serde::Deserialize;

#[derive(Deserialize)]
pub struct MonitorConfigFileParser {
    pub monitor: MonitorConfiguration,
    //pub client: ClientConfigFile,
}

#[derive(Deserialize)]
pub struct MonitorConfiguration {
    
}


impl TryFrom<&str> for MonitorConfigFileParser {
    type Error = toml::de::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        toml::from_str(value)
    }
}