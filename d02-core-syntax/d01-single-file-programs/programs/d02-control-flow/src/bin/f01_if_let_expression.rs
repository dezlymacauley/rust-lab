/*
    ABOUT: `if let` expression

    Use case:
    1. You have a variable that is an enum data type.

    2. You have some logic that should only run if that variable is set to
    a specific enum variant.
*/

fn main() {

    // driver_name is an Option enum.
    // It has two enum variants:
    // Option<T>
    // None
    
    let driver_name: Option<String> = Some(String::from("Batman"));

    // if driver_name is set to `None`, this message won't appear.
    if let Some(value) = driver_name {
        println!("driver_name is set to: {value}");
    }
}
