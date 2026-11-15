// Brings the `env` module from the Rust standard library into scope
// I need this to use `env::var()`
use std::env;

fn main() {
    //_________________________________________________________________________

    // STEP: 1 => Create your `.env` file in the root of the project,
    // and ensure that it is listed in the `.gitignore` file

    // For this project my `.env` file only contains the following:
    // SVELTEKIT_UI_PORT=6969
    // POSTGRES_DB_PORT=5582

    //_________________________________________________________________________

    // STEP: 2 => Create an absolute file path to the `.env` file

    // The `concat!` macro is used to join two strings
    // `env!("CARGO_MANIFEST_DIR")` will give you the absolute 
    // file path of the Cargo.toml file of the project
    let file_path_of_dot_env: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/.env");

    //_________________________________________________________________________

    // step: 2 => use `dotenvy` to load the variables listed in the
    // `.env` file into the process environment

    // The process environment is a collection of environment variables
    // that is available to a process.
    // A process is simply a running program.

    // `from_path` returns a Result enum
    // 1. If it successfuly loads the variables from the `.env` file,
    // then it will return `()`. This is an empty tuple,
    // also known as the `unit type` in Rust.
    // This is used to show that the function does not return a value that
    // can be assigned to `dotenvy_status` if it succeeds.
    let dotenvy_status: Result<(), dotenvy::Error> =
        dotenvy::from_path(file_path_of_dot_env);

    match dotenvy_status {
        Ok(()) => {
            println!("\nSuccessfully loaded `.env` variables\n");
        }
        Err(error_message) => {
            println!("\nFailed to load `.env` variables: {error_message}\n");
            // Exit the program after showing the error message above.
            return;
        }
    }

    //_________________________________________________________________________

    // STEP: 3 => Use `env::var()` save environment variables from
    // the process environment into Rust variables.

    let sveltekit_ui_port: String =
        env::var("SVELTEKIT_UI_PORT").expect("SVELTEKIT_UI_PORT is not set");

    let postgres_db_port: String = env::var("POSTGRES_DB_PORT")
        .expect("POSTGRES_DB_PORT is not set");

    println!("sveltekit_ui_port: {sveltekit_ui_port}\n");
    println!("postgres_db_port: {postgres_db_port}\n");

    //_________________________________________________________________________


    //_________________________________________________________________________
}
