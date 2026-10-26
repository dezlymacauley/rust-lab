/*
    ABOUT: Routes with multiple parameters
*/

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

//_____________________________________________________________________________

// EXAMPLE: 1 => Accessing path parameters by index

/*

#[get("/tasks/{id}/{status}")]
async fn get_task(
    path_parameters: web::Path<(u32, String)>
    ) -> impl Responder {
    let message = format!(
        "Task ID: {}. Status: {}", 
        path_parameters.0, path_parameters.1
    );
    HttpResponse::Ok().body(message)
}

*/

//_____________________________________________________________________________

// EXAMPLE: 2 => Accessing path parameters by destructuring the tuple

/*

#[get("/tasks/{id}/{status}")]
async fn get_task(path_parameters: web::Path<(u32, String)>) -> impl Responder {

    let (id, status) = path_parameters.into_inner();

    let message = format!("Task ID: {id}. Status: {status}");

    HttpResponse::Ok().body(message)
}

*/

//_____________________________________________________________________________

// EXAMPLE: 3 => Accessing path parameters with help from `serde`

#[derive(Deserialize)]
struct Params {
    id: u32,
    status: String
}

#[get("/tasks/{id}/{status}")]
async fn get_task(path_parameters: web::Path<Params>) -> impl Responder {

    let message = format!(
        "Task ID: {}, Status: {}", 
        path_parameters.id, path_parameters.status
    );

    HttpResponse::Ok().body(message)
}

//_____________________________________________________________________________

// #[get("/tasks/{id}/{status}")]
// async fn get_tasks(path: web::Path<(u32, String)>) -> impl Responder {
//     let (id, status) = path.into_inner();
//     HttpResponse::Ok().body(format!("Task ID: {id}. Status: {status}"))
// }

#[actix_web::main]
async fn main() {

    let actix_web_app = || {
    App::new()
        .service(get_task)
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
