/*
    ABOUT: Working with JSON
*/

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};

//_____________________________________________________________________________

// EXAMPLE: 1 => Creating a Post request that sends JSON

#[derive(Deserialize, Serialize)]
struct Task {
    id: u32,
    title: String,
    completed: bool
}

// Serialize = Rust data → JSON
// Deserialize = JSON → Rust data
#[post("/tasks")]
async fn create_task(task: web::Json<Task>) -> impl Responder {
    HttpResponse::Ok().json(task.into_inner())
}


//_____________________________________________________________________________

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

#[derive(Deserialize)]
struct QueryParams {
    status: Option<String>,
    limit: Option<usize>
}

#[get("/tasks")]
async fn list_tasks(query: web::Query<QueryParams>) -> impl Responder {

    let status = query.status.as_deref().unwrap_or("all");
    let limit = query.limit.unwrap_or(10);

    let message = format!("Fetching {limit} tasks with status: {status}");

    HttpResponse::Ok().body(message)
}

//_____________________________________________________________________________
//_____________________________________________________________________________

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

    println!("Starting Actix Web Server at http://{ip_address}:{port}");

    match actix_web_server.await {
        Ok(()) => {}
        Err(error) => eprintln!("Actix Web Server failed: {error}")
    }
}
