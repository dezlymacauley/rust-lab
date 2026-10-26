use std::{fs, io};

/*
    ___________________________________________________________________________

    1. Starting point

    The data type of `attempt_to_read_user_config` is `Result<String, io::Error>`,
    because that is the return type of the function `read_to_string()`

    pub fn read_user_config() -> Result<String, io::Error> {

        let user_config_data: String;

        let attempt_to_read_user_config: io::Result<String> =
            fs::read_to_string("src/user_config.toml");

        match attempt_to_read_user_config {
            Ok(data) => {
                user_config_data = data.trim().to_string();
                return Ok(user_config_data);
            }
            Err(error_message) => {
                return Err(error_message);
            }
        }
    }
    ___________________________________________________________________________

    2. I get rid of the `attempt_to_read_user_config` variable,
    and set the value of `user_config_data` directly from `read_to_string()`.

    let user_config_data = fs::read_to_string("src/user_config.toml");

    Now the data type of `user_config_data` is `Result<String, io::Error>`

    This means that if `read_to_string()` succeeds,
    the value of `user_config_data` will be `Ok(String)`.

    This means that if `read_to_string()` fails,
    the value of `user_config_data` will be `Err(io::Error)`.
    ___________________________________________________________________________

    3. Using the `?` operator for error propagation.

    let user_config_data: String = fs::read_to_string("src/user_config.toml")?;

    Now the data type of `user_config_data` is a String.

    This is what the `?` operator does.

    This means that if `read_to_string()` succeeds,
    the data type of `user_config_data` will be `String`,
    and not `Result<String, io::Error>`. The `?` operator unwraps the enum so
    that you can directly access the data type inside `Ok()`

    This means that if `read_to_string()` fails,
    the `error_message` inside `Err()` will be returned to the function
    that `read_to_string()` is inside of.

    In this case, that is the `read_user_config()` function.

    So the question mark operator can simplify your code but the trade-off
    is this:

    The error type of the function that you are adding the question mark to
    must be compatible with the error type in the return type of the function
    that you are propagating the error to.

    Since `read_to_string()` returns `Err(io::Error)` when it fails,
    the return type of `read_user_config()` must be changed to:

    pub fn read_user_config() -> Result<String, io::Error> {
        let user_config_data: String = fs::read_to_string("src/user_config.toml")?;
        Ok(user_config_data)
    }
    ___________________________________________________________________________

*/

pub fn read_user_config() -> Result<String, io::Error> {
    let user_config_data: String = fs::read_to_string("src/user_config.toml")?;
    Ok(user_config_data)
}
