use crate::error::Error;
use crate::monitors::{FrequencyPeriod, FrequencyValue, Monitor, ReleaseData};
use async_trait::async_trait;
use pass_it_on::notifications::{ClientReadyMessage, Message};
use serde::Deserialize;
use std::time::Duration;
use tracing::trace;
use crate::configuration::GlobalConfiguration;

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
    #[serde(default)]
    pub github_personal_token: Option<String>,
}

#[async_trait]
#[typetag::deserialize(name = "github")]
impl Monitor for GithubConfiguration {
    async fn check(&self) -> Result<ReleaseData, Error> {
        self.get_latest_release().await
    }

    fn message(&self, version: ReleaseData) -> ClientReadyMessage {
        Message::new(format!(
            "Release {} now available for {}/{}. {}",
            version.version,
            self.owner.as_str(),
            self.repo.as_str(),
            version.link.unwrap_or_default()
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
            self.owner.as_str(),
            self.repo.as_str()
        )
    }

    fn frequency(&self) -> Duration {
        self.period.to_duration(self.frequency.0)
    }

    fn set_global_configs(&mut self, configs: &GlobalConfiguration) {
        if self.github_personal_token.is_none() && configs.github_personal_token.is_some() {
            self.github_personal_token = configs.github_personal_token.clone();
        }
    }
}

impl GithubConfiguration {
    async fn get_latest_release(&self) -> Result<ReleaseData, Error> {
        trace!(
            "Checking Github latest release for repository {}/{}",
            self.repo.as_str(),
            self.owner.as_str()
        );
        let release = {
            let instance = match &self.github_personal_token {
                None => octocrab::OctocrabBuilder::default().build()?,
                Some(token) => octocrab::OctocrabBuilder::default().personal_token(token.as_str()).build()?

            };
            instance
                .repos(self.owner.as_str(), self.repo.as_str())
                .releases()
                .get_latest()
                .await?
        };
        trace!(
            "Found Github latest release {} for repository {}/{}",
            release.tag_name.as_str(),
            self.repo.as_str(),
            self.owner.as_str()
        );

        Ok(ReleaseData {
            version: release.tag_name,
            link: Some(release.html_url.to_string()),
        })
    }
}
