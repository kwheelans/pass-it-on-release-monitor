use crate::database::queries::add_monitor;
use crate::monitors::Monitor;
use crate::monitors::github_release::{
    GithubConfiguration, GithubConfigurationInner, TYPE_NAME_GITHUB,
};
use crate::monitors::rancher_channel_server::{
    RancherChannelServerConfiguration, RancherChannelServerConfigurationInner,
    TYPE_NAME_RANCHER_CHANNEL,
};
use crate::ui::handlers::index::get_index;
use crate::ui::handlers::{
    ADD_RECORD_TITLE, AppState, common_form_values, github_form_values, rancher_channel_form_values,
};
use crate::ui::pages::add_page::{add_github_monitor_page, add_rancher_channel_page};
use axum::Form;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use maud::Markup;
use std::collections::HashMap;
use tracing::debug;

/// Display the Add Monitor page
pub async fn get_add_monitor(
    state: State<AppState>,
    Path(monitor_type): Path<String>,
) -> Result<Markup, StatusCode> {
    debug!("Display Add monitor record");

    match monitor_type.as_str() {
        TYPE_NAME_GITHUB => Ok(add_github_monitor_page(
            format!("{} - {}", ADD_RECORD_TITLE, "Github").as_str(),
            state.css_path(),
        )
        .await),
        TYPE_NAME_RANCHER_CHANNEL => Ok(add_rancher_channel_page(
            format!("{} - {}", ADD_RECORD_TITLE, "Rancher-Channel").as_str(),
            state.css_path(),
        )
        .await),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn post_add_monitor_record(
    state: State<AppState>,
    Path(monitor_type): Path<String>,
    Form(form): Form<HashMap<String, String>>,
) -> Result<Markup, StatusCode> {
    let monitor = match monitor_type.as_str() {
        TYPE_NAME_GITHUB => Ok(post_add_github_monitor(form).await),
        TYPE_NAME_RANCHER_CHANNEL => Ok(post_add_rancher_channel(form).await),
        _ => Err(StatusCode::NOT_FOUND),
    }?;

    add_monitor(&state.db, monitor)
        .await
        .expect("unable to insert");
    get_index(state, None).await
}

async fn post_add_github_monitor(form: HashMap<String, String>) -> Box<dyn Monitor> {
    debug!("Submit Add Github monitor record");
    debug!("Form: {:?}", form);
    let (owner, repo, github_personal_token) = github_form_values(&form);
    let (name, notification, frequency, period) = common_form_values(&form);

    Box::new(GithubConfiguration {
        name,
        inner: GithubConfigurationInner {
            owner,
            repo,
            notification,
            frequency,
            period,
            github_personal_token,
        },
    })
}

async fn post_add_rancher_channel(form: HashMap<String, String>) -> Box<dyn Monitor> {
    debug!("Submit Add Rancher Channel monitor record");
    debug!("{:?}", form);
    let (url, channel) = rancher_channel_form_values(&form);
    let (name, notification, frequency, period) = common_form_values(&form);

    Box::new(RancherChannelServerConfiguration {
        name,
        inner: RancherChannelServerConfigurationInner {
            url,
            channel,
            notification,
            frequency,
            period,
        },
    })
}
