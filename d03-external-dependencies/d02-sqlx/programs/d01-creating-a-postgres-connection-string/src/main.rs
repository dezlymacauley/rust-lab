// SECTION: 1 => Module Imports

mod postgres_config;

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

// STEP: 2 => Create a struct for the Postgres configuration

struct PostgresConfig {
    pguser: String,
    pgpassword: String,
    pghost: String,
    pgport: String,
    pgdatabase: String,
}

impl PostgresConfig {
    fn new() -> Self {
        Self {
            pguser: get_env_value("PGUSER"),
            pgpassword: get_env_value("PGPASSWORD"),
            pghost: get_env_value("PGHOST"),
            pgport: get_env_value("PGPORT"),
            pgdatabase: get_env_value("PGDATABASE"),
        }
    }

    fn connection_string(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.pguser,
            self.pgpassword,
            self.pghost,
            self.pgport,
            self.pgdatabase
        )
    }
}
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

    // STEP: 5 => Create a Postgres configuration

    let postgres_config: PostgresConfig = PostgresConfig::new();
    //_________________________________________________________________________

    // STEP: 6 => Use the PostgresConfig to create a connection String

    let postgres_connection_string: String =
        postgres_config.connection_string();

    println!("\npostgres_connection_string:");
    println!("{postgres_connection_string}\n");
    //_________________________________________________________________________
}
