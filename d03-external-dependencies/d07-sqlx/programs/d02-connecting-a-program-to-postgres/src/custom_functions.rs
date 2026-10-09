// SECTION: 1 => Enviroment Setup

use anyhow::Context;

// pub struct AppConfig {
//     pub sveltekit_ui_port: String,
//     pub postgres_db_port: String,
// }

// DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/ticketing_system
//
// PROTOCOL=postgres
// USERNAME=postgres
// PASSWORD=password
// HOST=127.0.0.1
// PORT=5432
// DATABASE_NAME=ticketing_system



pub fn load_dot_env_file() -> anyhow::Result<()> {
    let absolute_path_to_dot_env_file: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/.env");

    let dot_env_file_data: () =
        dotenvy::from_path(absolute_path_to_dot_env_file)
            .context("Failed to .env file")?;

    println!("\nSuccessfully loaded `.env` variables\n");
    Ok(dot_env_file_data)
}

//_____________________________________________________________________________

// SECTION: 2 => Database Setup


//_____________________________________________________________________________
