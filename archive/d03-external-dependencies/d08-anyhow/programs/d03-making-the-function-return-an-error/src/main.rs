/*
    ABOUT: Making the function return an error
*/

mod custom_functions;
use std::io;

use custom_functions::read_user_config;

fn main() {
    let user_config_data: Result<String, io::Error> = read_user_config();

    match user_config_data {
        Ok(data) => {
            println!("user_config_data: {data}");
        }
        Err(error_message) => {
            eprintln!("Failed to read user_config.toml");
            eprintln!("Error: {error_message}");

            // Exit `main` immediately. "Program Completed" never prints.
            return;
        }
    }

    println!("Program Completed");
}
