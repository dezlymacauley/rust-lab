use std::{fs, io};

/*
    I modified this function to return a String if it succeeds,
    and an error if it fails.

    To be more specific:
    1. If `read_user_config` succeeds, it should return `Ok(String)`
    2. If `read_user_config` fails, it should return `Err(error)`

    This will allow the function to be used to set a variable in the file
    that imports it.
*/
pub fn read_user_config() -> Result<String, io::Error> {
    let attempt_to_read_user_config: io::Result<String> =
        fs::read_to_string("src/user_config.toml");

    match attempt_to_read_user_config {
        Ok(data) => {
            let user_config_data = data.trim().to_string();
            return Ok(user_config_data);
        }
        Err(error_message) => {
            return Err(error_message);
        }
    }
}
