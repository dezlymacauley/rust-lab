use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web")
}

#[actix_web::main]
async fn main() {

    let actix_web_app = || {
    App::new()
        .service(hello)
    };

    let ip_address: &str = "127.0.0.1";
    let port: u16 = 8080;

    let server_address = (ip_address, port);

    let actix_web_server = HttpServer::new(actix_web_app)
        .bind(server_address)
        .expect("Actix Web Server could not bind to the server address")
        .run();

    // `.run()` is an async function. So the Actix Web Server won't start
    // until you use `.await`.

    // `.await` can only be used inside an `async` function,
    // so you have to change `fn main`, to `async fn main`

    println!("Starting Actix Web Server at http://{ip_address}:{port}");

    match actix_web_server.await {
        Ok(()) => {}
        Err(error) => eprintln!("Actix Web Server failed: {error}")
    }
}
