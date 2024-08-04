use maud::{html, Markup};

use crate::templating::layout;

pub fn render() -> Markup {
    let markup = html! {
        h1 { "404 NOT FOUND" }
    };

    layout::render(markup)
}
