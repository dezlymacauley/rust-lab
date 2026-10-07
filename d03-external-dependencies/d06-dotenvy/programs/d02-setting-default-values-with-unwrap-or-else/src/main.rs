use std::env;

fn main() {
    //_________________________________________________________________________

    // STEP: 1 => Create a `.env` file in the root of the project,
    // and ensure that it is listed in the `.gitignore` file

    // For this project the `.env` file should contains the following:
    // SVELTEKIT_UI_PORT=6969
    // POSTGRES_DB_PORT=5582

    //_________________________________________________________________________

    // STEP: 2 => Create an absolute file path to the `.env` file

    let file_path_of_dot_env: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/.env");

    //_________________________________________________________________________

    // STEP: 2 => use `dotenvy` to load the variables listed in the
    // `.env` file into the process environment

    let dotenvy_status: Result<(), dotenvy::Error> =
        dotenvy::from_path(file_path_of_dot_env);

    if let Err(error_message) = dotenvy_status {
        println!("\nFailed to load `.env` variables: {error_message}\n");
        // Exit the program after showing the error message above.
        return;
    }
    //_________________________________________________________________________

    // STEP: 3 => Use `env::var()` save environment variables from
    // the process environment into Rust variables.

    let sveltekit_ui_port: String = env::var("SVELTEKIT_UI_PORT")
        .unwrap_or_else(|error_message| {
            eprintln!("\nError: {error_message}");
            eprintln!("SVELTEKIT_UI_PORT is not set");
            eprintln!("The default port of 7000 will be used.\n");
            "7000".to_string()
        });

    let postgres_db_port: String =
        env::var("POSTGRES_DB_PORT").unwrap_or_else(|error_message| {
            eprintln!("\nError: {error_message}");
            eprintln!("POSTGRES_DB_PORT is not set");
            eprintln!("The default port of 5432 will be used.\n");
            "5432".to_string()
        });

    println!("\nsveltekit_ui_port: {sveltekit_ui_port}");
    println!("postgres_db_port: {postgres_db_port}\n");

    //_________________________________________________________________________
}
