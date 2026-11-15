/*
    ABOUT: query parameters
*/

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

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

// EXAMPLE: 1 => Required 

#[derive(Deserialize)]
struct SongsGetParams {
    song_name: String
}

#[get("/songs")]
async fn songs_get(params: web::Query<SongsGetParams>) -> impl Responder {

    let params: SongsGetParams = params.into_inner();

    let song_name: String = params.song_name; 

    let message = format!("You requested the song: {song_name}");

    HttpResponse::Ok().body(message)
}

// NOTE: This is how you use a query parameter

// http://localhost:8080/songs?song_name=Bury+the+Light

// You should get the following output back:
// You requested the song: Bury the Light

//_____________________________________________________________________________

// EXAMPLE: 2 => Optional

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

#[actix_web::main]
async fn main() {

    let actix_web_app = || {
    App::new()
        .service(get_task)
        .service(songs_get)
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
