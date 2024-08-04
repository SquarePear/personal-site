use maud::{html, Markup, DOCTYPE};

use crate::templating::components::header;

pub fn render(markup: Markup) -> Markup {
    let markup = html! {
        (DOCTYPE)
        html lang="en" {
            (head())
            body {
                (header::render())
                (markup)
            }
        }
    };

    markup
}

fn head() -> Markup {
    html! {
        head {
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { "Jeffrey Harmon" }
        }
    }
}
