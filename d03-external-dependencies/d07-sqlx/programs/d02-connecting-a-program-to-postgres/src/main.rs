/*
    ABOUT: Connecting a program to Postgres
*/

mod custom_functions;
use custom_functions::load_dot_env_file;

fn main() -> anyhow::Result<()>  {
    load_dot_env_file()?;

    Ok(())
}
