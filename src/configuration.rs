use crate::monitors::Monitor;
use pass_it_on::ClientConfigFile;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumString};

const DEFAULT_DATA_PATH: &str = "release-monitor.sqlite";
const PICO_CSS_CDN_BASE: &str = "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/";

#[derive(Debug, Clone, Copy, EnumString, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum PicoCssColour {
    Amber,
    Blue,
    Cyan,
    Fuchsia,
    Green,
    Grey,
    Indigo,
    Jade,
    Lime,
    Orange,
    Pink,
    Pumpkin,
    Purple,
    Red,
    Sand,
    Slate,
    Violet,
    Yellow,
    Zinc,
}

impl PicoCssColour {
    pub fn get_pico_css_name(&self) -> String {
        format!("pico.classless.{}.min.css", self.as_ref())
    }
}

#[derive(Debug, Deserialize)]
pub struct ReleaseMonitorConfiguration {
    #[serde(default)]
    pub global: GlobalConfiguration,
    pub monitors: MonitorConfiguration,
    pub client: ClientConfigFile,
    pub webui: WebUiConfiguration,
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
    pub github_personal_token: Option<String>,
}

impl Default for GlobalConfiguration {
    fn default() -> Self {
        Self {
            persist: true,
            db_path: DEFAULT_DATA_PATH.to_string(),
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct WebUiConfiguration {
    pub port: u16,
    pub listen_address: String,
    pub pico_css_base_path: String,
    pub pico_css_color: PicoCssColour,
}

impl Default for WebUiConfiguration {
    fn default() -> Self {
        Self {
            port: 8080,
            listen_address: "0.0.0.0".to_string(),
            pico_css_base_path: PICO_CSS_CDN_BASE.into(),
            pico_css_color: PicoCssColour::Indigo,
        }
    }
}

pub fn get_css_path(path: String, colour: PicoCssColour) -> String {
    format!("{}{}", path, colour.get_pico_css_name())
}
