/*
    ABOUT: Error Propagation
*/

mod custom_functions;
use std::io;

use custom_functions::read_user_config;

fn main() -> Result<(), io::Error> {
    let user_config_data: String = read_user_config()?;
    println!("user_config_data: {}", user_config_data.trim());

    Ok(())
}
