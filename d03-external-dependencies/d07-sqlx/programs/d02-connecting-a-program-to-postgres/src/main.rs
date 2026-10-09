/*
    ABOUT: Connecting a program to Postgres
*/

mod custom_data_types;
use custom_data_types::AppConfig;

fn main() -> anyhow::Result<()>  {

    let app_config: AppConfig = AppConfig::new()?;

    Ok(())
}
