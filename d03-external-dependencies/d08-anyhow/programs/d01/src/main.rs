/*
    ABOUT:

    Create a file called `user_config.toml` in the `src`
    directory of the project.

    Add this to the file:

    [appearance]
    theme = "dark"
*/

use std::{fs, io};

fn main() {

    let user_config: String;

    let attempt_to_read_user_config: io::Result<String> =
        fs::read_to_string("user_config.toml");

    match attempt_to_read_user_config {
        Ok(data) => {
            user_config = data;
        },
        Err(error_message) => {
            eprintln!("Failed to read user_config.toml");
            eprintln!("Error: {error_message}");
            return;
        }
    }

    println!("user_config: {user_config}");

}
