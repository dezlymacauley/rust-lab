/*
    ABOUT: Creating a server to create the API - Part 1

*/

// `HttpServer` is a struct that allows you to create an instance
// of an Actix Web Server that can be used to serve an instance of `Server`
use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("Route: /")
}

#[get("/content-creators")]
async fn content_creators_get() -> impl Responder {
    HttpResponse::Ok().body("Route: /content-creators")
}

fn main() {

    // The `HttpServer::new()` method accepts a closure that creates an
    // Actix Web App, so `actix_web_app` must be a closure that returns an `App`.
    // A closure is an inline function.
    let actix_web_app = || {
    App::new()
        .service(root_get)
        .service(content_creators_get)
    };

    let ip_address: &str = "127.0.0.1";
    let port: u16 = 8080;

    // The `.bind()` method of the `HttpServer` struct,
    // accepts the server address a tupple
    let server_address = (ip_address, port);

    #[allow(unused_variables)]
    let actix_web_server = HttpServer::new(actix_web_app)
        .bind(server_address)
        .expect("Actix Web Server could not bind to the server address")
        .run();
}
