/*
    ABOUT: The `.clone()` method

    This is used when you want to create a variable that has a snapshot
    of an element that does not implement the clone trait.

    E.g. Like the `String` data type.
*/

fn main() {

    //_________________________________________________________________________

    let user_name: String = String::from("Dezly");
    println!("\nThe contents of user_name are stored at");
    println!("memory address: {:p}", user_name.as_ptr());
    // The contents of user_name are stored at
    // memory address: 0x55c984a2ed50

    let snapshot_of_user_name: String = user_name;
    println!("\nThe contents of snapshot_of_user_name are stored at");
    println!("memory address: {:p}", snapshot_of_user_name.as_ptr());
    // The contents of snapshot_of_user_name are stored at
    // memory address: 0x55c984a2ed50

    //_________________________________________________________________________
}
