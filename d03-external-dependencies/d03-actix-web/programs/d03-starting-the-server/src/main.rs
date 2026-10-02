use actix_web::{App, HttpServer};

fn main() {
    #[allow(unused_variables)]
    let actix_web_app = App::new();

    let actix_web_server = HttpServer::new(|| {actix_web_app});
}
