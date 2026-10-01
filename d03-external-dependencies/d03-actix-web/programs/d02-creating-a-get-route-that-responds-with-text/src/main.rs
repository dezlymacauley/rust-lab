// The `get` macro allows you to turn an async function,
// into a route handler.
use actix_web::{App, HttpResponse, Responder, get};

#[get("/")]
async fn root_get() -> impl Responder {
    HttpResponse::Ok().body("This is the / route")
}

#[get("/trending")]
async fn trending_get() -> impl Responder {
    HttpResponse::Ok().body("This is the /trending route")
}

fn main() {

    // To add a route handler, 
    // just use `.service(name_of_route_handler)`

    #[allow(unused_variables)]
    let actix_web_app = App::new()
        .service(root_get)
        .service(trending_get);
}
