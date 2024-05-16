use crate::error::Error;
use log::{debug};
use serde::Deserialize;

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

async fn get_channels(url: &str) -> Result<Collection, Error> {
    let data = reqwest::get(url).await?.text().await?;
    Ok(serde_json::from_str(data.as_str())?)
}

pub async fn check_channel(config: &RancherChannelServerConfiguration) -> Result<String, Error> {
    debug!("Checking Rancher Channels for {}", config.url.as_str());
    let channels = get_channels(config.url.as_str()).await?;
    
    debug!("Received Rancher Channels");
    let search = config.channel.as_str();

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
