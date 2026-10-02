// The `get` macro allows you to turn an async function,
// into a route handler.
use actix_web::{App, HttpResponse, Responder, get};

// NOTE: My naming preference for route handler functions
// ------------------------------------------------------
// I prefer the syntax: name_of_route request_type.
// This makes it easier to structure API tests with hurl,
// which is a Rust-powered API testing tool.

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("Route: /")
}

#[get("/content-creators")]
async fn content_creators_get() -> impl Responder {
    HttpResponse::Ok().body("Route: /content-creators")
}

fn main() {

    // To add a route handler, 
    // just use `.service(name_of_route_handler)`
    #[allow(unused_variables)]
    let actix_web_app = App::new()
        .service(root_get)
        .service(content_creators_get);
}
