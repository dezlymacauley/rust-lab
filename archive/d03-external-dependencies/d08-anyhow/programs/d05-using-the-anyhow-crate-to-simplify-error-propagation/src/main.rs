/*
    ABOUT: Using the `anyhow` crate to simplify `Error Propagation`
*/

mod custom_functions;
use custom_functions::read_user_config;

fn main() -> anyhow::Result<()> {
    let user_config_data: String = read_user_config()?;
    println!("user_config_data: {}", user_config_data.trim());

    Ok(())
}
