/*

    ABOUT: Connecting to Postgres

    Add the following external dependencies to your project:
    cargo add dotenvy
    cargo add tokio --features full
    cargo add sqlx --features runtime-tokio,macros,time,postgres

*/

// SECTION: 1 => Module Imports

mod environment_setup;
use environment_setup::PostgresConfig;
//_____________________________________________________________________________

fn main() {
    //_________________________________________________________________________

    // STEP: 1 => Load the environment variables

    match environment_setup::load_dot_env() {
        Ok(()) => {
            println!("\n✅ Successfully loaded `.env` variables");
        }
        Err(error_message) => {
            eprintln!("\n❌ Error: Failed to load `.env` variables:");
            eprintln!("{error_message}\n");

            return;
        }
    }
    //_________________________________________________________________________

    // STEP: 2 => Create a Postgres config and connection string

    let postgres_config: PostgresConfig = PostgresConfig::new();
    let postgres_connection_string: String =
        postgres_config.connection_string();

    println!("\npostgres_connection_string:");
    println!("{postgres_connection_string}");
    //_________________________________________________________________________
}
