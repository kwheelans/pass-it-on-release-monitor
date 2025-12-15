use crate::monitors::{FrequencyPeriod, FrequencyValue};
use crate::ui::handlers::add::{get_add_monitor, post_add_monitor_record};
use crate::ui::handlers::edit::{get_edit_monitor, post_edit_monitor_record};
use crate::ui::handlers::index::{delete_monitor_record, get_index};
use axum::routing::get;
use axum::{Router, serve};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub mod add;
pub mod edit;
pub mod index;

const INDEX_PAGE_TITLE: &str = "Release Monitor";
const ADD_RECORD_TITLE: &str = "Add Monitor Record";

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    stylesheet_href: String,
    local_css_path: Option<PathBuf>,
}

impl AppState {
    pub fn new(
        db: DatabaseConnection,
        stylesheet_href: String,
        local_css_path: Option<PathBuf>,
    ) -> Self {
        Self {
            db,
            stylesheet_href,
            local_css_path,
        }
    }
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn stylesheet_href(&self) -> &str {
        &self.stylesheet_href
    }

    pub fn local_css_path(&self) -> &Option<PathBuf> {
        &self.local_css_path
    }
}

pub async fn serve_web_ui(state: AppState, listener: TcpListener) {
    let root_route = match state.local_css_path() {
        None => Router::new().route("/", get(get_index)),
        Some(p) => Router::new()
            .route("/", get(get_index))
            .nest_service("/css", ServeDir::new(p)),
    };
    let other_routes = Router::new()
        .route("/{id}", get(get_index).post(delete_monitor_record))
        .route(
            "/add/{monitor_type}",
            get(get_add_monitor).post(post_add_monitor_record),
        )
        .route(
            "/edit/{id}",
            get(get_edit_monitor).post(post_edit_monitor_record),
        );
    let routes = Router::new()
        .merge(root_route)
        .merge(other_routes)
        .with_state(state);

    serve(listener, routes).await.expect("axum serve error")
}

fn common_form_values(
    form: &HashMap<String, String>,
) -> (String, String, FrequencyValue, FrequencyPeriod) {
    let name = form.get("name").expect("unable to retrieve name");
    let notification = form
        .get("notification")
        .expect("unable to retrieve notification");
    let frequency = FrequencyValue::try_from(
        form.get("frequency")
            .expect("unable to retrieve frequency")
            .as_ref(),
    )
    .unwrap_or_default();
    let period = FrequencyPeriod::try_from(
        form.get("period")
            .expect("unable to retrieve period")
            .as_ref(),
    )
    .unwrap_or_default();
    (name.into(), notification.into(), frequency, period)
}

fn github_form_values(form: &HashMap<String, String>) -> (String, String, Option<String>) {
    let owner = form.get("owner").expect("unable to retrieve owner").into();
    let repo = form.get("repo").expect("unable to retrieve repo").into();
    let token = form
        .get("token")
        .expect("unable to retrieve token")
        .to_string();
    let token = match token.is_empty() {
        true => None,
        false => Some(token),
    };
    (owner, repo, token)
}

fn rancher_channel_form_values(form: &HashMap<String, String>) -> (String, String) {
    let url = form.get("url").expect("unable to retrieve url").into();
    let channel = form
        .get("channel")
        .expect("unable to retrieve channel")
        .into();
    (url, channel)
}
