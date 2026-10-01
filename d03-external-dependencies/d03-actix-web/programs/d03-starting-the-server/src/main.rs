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
    #[allow(unused_variables)]

    let actix_web_app = App::new()
        .service(root_get)
        .service(trending_get);
}
