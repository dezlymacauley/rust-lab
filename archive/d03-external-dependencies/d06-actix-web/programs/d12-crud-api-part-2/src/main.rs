use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

type TaskStore = Mutex<HashMap<u32, Task>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("=== Module 6: Web Framework Basics ===");
    println!("Server running at http://127.0.0.1:8080");
    println!("Try these endpoints:");
    println!("  GET  /hello");
    println!("  GET  /tasks/123");
    println!("  GET  /tasks?status=completed&limit=5");
    println!("  POST /tasks (with JSON body)");
    println!("  PUT  /tasks/123 (with JSON body)");
    println!("  DELETE /tasks/123");

    // Initialize shared state
    let task_store = web::Data::new(Mutex::new(HashMap::new()));

    // Add some initial tasks
    {
        let mut tasks = task_store.lock().unwrap();
        tasks.insert(
            1,
            Task {
                id: 1,
                title: String::from("Learn Rust"),
                completed: false,
            },
        );
        tasks.insert(
            2,
            Task {
                id: 2,
                title: String::from("Build web app"),
                completed: true,
            },
        );
    }

    HttpServer::new(move || {
        App::new()
            .app_data(task_store.clone())
            // Lesson 6.1: HTTP Requests and Responses
            .service(hello)
            .service(get_task_by_id)
            .service(list_tasks_with_query)
            .service(create_task)
            .service(update_task)
            .service(delete_task)
            // Lesson 6.2: Shared State examples
            .service(list_all_tasks)
            .service(get_completed_tasks)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// ============================================
// Lesson 6.1: HTTP Requests and Responses
// ============================================

/// Basic route handler - the "Hello World" of Actix-web
#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix-web!")
}

/// Path parameters - extract dynamic segments from URL
#[get("/tasks/{id}")]
async fn get_task_by_id(id: web::Path<u32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Getting task with ID: {}", id))
}

/// Query parameters - extract optional parameters from URL
#[derive(Deserialize)]
struct TaskQuery {
    status: Option<String>,
    limit: Option<usize>,
}

#[get("/tasks")]
async fn list_tasks_with_query(query: web::Query<TaskQuery>) -> impl Responder {
    let status = query.status.as_deref().unwrap_or("all");
    let limit = query.limit.unwrap_or(10);
    HttpResponse::Ok().body(format!(
        "Listing tasks with status='{}' and limit={}",
        status, limit
    ))
}

/// JSON request body - deserialize JSON into Rust struct
#[post("/tasks")]
async fn create_task(task: web::Json<Task>) -> impl Responder {
    HttpResponse::Created().json(task.into_inner())
}

/// JSON response body - serialize Rust struct to JSON
#[get("/tasks/all")]
async fn list_all_tasks(data: web::Data<TaskStore>) -> impl Responder {
    let tasks = data.lock().unwrap();
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    HttpResponse::Ok().json(task_list)
}

/// HTTP status codes - return appropriate status codes
#[put("/tasks/{id}")]
async fn update_task(
    id: web::Path<u32>,
    task: web::Json<Task>,
    data: web::Data<TaskStore>,
) -> impl Responder {
    let mut tasks = data.lock().unwrap();
    if tasks.contains_key(&id) {
        let task_clone = task.clone();
        tasks.insert(*id, task.into_inner());
        HttpResponse::Ok().json(task_clone)
    } else {
        HttpResponse::NotFound().body(format!("Task with ID {} not found", id))
    }
}

/// DELETE endpoint - complete the CRUD operations
#[delete("/tasks/{id}")]
async fn delete_task(id: web::Path<u32>, data: web::Data<TaskStore>) -> impl Responder {
    let mut tasks = data.lock().unwrap();
    if tasks.remove(&id).is_some() {
        HttpResponse::Ok().body(format!("Task {} deleted", id))
    } else {
        HttpResponse::NotFound().body(format!("Task with ID {} not found", id))
    }
}

#[get("/tasks/completed")]
async fn get_completed_tasks(data: web::Data<TaskStore>) -> impl Responder {
    let tasks = data.lock().unwrap();
    let completed_tasks: Vec<Task> = tasks
        .values()
        .filter(|task| task.completed)
        .cloned()
        .collect();
    HttpResponse::Ok().json(completed_tasks)
}
