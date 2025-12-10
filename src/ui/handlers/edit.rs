use crate::database::MonitorModel;
use crate::database::queries::{select_one_monitor, update_monitor};
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
use crate::ui::pages::edit_page::{edit_github_monitor_page, edit_rancher_channel_monitor_page};
use axum::Form;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use maud::Markup;
use sea_orm::{IntoActiveModel, Set};
use std::collections::HashMap;
use tracing::{debug, error};

pub async fn get_edit_monitor(
    state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    match select_one_monitor(state.db(), id).await {
        Ok(Some(model)) => match model.monitor_type.as_str() {
            TYPE_NAME_GITHUB => Ok(edit_github_monitor(state, model).await?),
            TYPE_NAME_RANCHER_CHANNEL => Ok(edit_rancher_channel_monitor(state, model).await?),
            _ => Err(StatusCode::NOT_FOUND),
        },
        Ok(None) => {
            error!("Database Select by ID returned nothing");
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Database Select by ID failed: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn edit_github_monitor(
    state: State<AppState>,
    model: MonitorModel,
) -> Result<Markup, StatusCode> {
    match serde_json::from_str::<GithubConfigurationInner>(model.configuration.as_str()) {
        Ok(inner) => {
            let monitor = GithubConfiguration {
                name: model.name.clone(),
                inner,
            };
            Ok(edit_github_monitor_page(ADD_RECORD_TITLE, state.css_path(), monitor).await)
        }
        Err(e) => {
            error!("Unable to parse JSON: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn edit_rancher_channel_monitor(
    state: State<AppState>,
    model: MonitorModel,
) -> Result<Markup, StatusCode> {
    match serde_json::from_str::<RancherChannelServerConfigurationInner>(
        model.configuration.as_str(),
    ) {
        Ok(inner) => {
            let monitor = RancherChannelServerConfiguration {
                name: model.name.clone(),
                inner,
            };
            Ok(
                edit_rancher_channel_monitor_page(ADD_RECORD_TITLE, state.css_path(), monitor)
                    .await,
            )
        }
        Err(e) => {
            error!("Unbale to parse JSON: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn post_edit_monitor_record(
    state: State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<HashMap<String, String>>,
) -> Result<Markup, StatusCode> {
    match select_one_monitor(state.db(), id).await {
        Ok(Some(model)) => {
            let monitor = match model.monitor_type.as_str() {
                TYPE_NAME_GITHUB => Ok(submit_edit_github_monitor(form).await),
                TYPE_NAME_RANCHER_CHANNEL => Ok(submit_edit_rancher_channel_monitor(form).await),
                _ => Err(StatusCode::NOT_FOUND),
            }?;

            let mut active_model = model.into_active_model();
            active_model.name = Set(monitor.name());
            active_model.configuration = Set(monitor.inner_to_json());

            match update_monitor(state.db(), active_model).await {
                Ok(_) => Ok(get_index(state, None).await?),
                Err(e) => {
                    debug!("{}", e);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Ok(None) => {
            error!("Database Select by ID returned nothing");
            Err(StatusCode::NOT_FOUND)
        }
        Err(e) => {
            error!("Database Select by ID failed: {}", e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

async fn submit_edit_github_monitor(form: HashMap<String, String>) -> Box<dyn Monitor> {
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

async fn submit_edit_rancher_channel_monitor(form: HashMap<String, String>) -> Box<dyn Monitor> {
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
