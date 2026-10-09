/*
    ABOUT: Reading a `.toml` file to a String variable

    Create a file called `user_config.toml` in the `src`
    directory of the project.

    Add this to the `src/user_config.toml` file:

    [appearance]
    theme = "dark"
*/

use std::{fs, io};

fn main() {

    // This variable is a placeholder for the user config data.
    // The variable is simply being declared but not being initialized.
    // In simple terms, I'm telling Rust that this variable will contain
    // a String but I'm not setting a String value.
    //
    // This is safe to do in Rust, because Rust will not allow you to use
    // an unitialized variable.

    // Another thing to be aware of is that you don't need to add the `mut`
    // keyword so that the variable can be initialized later.
    // Rust does not view initialization as a mutation.
    //
    // The `mut` keyword is only needed if you intend to update the variable
    // after it has been intitalized.
    let user_config_data: String;

    let attempt_to_read_user_config: io::Result<String> =
        fs::read_to_string("src/user_config.toml");
    
    match attempt_to_read_user_config {
        Ok(data) => {
            user_config_data = data.trim().to_string();
        },
        Err(error_message) => {
            eprintln!("Failed to read user_config.toml");
            eprintln!("Error: {error_message}");

            // If there is an error, the function that this match expression
            // is inside of, which is `fn main`, will exit immeadiately.
            return;
        }
    }

    // This line is safe because it will never run if there is an error.
    println!("user_config_data: {user_config_data}");
}
