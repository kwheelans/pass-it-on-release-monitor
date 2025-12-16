use crate::database::queries::{delete_monitor, select_all_monitors};
use crate::ui::handlers::{AppState, INDEX_PAGE_TITLE, UI_ROOT};
use crate::ui::pages::index_page::index_page;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use maud::Markup;
use tracing::{debug, error};

/// Display the Index Page
pub async fn get_index(
    state: State<AppState>,
    id: Option<Path<i64>>,
) -> Result<Markup, StatusCode> {
    let records = match select_all_monitors(state.db()).await {
        Ok(r) => Ok(r),
        Err(e) => {
            error!("Select all failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }?;
    if let Some(Path(selected)) = id {
        Ok(index_page(
            INDEX_PAGE_TITLE,
            state.stylesheet_href(),
            records,
            Some(selected),
        )
        .await)
    } else {
        Ok(index_page(INDEX_PAGE_TITLE, state.stylesheet_href(), records, None).await)
    }
}

pub async fn delete_monitor_record(
    state: State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    debug!("Delete monitor record id: {}", id);
    delete_monitor(&state.db, id)
        .await
        .expect("unable to delete record");
    Ok(Redirect::to(UI_ROOT))
}
