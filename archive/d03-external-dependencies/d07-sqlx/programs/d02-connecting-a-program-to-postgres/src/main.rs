/*
    ABOUT: Connecting a program to Postgres
*/

mod custom_data_types;
use custom_data_types::AppConfig;

mod custom_functions;
use custom_functions::establish_connection;
use sqlx::{Pool, Postgres};

#[tokio::main]
async fn main() -> anyhow::Result<()>  {

    //_________________________________________________________________________

    // STEP: 1 => Load the program configuration

    let app_config: AppConfig = AppConfig::new()?;
    app_config.print_config();
    //_________________________________________________________________________
    
    // STEP: 2 => Establish a connection to the database
   
    let _database_connection_pool: Pool<Postgres> = 
        establish_connection(&app_config.database_url).await?;

    //_________________________________________________________________________
    
    Ok(())
}
