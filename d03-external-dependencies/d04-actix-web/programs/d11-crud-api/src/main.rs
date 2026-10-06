/*
    ABOUT: CRUD API
*/

use std::{collections::HashMap, sync::Mutex};

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::{Deserialize, Serialize};

//_____________________________________________________________________________

// SECTION: Data Structures 

// Serialize = Rust data → JSON
// Deserialize = JSON → Rust data
#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool
}

// The Mutex is to ensure that only one part of the program,
// to be specific, one thread can access this in-memory database.
type TaskStore = Mutex<HashMap<u32, Task>>;

//_____________________________________________________________________________

// SECTION:  Create (Post Request)

#[post("/tasks")]
async fn create_task(
    task: web::Json,
    data: web::Data,
) -> impl Responder {
    let mut tasks = data.lock().unwrap();
    tasks.insert(task.id, task.into_inner());
    HttpResponse::Created().json(task)
}

//_____________________________________________________________________________

// SECTION:  Read (Get Request)

#[get("/tasks")]
async fn list_tasks(data: web::Data<TaskStore>) -> impl Responder {
    let tasks = data.lock().unwrap();
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    HttpResponse::Ok().json(task_list)
}
//_____________________________________________________________________________


//_____________________________________________________________________________


// #[derive(Deserialize)]
// struct QueryParams {
//     status: Option<String>,
//     limit: Option<usize>
// }
//
// #[get("/tasks")]
// async fn list_tasks(query: web::Query<QueryParams>) -> impl Responder {
//
//     let status = query.status.as_deref().unwrap_or("all");
//     let limit = query.limit.unwrap_or(10);
//
//     let message = format!("Fetching {limit} tasks with status: {status}");
//
//     HttpResponse::Ok().body(message)
// }
//
//_____________________________________________________________________________



//_____________________________________________________________________________


//_____________________________________________________________________________
//_____________________________________________________________________________

#[actix_web::main]
async fn main() {

    let actix_web_app = || {
    App::new()
        // .service(get_task)
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
