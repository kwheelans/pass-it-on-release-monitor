use crate::database::queries::delete_monitor;
use crate::ui::handlers::{AppState, INDEX_PAGE_TITLE};
use crate::ui::pages::index_page::index_page;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use maud::Markup;
use tracing::debug;

/// Display the Index Page
pub async fn get_ui_index(state: State<AppState>) -> Result<Markup, StatusCode> {
    Ok(index_page(state, INDEX_PAGE_TITLE).await)
}

pub async fn get_ui_index_select_id(
    mut state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    debug!("Selected record id: {}", id);
    state.set_id(Some(id));
    get_ui_index(state).await
}

pub async fn delete_monitor_record(
    state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    debug!("Delete monitor record id: {}", id);
    delete_monitor(&state.db, id)
        .await
        .expect("unable to delete record");
    get_ui_index(state).await
}
