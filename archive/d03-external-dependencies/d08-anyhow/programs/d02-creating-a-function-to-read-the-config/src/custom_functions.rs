use std::{fs, io};

pub fn read_user_config() {

    let user_config_data: String;

    let attempt_to_read_user_config: io::Result<String> =
        fs::read_to_string("src/user_config.toml");

    match attempt_to_read_user_config {
        Ok(data) => {
            user_config_data = data.trim().to_string();
            println!("user_config_data: {user_config_data}");
        }
        Err(error_message) => {
            eprintln!("Failed to read user_config.toml");
            eprintln!("Error: {error_message}");
            // I removed the `return` here.
            // It's not needed because there are no lines of code after
            // the `match expression` ends.
        }
    }

}
