use axum::extract::{Path, State};
use axum::http::StatusCode;
use maud::Markup;
use sea_orm::{DatabaseConnection};
use tracing::error;
use crate::database::queries::select_one_monitor;
use crate::monitors::github_release::{GithubConfiguration, TYPE_NAME_GITHUB};
use crate::monitors::rancher_channel_server::{RancherChannelServerConfiguration, TYPE_NAME_RANCHER_CHANNEL};
use crate::webpage::edit_page::{edit_github_monitor_page, edit_rancher_channel_monitor_page};
use crate::webpage::handlers::{AppState, ADD_RECORD_TITLE};

pub async fn get_ui_edit_monitor(
    state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    match select_one_monitor(state.db(), id).await {
        Ok(Some(model)) => {
            match model.monitor_type.as_str() {
                TYPE_NAME_GITHUB => Ok(edit_github_monitor(id, &state.db).await?),
                TYPE_NAME_RANCHER_CHANNEL => Ok(edit_rancher_channel_monitor(id, &state.db).await?),
                _ => Err(StatusCode::NOT_FOUND),
            }
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

async fn edit_github_monitor(id: i64, db: &DatabaseConnection) -> Result<Markup, StatusCode> {
    match select_one_monitor(db, id).await.expect("unable to select") {
        None => Err(StatusCode::NO_CONTENT),
        Some(model) => {
            match serde_json::from_str::<GithubConfiguration>(model.configuration.as_str()) {
                Ok(monitor) => Ok(edit_github_monitor_page(ADD_RECORD_TITLE, monitor).await),
                Err(e) => {
                    error!("Unbale to parse JSON: {}", e);
                    Err(StatusCode::NOT_FOUND)
                }
            }
        }
    }
}

async fn edit_rancher_channel_monitor(
    id: i64,
    db: &DatabaseConnection,
) -> Result<Markup, StatusCode> {
    match select_one_monitor(db, id).await.expect("unable to select") {
        None => Err(StatusCode::NO_CONTENT),
        Some(model) => {
            match serde_json::from_str::<RancherChannelServerConfiguration>(model.configuration.as_str()) {
                Ok(monitor) => Ok(edit_rancher_channel_monitor_page(ADD_RECORD_TITLE, monitor).await),
                Err(e) => {
                    error!("Unbale to parse JSON: {}", e);
                    Err(StatusCode::NOT_FOUND)
                }
            }
        }
    }
}