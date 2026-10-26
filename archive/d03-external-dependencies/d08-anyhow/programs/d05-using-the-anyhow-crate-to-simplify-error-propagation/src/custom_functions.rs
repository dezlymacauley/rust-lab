/*
    ___________________________________________________________________________

    The `anyhow` package allows you to simplify error propagation with the
    data type `anyhow::Result<T>`

    `T` is the data type that should be returned by the function
    if it is successful

    So this:
    pub fn read_user_config() -> Result<String, io::Error> {

    Becomes:
    pub fn read_user_config() -> anyhow:Result<String> {

    ___________________________________________________________________________

    The `anyhow` package also allows you to add error message before,
    using error propagation.

    You do this by briging the `Context` trait into scope:

    use anyhow::Context;

    and then using `.context("You custom error message")`,
    followed by the `?` operator for error propagation.


    ___________________________________________________________________________

*/

use std::fs;

use anyhow::Context;

pub fn read_user_config() -> anyhow::Result<String> {
    let user_config_data: String = fs::read_to_string("src/user_config.toml")
        .context("Failed to read src/user_config_data.toml")?;
    Ok(user_config_data)
}
