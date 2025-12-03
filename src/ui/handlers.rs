use crate::ui::handlers::add::get_ui_add_monitor;
use crate::ui::handlers::edit::get_ui_edit_monitor;
use crate::ui::handlers::index::{delete_monitor_record, get_ui_index, get_ui_index_select_id};
use axum::routing::get;
use axum::{serve, Router};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

pub mod add;
pub mod edit;
pub mod index;

const INDEX_PAGE_TITLE: &str = "Release Monitor";
const ADD_RECORD_TITLE: &str = "Add Monitor Record";

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    id: Option<i64>,
    monitor_type: Option<String>,
}

impl AppState {
    pub fn new(db: DatabaseConnection, id: Option<i64>, monitor_type: Option<String>) -> Self {
        Self {
            db,
            id,
            monitor_type,
        }
    }
    pub fn set_id(&mut self, id: Option<i64>) {
        self.id = id;
    }
    pub fn set_monitor_type(&mut self, monitor_type: Option<String>) {
        self.monitor_type = monitor_type;
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn id(&self) -> Option<i64> {
        self.id
    }

    pub fn monitor_type(&self) -> &Option<String> {
        &self.monitor_type
    }
}

pub async fn serve_web_ui(state: AppState, listener: TcpListener) {
    let routes = Router::new()
        .route("/", get(get_ui_index))
        .route(
            "/{id}",
            get(get_ui_index_select_id).post(delete_monitor_record),
        )
        .route("/add/{monitor_type}", get(get_ui_add_monitor))
        .route("/edit/{id}", get(get_ui_edit_monitor))
        .with_state(state);

    serve(listener, routes).await.expect("axum serve error")
}
