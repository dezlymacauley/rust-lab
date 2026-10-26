/*
    ABOUT: Creating Get Routes that respond with text

    My naming preference for route handler functions:

    I prefer the syntax: name_of_route request_type.
    This makes it easier to structure API tests with hurl,
    which is a Rust-powered API testing tool.
*/

// The `get` macro allows you to register an async function,
// as route handler for a specific route.

// `HttpResponse` is a struct that allows you to create an HttpResponse
// that will be sent to the client, when a request is made to a specific
// route.

// `Responder` is a trait that allows the return type
// of a function to be used as an HTTP response.
use actix_web::{App, HttpResponse, Responder, get};

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
