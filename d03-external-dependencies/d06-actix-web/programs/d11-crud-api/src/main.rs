/*
    ABOUT: CRUD API
*/

use std::{collections::HashMap, sync::Mutex};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, put, delete, web};
use serde::{Deserialize, Serialize};

//_____________________________________________________________________________

// SECTION: Data Structures 

// Serialize = Rust data → JSON
// Deserialize = JSON → Rust data
#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

// The Mutex is to ensure that only one part of the program,
// to be specific, one thread can access this in-memory database.
type TaskStore = Mutex<HashMap<u32, Task>>;

//_____________________________________________________________________________

// SECTION: Create (Post Request)

#[post("/tasks")]
async fn create_task(
    task: web::Json<Task>,
    data: web::Data<TaskStore>,
) -> impl Responder {
    let task = task.into_inner();
    let mut tasks = data.lock().unwrap();
    tasks.insert(task.id, task.clone());
    HttpResponse::Created().json(task)
}

//_____________________________________________________________________________

// SECTION: Read (Get Request)

#[get("/tasks")]
async fn list_tasks(data: web::Data<TaskStore>) -> impl Responder {
    let tasks = data.lock().unwrap();
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    HttpResponse::Ok().json(task_list)
}

//_____________________________________________________________________________

// SECTION: Update (Put Request)

#[put("/tasks/{id}")]
async fn update_task(
    id: web::Path<u32>,
    task: web::Json<Task>,
    data: web::Data<TaskStore>,
) -> impl Responder {
    let id = id.into_inner();
    let mut tasks = data.lock().unwrap();
    if let Some(existing_task) = tasks.get_mut(&id) {
        *existing_task = task.into_inner();
        HttpResponse::Ok().json(&*existing_task)
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

//_____________________________________________________________________________

// SECTION: (Delete Request)

#[delete("/tasks/{id}")]
async fn delete_task(
    id: web::Path<u32>,
    data: web::Data<TaskStore>,
) -> impl Responder {
    let id = id.into_inner();
    let mut tasks = data.lock().unwrap();
    if tasks.remove(&id).is_some() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Task not found")
    }
}

//_____________________________________________________________________________

#[actix_web::main]
async fn main() {
    let store = web::Data::new(TaskStore::new(HashMap::new()));

    let actix_web_app = move || {
        App::new()
            .app_data(store.clone())
            .service(create_task)
            .service(list_tasks)
            .service(update_task)
            .service(delete_task)
    };

    let ip_address: &str = "127.0.0.1";
    let port: u16 = 8080;

    let server_address = (ip_address, port);

    let actix_web_server = HttpServer::new(actix_web_app)
        .bind(server_address)
        .expect("Actix Web Server could not bind to the server address")
        .run();

    println!("Starting Actix Web Server at http://{ip_address}:{port}");

    if let Err(error) = actix_web_server.await {
        eprintln!("Actix Web Server failed: {error}");
    }
}
