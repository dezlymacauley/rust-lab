// This brings the `App` struct from the external package `actix_web`,
// into scope.
use actix_web::App;

fn main() {
    /*
        `#[allow(unused_variables)]` is used to silence the compiler warning
        about the variable `actix_web` not being used.
    */
    #[allow(unused_variables)]
    let actix_web_app = App::new();

    // The variable `actix_web_app`, is an instance of `App`,
    // which is a struct that is used to configure your Actix App.

    // To create a working API, you need 3 things:
    // 1. `actix_web_app` must be configured to handle at least one request.
    // 2. `actix_web_app` must be attached to a server.
    // 3. The server needs to bind to a network address so that 
    // `actix_web_app` can listen for incomming requests.
}
