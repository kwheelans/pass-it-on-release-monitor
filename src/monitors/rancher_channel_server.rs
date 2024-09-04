use crate::error::Error;
use crate::monitors::{FrequencyPeriod, FrequencyValue, Monitor};
use async_trait::async_trait;
use log::trace;
use pass_it_on::notifications::{ClientReadyMessage, Message};
use serde::Deserialize;
use std::time::Duration;

const TYPE_NAME: &str = "rancher-channel";

#[derive(Deserialize, Debug)]
pub struct RancherChannelServerConfiguration {
    pub url: String,
    pub channel: String,
    pub notification: String,
    #[serde(default)]
    pub frequency: FrequencyValue,
    #[serde(default)]
    pub period: FrequencyPeriod,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Channels {
    pub id: String,
    pub name: String,
    pub latest: String,
    pub links: Links,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Links {
    #[serde(rename = "self")]
    pub link: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Actions;

#[async_trait]
#[typetag::deserialize(name = "rancher-channel")]
impl Monitor for RancherChannelServerConfiguration {
    async fn check(&self) -> Result<String, Error> {
        self.check_channel().await
    }

    fn message(&self, version: &str) -> ClientReadyMessage {
        Message::new(format!(
            "Version {} now available for channel {} at {}",
            version,
            self.channel.as_str(),
            self.url.as_str()
        ))
        .to_client_ready_message(self.notification.as_str())
    }

    fn monitor_type(&self) -> String {
        TYPE_NAME.to_string()
    }

    fn monitor_id(&self) -> String {
        format!(
            "{}-{}-{}",
            self.monitor_type(),
            self.channel.as_str(),
            self.url.as_str()
        )
    }

    fn frequency(&self) -> Duration {
        self.period.to_duration(self.frequency.0)
    }
}

impl RancherChannelServerConfiguration {
    async fn get_channels(&self) -> Result<Collection, Error> {
        let data = reqwest::get(self.url.as_str()).await?.text().await?;
        Ok(serde_json::from_str(data.as_str())?)
    }

    async fn check_channel(&self) -> Result<String, Error> {
        trace!("Checking Rancher Channels for {}", self.monitor_id());
        let channels = self.get_channels().await?;

        trace!("Received Rancher Channels for {}", self.monitor_id());
        let search = self.channel.as_str();

        match channels
            .data
            .into_iter()
            .find(|c| c.id.eq_ignore_ascii_case(search))
        {
            Some(channel) => {
                trace!("Name: {} Version: {}", channel.name, channel.latest);
                Ok(channel.latest)
            }
            None => Err(Error::RancherChannelNotFound(search.to_string())),
        }
    }
}
