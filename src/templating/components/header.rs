use maud::{html, Markup};

pub fn render() -> Markup {
    html! {
        header {
            nav {
                ul {
                    li { a href="/" { "~/" } }
                    li { a href="/about" { "~/about" } }
                    li { a href="/projects" { "~/projects" } }
                    li { a href="/blog" { "~/blog" } }
                }
            }
        }
    }
}
