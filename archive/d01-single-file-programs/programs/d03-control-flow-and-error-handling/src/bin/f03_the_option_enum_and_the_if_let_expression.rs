/*
    ABOUT: The `Option` enum and the `if let` expression

    Use case:
    1. You have a variable that is an enum data type.

    2. You have some logic that should only run if that variable is set to
    a specific enum variant.
*/

fn main() {

    //_________________________________________________________________________
   
    // EXAMPLE: 1 => Variable is set   

    // driver_name is an Option enum.
    // It has two enum variants:
    // Option<T>
    // None
    
    let support_character_name: Option<String> = Some("Bane".to_string());

    // if support_character_name is set to `None`, this message won't appear.
    if let Some(data) = support_character_name {
        println!("support_character_name is set to: {data}");
    }

    //_________________________________________________________________________
    
    // EXAMPLE: 2 => Variable is not set   

    let space_ship_name: Option<String> = None;

    // if space_ship_name is set to `None`, this message won't appear.
    if let Some(data) = space_ship_name {
        println!("space_ship_name is set to: {data}");
    }

    //_________________________________________________________________________
}
