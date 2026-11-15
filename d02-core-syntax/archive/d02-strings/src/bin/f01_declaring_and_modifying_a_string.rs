/*
    ABOUT: Declaring and modifying a `String`
*/

fn main() {
    //_________________________________________________________________________

    let mut user_name: String = String::from("Dezly");

    println!("\nThe contents of user_name are: {user_name}");

    println!("\nThe contents of user_name are stored at:");
    println!("Memory Adress: {:p}", user_name.as_ptr());
    // The contents of user_name are stored at:
    // Memory Adress: 0x5634e41d4d50

    //_________________________________________________________________________

    user_name = String::from("Renji");
    println!("\nThe contents of user_name are: {user_name}");
    // The contents of user_name are: Renji
    
    println!("\nThe contents of user_name are stored at:");
    println!("Memory Adress: {:p}", user_name.as_ptr());
    // The contents of user_name are stored at:
    // Memory Adress: 0x558022f02d70

    //_________________________________________________________________________
    

    //_________________________________________________________________________
}
