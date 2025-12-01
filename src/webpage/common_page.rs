use maud::{DOCTYPE, Markup, html};

pub(super) async fn base() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                meta name="color-scheme" content="light dark";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.classless.indigo.min.css";
            }
        }
    }
}

pub(super) async fn title(title: &str) -> Markup {
    html! {
        header {
            h1 { (title) };
            hr;
        }
    }
}
