/*
    ABOUT: Loading Environment Variables

    1. Before creating a `.env` file,
    first ensure that `.env` is listed in the `.gitignore` file of
    the project.

    2. For this project, create a .env file with the following enviroment
    variables:

    PGUSER="postgres"
    PGPASSWORD="postgres"
    PGHOST="127.0.0.1"
    PGPORT="7676"
    PGDATABASE="rust_lab"

    3. Add `dotenvy` to your project:
    cargo add dotenvy

    NOTE: Ensure that you set a strong password for `PGPASSWORD`
*/

//_____________________________________________________________________________

// SECTION: 1 => Standard Library Imports

use std::env;
use std::path::PathBuf;
//_____________________________________________________________________________

// STEP: 1 => Create a helper function to store .env values in Rust variables

fn get_env_value(name_of_variable: &str) -> String {
    let custom_error_message: String =
        format!("❌ Error: Please set {name_of_variable} in the .env file");

    let env_value: String = match env::var(name_of_variable) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("\n{custom_error_message}\n");
            std::process::exit(1);
        }
    };

    env_value
}
//_____________________________________________________________________________

// STEP: 2 => Create a struct




//_____________________________________________________________________________

fn main() {
    // STEP: 2 => Get the absolute path to the project

    let project_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // println!("project_root: {}", project_root.display());
    //_________________________________________________________________________

    // STEP: 3 => Get the absolute path to the `.env` file

    let file_path_of_dot_env: PathBuf = project_root.join(".env");
    // println!("dot_env_file_path: {}", dot_env_file_path.display());
    //_________________________________________________________________________

    // STEP: 4 => Load the environment variables from the `.env` file

    // `dotenvy` will be used for loading environment variables listed
    // in a `.env` file into the process environment,
    // so that they can be stored as Rust variables.

    let dotenvy_status: Result<(), dotenvy::Error> =
        dotenvy::from_path(file_path_of_dot_env);

    match dotenvy_status {
        Ok(()) => {
            println!("\n✅ Successfully loaded `.env` variables");
        }
        Err(error_message) => {
            println!("\n❌ Erorr: Failed to load `.env` variables:");
            println!("{error_message}\n");
            // Exit the program after showing the error message above.
            return;
        }
    }
    //_________________________________________________________________________

    // STEP: 5 => Store the environment variables as Rust variables

    let pguser: String = get_env_value("PGUSER");
    let pgpassword: String = get_env_value("PGPASSWORD");
    let pghost: String = get_env_value("PGHOST");
    let pgport: String = get_env_value("PGPORT");
    let pgdatabase: String = get_env_value("PGDATABASE");
    //_________________________________________________________________________

    // STEP: 6 => Create a Postgres connection string

    let postgres_connection_string: String = format!(
        "postgresql://{pguser}:{pgpassword}@{pghost}:{pgport}/{pgdatabase}"
    );

    println!("\npostgres_connection_string:");
    println!("{postgres_connection_string}\n");
    //_________________________________________________________________________
}
