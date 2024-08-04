use maud::Markup;
use tiny_http::Request;

mod home;
pub mod not_found;

pub fn route(req: &Request) -> Option<Markup> {
    match req.url() {
        "" | "/" => Some(home::render()),
        // "/about" => Some(about::render()),
        // "/projects" => Some(projects::render()),
        // "/blog" => Some(blog::render()),
        _ => None,
    }
}
