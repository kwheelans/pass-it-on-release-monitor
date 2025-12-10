use crate::database::MonitorModel;
use crate::monitors::github_release::TYPE_NAME_GITHUB;
use crate::monitors::rancher_channel_server::TYPE_NAME_RANCHER_CHANNEL;
use crate::ui::pages::{base, title};
use maud::{Markup, html};
use sea_orm::prelude::ChronoUtc;
use tracing::trace;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S %Z";

pub async fn index_page(
    page_title: &str,
    css_path: &str,
    records: Vec<MonitorModel>,
    id: Option<i64>,
) -> Markup {
    html! {
        (base(css_path).await)
        body {
            (title(page_title).await)
            main {
                (list_records(records, id).await)
            }
        }
    }
}

async fn list_records(records: Vec<MonitorModel>, id: Option<i64>) -> Markup {
    let now = ChronoUtc::now().format(DATE_FORMAT);
    let has_records = !records.is_empty();
    trace!("Index: {:?}", id);
    trace!("Has_records: {}", has_records);

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
            { "Current Time: " (now)}
            h2 { "Database Records" }
            @if has_records {
                table {
                    thead {
                        tr {
                            th {"ID"}
                            th {"Name"}
                            th {"Type"}
                            th {"Version"}
                            th {"Last Checked"}
                        }
                        @for record in records {
                            tr onclick={ "window.location='/" (record.id) "';" } {
                                td { (record.id) }
                                td { (record.name) }
                                td { (record.monitor_type) }
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
