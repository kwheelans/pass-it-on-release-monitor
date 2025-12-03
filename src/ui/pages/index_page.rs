use crate::AppState;
use crate::database::queries::select_all_monitors;
use crate::monitors::github_release::TYPE_NAME_GITHUB;
use crate::monitors::rancher_channel_server::TYPE_NAME_RANCHER_CHANNEL;
use crate::ui::pages::{base, title};
use axum::extract::State;
use maud::{Markup, html};
use tracing::log::debug;

pub async fn index_page(state: State<AppState>, page_title: &str) -> Markup {
    html! {
        (base().await)
        body {
            (title(page_title).await)
            main {
                (list_records(state).await)
            }
        }
    }
}

async fn list_records(state: State<AppState>) -> Markup {
    let db = state.db();
    let id = state.id();
    let records = select_all_monitors(db).await.expect("unable to select");
    let has_records = !records.is_empty();
    debug!("Index: {:?}", id);
    debug!("Has_records: {}", has_records);

    html! {
        section {
            table {
                tbody {
                    tr {
                        td width="50%" {
                            a href={ "/add/" (TYPE_NAME_GITHUB) } {
                                input type="button" value="Add Github";
                            }
                        }
                        td width="50%" {
                            a href={ "/add/" (TYPE_NAME_RANCHER_CHANNEL) } {
                                input type="button" value="Add Rancher Channel";
                            }
                        }
                    }
                }
            }

            h2 { "Database Records" }
            @if has_records {
                table {
                    thead {
                        tr {
                            th {"ID"}
                            th {"Name"}
                            th {"Type"}
                            th {"Configuration"}
                            th {"Version"}
                            th {"Last Checked"}
                        }
                        @for record in records {
                            tr onclick={ "window.location='/" (record.id) "';" } {
                                td { (record.id) }
                                td { (record.name) }
                                td { (record.monitor_type) }
                                td { (record.configuration) }
                                td { (record.version) }
                                td { (record.timestamp.0) }
                            }
                        }
                    }
                }
            } @else {
                "No database records"
            }
            @if let Some(selected_id) = id {
                section {
                    form action={ "/edit/" (selected_id) } method="get" {
                        header {
                            h3 { "Record ID " (selected_id) " selected" }
                            input type="Submit" value="Edit";
                            input type="Submit" value="Delete" formmethod="post" formaction={ "/" (selected_id)  };
                        }
                    }
                }
            }
        }
    }
}
