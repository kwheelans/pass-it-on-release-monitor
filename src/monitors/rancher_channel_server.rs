use crate::configuration::GlobalConfiguration;
use crate::error::Error;
use crate::monitors::{FrequencyPeriod, FrequencyValue, Monitor, ReleaseData};
use async_trait::async_trait;
use pass_it_on::notifications::{ClientReadyMessage, Message};
use serde::{Deserialize, Serialize};
use chrono::TimeDelta;
use tracing::trace;

pub const TYPE_NAME_RANCHER_CHANNEL: &str = "rancher-channel";

#[derive(Debug, Serialize, Deserialize)]
pub struct RancherChannelServerConfiguration {
    pub name: String,
    #[serde(flatten)]
    pub inner: RancherChannelServerConfigurationInner,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct RancherChannelServerConfigurationInner {
    pub url: String,
    pub channel: String,
    pub notification: String,
    #[serde(default)]
    pub frequency: FrequencyValue,
    #[serde(default)]
    pub period: FrequencyPeriod,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Collection {
    pub links: Links,
    pub data: Vec<Channels>,
    #[serde(skip)]
    pub actions: Actions,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub struct Channels {
    pub id: String,
    pub name: String,
    pub latest: String,
    pub links: Links,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub link: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Actions;

#[async_trait]
#[typetag::serde(name = "rancher-channel")]
impl Monitor for RancherChannelServerConfiguration {
    async fn check(&self, _github_configs: &GlobalConfiguration) -> Result<ReleaseData, Error> {
        self.check_channel().await
    }

    fn message(&self, version: ReleaseData) -> ClientReadyMessage {
        Message::new(format!(
            "Version {} now available for channel {} at {}",
            version.version,
            self.inner.channel.as_str(),
            self.inner.url.as_str()
        ))
        .to_client_ready_message(self.inner.notification.as_str())
    }

    fn monitor_type(&self) -> String {
        TYPE_NAME_RANCHER_CHANNEL.to_string()
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn frequency(&self) -> TimeDelta {
        self.inner.period.to_duration(self.inner.frequency.0)
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self.inner).expect("monitor to_json failed")
    }
}

impl RancherChannelServerConfiguration {
    async fn get_channels(&self) -> Result<Collection, Error> {
        let data = reqwest::get(self.inner.url.as_str()).await?.text().await?;
        Ok(serde_json::from_str(data.as_str())?)
    }

    async fn check_channel(&self) -> Result<ReleaseData, Error> {
        trace!("Checking Rancher Channels for {}", self.name());
        let channels = self.get_channels().await?;

        trace!("Received Rancher Channels for {}", self.name());
        let search = self.inner.channel.as_str();

        match channels
            .data
            .into_iter()
            .find(|c| c.id.eq_ignore_ascii_case(search))
        {
            Some(channel) => {
                trace!("Name: {} Version: {}", channel.name, channel.latest);
                Ok(ReleaseData {
                    version: channel.latest,
                    link: None,
                })
            }
            None => Err(Error::RancherChannelNotFound(search.to_string())),
        }
    }
}
