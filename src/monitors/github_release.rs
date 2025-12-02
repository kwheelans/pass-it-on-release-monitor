use crate::configuration::GlobalConfiguration;
use crate::error::Error;
use crate::monitors::{FrequencyPeriod, FrequencyValue, Monitor, ReleaseData};
use async_trait::async_trait;
use chrono::TimeDelta;
use pass_it_on::notifications::{ClientReadyMessage, Message};
use serde::{Deserialize, Serialize};
use tracing::trace;

pub const TYPE_NAME_GITHUB: &str = "github";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubConfiguration {
    pub name: String,
    #[serde(flatten)]
    pub inner: GithubConfigurationInner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubConfigurationInner {
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
#[typetag::serde(name = "github")]
impl Monitor for GithubConfiguration {
    async fn check(&self, global_config: &GlobalConfiguration) -> Result<ReleaseData, Error> {
        self.get_latest_release(global_config).await
    }

    fn message(&self, version: ReleaseData) -> ClientReadyMessage {
        Message::new(format!(
            "Release {} now available for {}/{}. {}",
            version.version,
            self.inner.owner.as_str(),
            self.inner.repo.as_str(),
            version.link.unwrap_or_default()
        ))
        .to_client_ready_message(self.inner.notification.as_str())
    }

    fn monitor_type(&self) -> String {
        TYPE_NAME_GITHUB.to_string()
    }

    fn name(&self) -> String {
        self.name.to_string()
    }

    fn frequency(&self) -> TimeDelta {
        self.inner.period.to_duration(self.inner.frequency.0)
    }

    fn inner_to_json(&self) -> String {
        serde_json::to_string(&self.inner).expect("monitor to_json failed")
    }
}

impl GithubConfiguration {
    async fn get_latest_release(
        &self,
        global_config: &GlobalConfiguration,
    ) -> Result<ReleaseData, Error> {
        trace!(
            "Checking Github latest release for repository {}/{}",
            self.inner.repo.as_str(),
            self.inner.owner.as_str()
        );
        let release = {
            let instance = match self.get_github_personal_token(global_config) {
                None => octocrab::OctocrabBuilder::default().build()?,
                Some(token) => octocrab::OctocrabBuilder::default()
                    .personal_token(token.as_str())
                    .build()?,
            };
            instance
                .repos(self.inner.owner.as_str(), self.inner.repo.as_str())
                .releases()
                .get_latest()
                .await?
        };
        trace!(
            "Found Github latest release {} for repository {}/{}",
            release.tag_name.as_str(),
            self.inner.repo.as_str(),
            self.inner.owner.as_str()
        );

        Ok(ReleaseData {
            version: release.tag_name,
            link: Some(release.html_url.to_string()),
        })
    }

    fn get_github_personal_token(&self, global_config: &GlobalConfiguration) -> Option<String> {
        if self.inner.github_personal_token.is_some() {
            self.inner.github_personal_token.clone()
        } else if global_config.github_personal_token.is_some() {
            global_config.github_personal_token.clone()
        } else {
            None
        }
    }
}
