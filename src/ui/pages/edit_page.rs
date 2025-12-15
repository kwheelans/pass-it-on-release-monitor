use crate::monitors::github_release::{GithubConfiguration, TYPE_NAME_GITHUB};
use crate::monitors::rancher_channel_server::{
    RancherChannelServerConfiguration, TYPE_NAME_RANCHER_CHANNEL,
};
use crate::monitors::{FrequencyPeriod, FrequencyValue};
use crate::ui::pages::{base, title};
use maud::{Markup, html};

pub async fn edit_github_monitor_page(
    page_title: &str,
    css_path: &str,
    monitor: GithubConfiguration,
) -> Markup {
    html! {
        (base(css_path).await)
        body {
            (title(page_title).await)
            main {
                form method="post" {
                    label for="owner" { "Owner" }
                    input type="text" id="owner" name="owner" placeholder="Enter Owner Name"  value=(monitor.inner.owner) autofocus minlength="1" required;
                    label for="repo" { "Repository" }
                    input type="text" id="repo" name="repo" placeholder="Enter Repository Name"  value=(monitor.inner.repo) minlength="1" required;
                    (common(monitor.name.as_str(), monitor.inner.notification.as_str(), monitor.inner.period, monitor.inner.frequency).await)
                    label for="token" { "Github Personal Token" }
                    input type="password" id="token" name="token" placeholder="Enter Github Personal Token" value=(monitor.inner.github_personal_token.unwrap_or_default()) ;
                    div {
                        input type="hidden" id="monitor_type" name="monitor_type" value=(TYPE_NAME_GITHUB);
                        input type="submit" value="Save";
                        a href="/" {
                            input type="button" value="Cancel";
                        }
                    }
                }
            }
        }
    }
}

pub async fn edit_rancher_channel_monitor_page(
    page_title: &str,
    css_path: &str,
    monitor: RancherChannelServerConfiguration,
) -> Markup {
    html! {
        (base(css_path).await)
        body {
            (title(page_title).await)
            main {
                form method="post" {
                    label for="url" { "URL" }
                    input type="text" id="url" name="url" placeholder="Enter Rancher Channel URL"  value=(monitor.inner.url) autofocus minlength="1" required;
                    label for="channel" { "Channel" }
                    input type="text" id="channel" name="channel" placeholder="Enter Channel Name"  value=(monitor.inner.channel) minlength="1" required;
                    (common(monitor.name.as_str(), monitor.inner.notification.as_str(), monitor.inner.period, monitor.inner.frequency).await)
                    div {
                        input type="hidden" id="monitor_type" name="monitor_type" value=(TYPE_NAME_RANCHER_CHANNEL);
                        input type="submit" value="Save";
                        a href="/" {
                            input type="button" value="Cancel";
                        }
                    }
                }
            }
        }
    }
}

async fn common<S: AsRef<str>>(
    name: S,
    notification: S,
    period: FrequencyPeriod,
    frequency: FrequencyValue,
) -> Markup {
    html! {
        label for="name" { "Monitor Name" }
        input type="text" id="name" name="name" placeholder="Enter Monitor Name"  value={(name.as_ref())} minlength="1" required;

        label for="notification" { "Notification Group" }
        input type="text" id="notification" name="notification" placeholder="Enter Notification Group"  value={(notification.as_ref())} minlength="1" required;

        label for="period" { "Frequency Period" }
        select id="period" name="period" {
            @if period.eq(&FrequencyPeriod::Minute) {
                option value="minute" selected {"Minute"}
                option value="hour" {"Hour"}
                option value="day" {"Day"}
                option value="week" {"Week"}
            } @else if period.eq(&FrequencyPeriod::Day) {
                option value="minute" {"Minute"}
                option value="hour" {"Hour"}
                option value="day" selected {"Day"}
                option value="week" {"Week"}
            } @else if period.eq(&FrequencyPeriod::Week) {
                option value="minute" {"Minute"}
                option value="hour" {"Hour"}
                option value="day" selected {"Day"}
                option value="week" {"Week"}
            } @else {
                option value="minute" {"Minute"}
                option value="hour" selected {"Hour"}
                option value="day" {"Day"}
                option value="week" {"Week"}
            }
        }

        label for="frequency" { "Frequency Value" }
        input type="number" id="frequency" name="frequency" placeholder="Enter value for selected frequency period"  value=(frequency.inner()) minlength="1" required;
    }
}
