use serde::Deserialize;
use crate::error::Error;

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
 pub links: Links
}

#[derive(Deserialize, Debug)]
pub struct Links {
 #[serde(rename = "self")]
 pub link: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Actions;

pub async fn get_channels(url: &str) -> Result<Collection, Error> {
 let data = reqwest::get(url).await?.text().await?;
 Ok(serde_json::from_str(data.as_str())?)
}
