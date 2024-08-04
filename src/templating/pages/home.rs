use maud::{html, Markup};

use crate::templating::layout;

pub fn render() -> Markup {
    let name = "World";
    let markup = html! {
        h1 { "Hello, " (name) "!" }
        p { "Welcome to my website!" }
    };

    layout::render(markup)
}
