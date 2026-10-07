/*
    ABOUT: Setting up SQLx with Postgres
*/

// Result enum syntax:
// Result<Data type returned if it successful, Data type returned if failed>
//
// Result<(), String>
// means that the function `main` can either return the unit type `()`,
// which is Rust's way of showing that a function returns nothing.
//
// Or the function will return a String if it fails
fn main() -> Result<(), String> {
    println!("\nConnected to Postgres\n");

    Ok(())
}
