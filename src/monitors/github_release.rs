use crate::error::Error;
use crate::monitors::{FrequencyPeriod, FrequencyValue, Monitor};
use async_trait::async_trait;
use log::trace;
use pass_it_on::notifications::{ClientReadyMessage, Message};
use serde::Deserialize;
use std::time::Duration;

const TYPE_NAME: &str = "github";

#[derive(Deserialize, Debug)]
pub struct GithubConfiguration {
    pub owner: String,
    pub repo: String,
    pub notification: String,
    #[serde(default)]
    pub frequency: FrequencyValue,
    #[serde(default)]
    pub period: FrequencyPeriod,
}

#[async_trait]
#[typetag::deserialize(name = "github")]
impl Monitor for GithubConfiguration {
    async fn check(&self) -> Result<String, Error> {
        self.get_latest_release().await
    }

    fn message(&self, version: &str) -> ClientReadyMessage {
        Message::new(format!(
            "Release {} now available for {}/{}",
            version,
            self.owner.as_str(),
            self.repo.as_str()
        ))
        .to_client_ready_message(self.notification.as_str())
    }

    fn monitor_type(&self) -> String {
        TYPE_NAME.to_string()
    }

    fn monitor_id(&self) -> String {
        format!("{}-{}", self.monitor_type(), self.repo.as_str())
    }

    fn frequency(&self) -> Duration {
        self.period.to_duration(self.frequency.0)
    }
}

impl GithubConfiguration {
    async fn get_latest_release(&self) -> Result<String, Error> {
        trace!(
            "Checking Github latest release for repository {}/{}",
            self.repo.as_str(),
            self.owner.as_str()
        );
        let release = octocrab::instance()
            .repos(self.owner.as_str(), self.repo.as_str())
            .releases()
            .get_latest()
            .await?;
        trace!(
            "Found Github latest release {} for repository {}/{}",
            release.tag_name.as_str(),
            self.repo.as_str(),
            self.owner.as_str()
        );
        Ok(release.tag_name)
    }
}
