/*
    ABOUT: Arrays of JSON
*/

// Brings the `Value` data type and the  `json` macro from the `serde_json`
// package into scope.
use serde_json::{json, Value};

fn main() {
    //_________________________________________________________________________

    // EXAMPLE: 1 => How to create the array of JSON

    let list_of_orders: Value = json!([]);

    println!("\nlist_of_orders");
    println!("{list_of_orders}\n");
    //_________________________________________________________________________
}
