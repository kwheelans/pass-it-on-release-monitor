use crate::webpage::AppState;
use crate::webpage::index_page::index_page;
use axum::extract::State;
use axum::http::StatusCode;
use maud::Markup;

const INDEX_PAGE_TITLE: &str = "Release Monitor";

pub async fn get_ui_index(state: State<AppState>) -> Result<Markup, StatusCode> {
    Ok(index_page(state, INDEX_PAGE_TITLE).await)
}
