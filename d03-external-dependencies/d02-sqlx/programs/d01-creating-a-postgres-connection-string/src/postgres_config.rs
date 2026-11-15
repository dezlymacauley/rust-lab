/*
    ABOUT: postgres_config.rs

    The module is responsible for the following:

    1. Loading the variables declared in project's `.env` file 
    into the process environment.

    2. Creating a PostgresConfig struct that will store the values of those
    environment variables in the program,
    and use them to create a Postgres connection string that can be used 
    by SQLx to connect to a Postgres database.
*/

//_____________________________________________________________________________

// Standard Library Imports
use std::env;
use std::path::PathBuf;
//_____________________________________________________________________________

// STEP: 1 => Create the relevant function

// This function will load the variables declared in the `.env` file 
// of the project into the process environment.
pub fn load_dot_env() -> Result<(), dotenvy::Error> {
    let project_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let file_path_of_dot_env: PathBuf = project_root.join(".env");

    dotenvy::from_path(file_path_of_dot_env)
}

// This function ensures that the PostgresConfig can be created safely,
// by checking if the process environment contains the env values needed
// to create a PostgresConfig
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

// STEP: 2 => Create a struct for the PostgreSQL configuration

// This struct lists the environment variables that are needed to create
// a PostgresConfig
pub struct PostgresConfig {
    pguser: String,
    pgpassword: String,
    pghost: String,
    pgport: String,
    pgdatabase: String,
}

impl PostgresConfig {
    pub fn new() -> Self {
        Self {
            pguser: get_env_value("PGUSER"),
            pgpassword: get_env_value("PGPASSWORD"),
            pghost: get_env_value("PGHOST"),
            pgport: get_env_value("PGPORT"),
            pgdatabase: get_env_value("PGDATABASE"),
        }
    }

    pub fn connection_string(&self) -> String {
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
