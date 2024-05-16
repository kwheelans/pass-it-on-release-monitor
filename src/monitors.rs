use async_trait::async_trait;
use crate::error::Error;

pub mod rancher_channel_server;

#[async_trait]
#[typetag::deserialize(tag = "type")]
pub trait Monitor {
    async fn check(&self) -> Result<String, Error>;
    async fn notify(&self) -> Result<(), Error>;
}