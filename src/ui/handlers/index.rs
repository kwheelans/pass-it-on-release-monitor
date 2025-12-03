use crate::database::queries::delete_monitor;
use crate::ui::handlers::{AppState, INDEX_PAGE_TITLE};
use crate::ui::pages::index_page::index_page;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use maud::Markup;
use tracing::debug;

/// Display the Index Page
pub async fn get_index(
    state: State<AppState>,
    id: Option<Path<i64>>,
) -> Result<Markup, StatusCode> {
    if let Some(Path(selected)) = id {
        Ok(index_page(state, INDEX_PAGE_TITLE, Some(selected)).await)
    } else {
        Ok(index_page(state, INDEX_PAGE_TITLE, None).await)
    }
}

pub async fn delete_monitor_record(
    state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<Markup, StatusCode> {
    debug!("Delete monitor record id: {}", id);
    delete_monitor(&state.db, id)
        .await
        .expect("unable to delete record");
    get_index(state, None).await
}
