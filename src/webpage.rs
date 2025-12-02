mod common_page;
mod handlers;
mod index_page;

use crate::webpage::handlers::get_ui_index;
use axum::routing::get;
use axum::{Router, serve};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub(super) struct AppState {
    db: DatabaseConnection,
    id: Option<i64>,
    monitor_type: Option<String>,
    //TODO: HashSet of existing name strings to check against and return message or confirmation dialog
}

impl AppState {
    pub fn new(db: DatabaseConnection, id: Option<i64>, monitor_type: Option<String>) -> Self {
        Self {
            db,
            id,
            monitor_type,
        }
    }
    pub fn set_index(&mut self, id: Option<i64>) {
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
        .with_state(state);

    serve(listener, routes).await.expect("axum serve error")
}
