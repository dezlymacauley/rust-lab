use anyhow::Context;
use std::env;

#[allow(dead_code)]
pub struct AppConfig {
    pub protocol: String,
    pub user_name: String,
    password: String,
    pub host: String,
    pub port: String,
    pub database_name: String,
    pub database_url: String,
}

impl AppConfig {
    /// Reads the `.env` file, loads the environment variables,
    /// and stores the values in an `AppConfig` struct.
    pub fn new() -> anyhow::Result<Self> {
        let absolute_path_to_dot_env_file: &str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/.env");

        dotenvy::from_path(absolute_path_to_dot_env_file)
            .context("Failed to load .env file")?;

        println!("\n✅ Successfully loaded `.env` variables\n");

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

    pub fn print_config(&self) {
        println!("    protocol: {}", self.protocol);
        println!("    user_name: {}", self.user_name);
        println!("    password: ****",);
        println!("    host: {}", self.host);
        println!("    port: {}", self.port);
        println!("    database_name: {}", self.database_name);

        let database_url = format!(
            "{}://{}:******@{}:{}/{}",
            self.protocol,
            self.user_name,
            self.host,
            self.port,
            self.database_name
        );

        println!("\n🗃️ database_url: {}\n", database_url);
    }
}
