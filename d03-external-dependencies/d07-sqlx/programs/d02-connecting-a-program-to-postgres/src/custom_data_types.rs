use anyhow::Context;
use std::env;

pub struct AppConfig {
    pub protocol: String,
    pub user_name: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database_name: String,
    pub database_url: String,
}

impl AppConfig {
    /// Loads the `.env` file and reads all required environment variables,
    /// constructing the full `database_url` dynamically.
    pub fn new() -> anyhow::Result<Self> {
        let absolute_path_to_dot_env_file: &str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/.env");

        dotenvy::from_path(absolute_path_to_dot_env_file)
            .context("Failed to load .env file")?;

        println!("\nSuccessfully loaded `.env` variables\n");

        let protocol = env::var("PROTOCOL")
            .context("PROTOCOL is not set in environment")?;

        let user_name = env::var("USER_NAME")
            .context("USER_NAME is not set in environment")?;

        let password = env::var("PASSWORD")
            .context("PASSWORD is not set in environment")?;

        let host =
            env::var("HOST").context("HOST is not set in environment")?;

        let port =
            env::var("PORT").context("PORT is not set in environment")?;

        let database_name = env::var("DATABASE_NAME")
            .context("DATABASE_NAME is not set in environment")?;

        let database_url = format!(
            "{protocol}://{user_name}:{password}@{host}:{port}/{database_name}"
        );

        Ok(Self {
            protocol,
            user_name,
            password,
            host,
            port,
            database_name,
            database_url,
        })
    }
}
