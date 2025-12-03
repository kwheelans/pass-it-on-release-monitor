use crate::monitors::github_release::TYPE_NAME_GITHUB;
use crate::monitors::rancher_channel_server::TYPE_NAME_RANCHER_CHANNEL;
use crate::ui::pages::{base, title};
use maud::{Markup, html};

pub async fn add_github_monitor_page(page_title: &str) -> Markup {
    html! {
        (base().await)
        body {
            (title(page_title).await)
            main {
                section {
                    form action={ "/add/" (TYPE_NAME_GITHUB) } method="post" {
                        div {
                            label for="owner" { "Owner" }
                            input type="text" id="owner" name="owner" placeholder="Enter Owner Name"  value="" autofocus minlength="1" required;

                            label for="repo" { "Repository" }
                            input type="text" id="repo" name="repo" placeholder="Enter Repository Name"  value="" minlength="1" required;

                            (common().await)

                            label for="token" { "Github Personal Token" }
                            input type="password" id="token" name="token" placeholder="Enter Github Personal Token" value="" ;
                        }
                        div {
                            input type="submit" value="Add";
                            a href="/" {
                                input type="button" value="Cancel";
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn add_rancher_channel_page(page_title: &str) -> Markup {
    html! {
        (base().await)
        body {
            (title(page_title).await)
            main {
                section {
                    form action={ "/add/" (TYPE_NAME_RANCHER_CHANNEL) } method="post" {
                        div {
                            label for="url" { "URL" }
                            input type="text" id="url" name="url" placeholder="Enter Rancher Channel URL"  value="" autofocus minlength="1" required;

                            label for="channel" { "Channel" }
                            input type="text" id="channel" name="channel" placeholder="Enter Channel Name"  value="" minlength="1" required;

                            (common().await)
                        }
                        div {
                            input type="submit" value="Add";
                            a href="/" {
                                input type="button" value="Cancel";
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn common() -> Markup {
    html! {
        label for="name" { "Monitor Name" }
        input type="text" id="name" name="name" placeholder="Enter Monitor Name"  value="" minlength="1" required;

        label for="notification" { "Notification Group" }
        input type="text" id="notification" name="notification" placeholder="Enter Notification Group"  value="" minlength="1" required;

        label for="period" { "Frequency Period" }
        select id="period" name="period" {
            option value="minute" {"Minute"}
            option value="hour" selected {"Hour"}
            option value="day" {"Day"}
            option value="week" {"Week"}
        }

        label for="frequency" { "Frequency Value" }
        input type="number" id="frequency" name="frequency" placeholder="Enter value for selected frequency period"  value="1" minlength="1" required;
    }
}
