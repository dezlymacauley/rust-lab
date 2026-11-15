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

fn main() {
    // STEP: 1 => Get the absolute path to the project

    let project_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // println!("project_root: {}", project_root.display());
    //_________________________________________________________________________

    // STEP: 2 => Get the absolute path to the `.env` file

    let file_path_of_dot_env: PathBuf = project_root.join(".env");
    // println!("dot_env_file_path: {}", dot_env_file_path.display());
    //_________________________________________________________________________

    // STEP: 3 => Load the environment variables from the `.env` file

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
            println!("\n❌ Failed to load `.env` variables:");
            println!("{error_message}\n");
            // Exit the program after showing the error message above.
            return;
        }
    }

    //_________________________________________________________________________

    // STEP: 4 => Store the environment variables as Rust variables

    let pguser: String = env::var("PGUSER").expect("PGUSER is not set");

    let pgpassword: String =
        env::var("PGPASSWORD").expect("PGPASSWORD is not set");

    let pghost: String = env::var("PGHOST").expect("PGHOST is not set");

    let pgport: String = env::var("PGPORT").expect("PGPORT is not set");

    let pgdatabase: String =
        env::var("PGDATABASE").expect("PGDATABASE is not set");

    //_________________________________________________________________________

    // STEP: 5 => Create a Postgres connection string

    let postgres_connection_string: String = format!(
        "postgresql://{pguser}:{pgpassword}@{pghost}:{pgport}/{pgdatabase}"
    );

    println!("\npostgres_connection_string:");
    println!("{postgres_connection_string}\n");
    //_________________________________________________________________________
}
