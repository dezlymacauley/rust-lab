/*
    ABOUT: Creating a JSON payload
*/

// Brings the `Value` data type and the  `json` macro from the `serde_json`
// package into scope.
use serde_json::{json, Value};

fn main() {
    //_________________________________________________________________________

    // EXAMPLE: 1 => How to create the payload

    let json_payload: Value = json!(
        {
            "userId": "7f10960a-ea7f-4e21-b3dc-06becbb729be",
            "userName": "dezlymacauley",
            "displayName": "Dezly Macauley"
        }
    );

    //_________________________________________________________________________

    // EXAMPLE: 2 => How to print the raw payload in the terminal

    println!("\njson_payload (Raw)");
    println!("{json_payload}");
    // json_payload (Raw)
    // {"displayName":"Dezly Macauley","userId":"7f10960a-ea7f-4e21-b3dc-06becbb729be","userName":"dezlymacauley"}

    //_________________________________________________________________________

    // EXAMPLE: 3 => How to pretty print the raw payload in the terminal

    println!("\njson_payload (Pretty Print)");
    println!("{json_payload:#}\n");
    /*

        json_payload (Pretty Print)
        {
          "displayName": "Dezly Macauley",
          "userId": "7f10960a-ea7f-4e21-b3dc-06becbb729be",
          "userName": "dezlymacauley"
        }

    */
    //_________________________________________________________________________
}
