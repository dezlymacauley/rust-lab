/*
    ABOUT: The `.clone()` method

    This creates an independent copy of a value when its type
    implements the `Clone` trait.

    For `String`, `.clone()` creates a new heap allocation
    containing a copy of the string data.
*/

fn main() {
    //_________________________________________________________________________

    let mut user_name: String = String::from("Dezly");
    println!("\nThe contents of user_name are stored at");
    println!("memory address: {:p}", user_name.as_ptr());
    // The contents of user_name are stored at
    // memory address: 0x56153e834d50

    let snapshot_of_user_name: String = user_name.clone();
    println!("\nThe contents of snapshot_of_user_name are stored at");
    println!("memory address: {:p}", snapshot_of_user_name.as_ptr());
    // The contents of snapshot_of_user_name are stored at
    // memory address: 0x56153e834d70
    
    //_________________________________________________________________________

    // NOTE: Why the the memory addresses are different

    // Because of the `.clone()` method, 
    // the `snapshot_of_user_name` gets a snapshot of the contents of 
    // `user_name` and this is stored at a different memory address.

    //_________________________________________________________________________
   
    // user_name is still valid because no transfer of ownership happened.
    println!("user_name: {user_name}");
    println!("snapshot_of_user_name: {snapshot_of_user_name}");

    //_________________________________________________________________________
}
