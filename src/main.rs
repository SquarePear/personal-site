use crate::templating::pages::route;
use ascii::AsciiString;
use maud::Markup;
use std::{fs::File, io::Cursor, path::Path, sync::Arc, thread};
use tiny_http::{Header, Response, Server};

mod templating;

fn main() {
    println!("Starting server!");

    let server = Arc::new(Server::http("127.0.0.1:3000").unwrap());

    let mut handles = Vec::new();

    for thread_num in 0..std::thread::available_parallelism().unwrap().into() {
        println!("Starting thread: {}", thread_num);

        let server = server.clone();
        handles.push(thread::spawn(move || server_thread(server)));
    }

    for handle in handles {
        handle.join().unwrap()
    }
}

fn server_thread(server: Arc<Server>) {
    for request in server.incoming_requests() {
        println!(
            "Incoming request! method: {:?}, url: {:?}",
            request.method(),
            request.url()
        );

        if let Some(markup) = route(&request) {
            let response = markup_response(markup);

            let _ = request.respond(response);
        } else if let Some(resource) = find_resource(request.url()) {
            let _ = request.respond(resource);
        } else {
            let response = markup_response(templating::pages::not_found::render());

            let _ = request.respond(response);
        }
    }
}

fn find_resource(url: &str) -> Option<Response<File>> {
    let file = File::open(&Path::new(&format!("public{}", url))).ok();

    let resource = Response::from_file(file?);

    Some(resource)
}

fn markup_response(markup: Markup) -> Response<Cursor<Vec<u8>>> {
    let response = Response::from_string(markup.into_string());

    let response = response.with_header(Header {
        field: "Content-Type".parse().unwrap(),
        value: AsciiString::from_ascii("text/html; charset=utf8;").unwrap(),
    });

    response
}
