use crate::monitors::github_release::TYPE_NAME_GITHUB;
use crate::monitors::rancher_channel_server::TYPE_NAME_RANCHER_CHANNEL;
use crate::ui::handlers::ADD_RECORD_TITLE;
use crate::ui::pages::add_page::{add_github_monitor_page, add_rancher_channel_page};
use axum::extract::Path;
use axum::http::StatusCode;
use maud::Markup;
use tracing::debug;

/// Display the Add Monitor page
pub async fn get_ui_add_monitor(Path(monitor_type): Path<String>) -> Result<Markup, StatusCode> {
    debug!("Display Add monitor record");

    match monitor_type.as_str() {
        TYPE_NAME_GITHUB => Ok(add_github_monitor_page(
            format!("{} - {}", ADD_RECORD_TITLE, "Github").as_str(),
        )
        .await),
        TYPE_NAME_RANCHER_CHANNEL => Ok(add_rancher_channel_page(
            format!("{} - {}", ADD_RECORD_TITLE, "Rancher-Channel").as_str(),
        )
        .await),
        _ => Err(StatusCode::NOT_FOUND),
    }
}
