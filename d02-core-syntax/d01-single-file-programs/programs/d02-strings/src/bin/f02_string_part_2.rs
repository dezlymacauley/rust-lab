/*
    ABOUT: String

*/

fn main() {

    //_________________________________________________________________________

    let user_name: String = String::from("Dezly");
    println!("\nThe contents of user_name are stored at");
    println!("memory address: {:p}", user_name.as_ptr());
    // The contents of user_name are stored at
    // memory address: 0x55c984a2ed50

    // NOTE: A move happens here

    let snapshot_of_user_name: String = user_name;
    println!("\nThe contents of snapshot_of_user_name are stored at");
    println!("memory address: {:p}", snapshot_of_user_name.as_ptr());
    // The contents of snapshot_of_user_name are stored at
    // memory address: 0x55c984a2ed50

    //_________________________________________________________________________
   
    // The code will not compile is you try to use the variable `user_name`.
    // println!("user_name: {user_name}");

    //_________________________________________________________________________
}
