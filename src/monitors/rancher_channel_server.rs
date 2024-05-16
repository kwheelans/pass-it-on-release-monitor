use async_trait::async_trait;
use crate::error::Error;
use log::{debug};
use serde::Deserialize;
use crate::monitors::Monitor;

#[derive(Deserialize, Debug)]
pub struct RancherChannelServerConfiguration {
    pub url: String,
    pub channel: String,
    pub notification: String,
}

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

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Channels {
    pub id: String,
    pub name: String,
    pub latest: String,
    pub links: Links,
}

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

    async fn notify(&self) -> Result<(), Error> {
        todo!("notify not implemented");
        //Ok(())
    }
    
}


impl RancherChannelServerConfiguration {
    async fn get_channels(&self) -> Result<Collection, Error> {
        let data = reqwest::get(self.url.as_str()).await?.text().await?;
        Ok(serde_json::from_str(data.as_str())?)
    }

    async fn check_channel(&self) -> Result<String, Error> {
        debug!("Checking Rancher Channels for {}", self.url.as_str());
        let channels = self.get_channels().await?;

        debug!("Received Rancher Channels for {}", self.url.as_str());
        let search = self.channel.as_str();

        match channels
            .data
            .into_iter()
            .find(|c| c.id.eq_ignore_ascii_case(search))
        {
            Some(channel) => {
                debug!("Name: {} Version: {}", channel.name, channel.latest);
                Ok(channel.latest)
            }
            None => Err(Error::NoChannelFound(search.to_string())),
        }
    }
    
}
