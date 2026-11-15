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
          "orderId": "44873812-9570-48d8-bff7-afa105b15e57",
          "status": "outForDelivery",
          "isPickup": false,
          "customer": {
            "name": "Sarah Jenkins",
            "address": "123 Main St, Apt 4B"
          },
          "driver": {
            "name": "Marcus",
            "phone": "555-0198",
            "vehicle": "Silver Toyota Corolla"
          },
          "items": ["Pepperoni Pizza", "Spicy nuggets"],
          "discount": null,
          "totalPrice": 24.50
        }
    );

    //_________________________________________________________________________

    // EXAMPLE: 2 => How to print the raw payload in the terminal

    println!("\njson_payload (Raw)");
    println!("{json_payload}");

    //_________________________________________________________________________

    // EXAMPLE: 3 => How to pretty print the raw payload in the terminal

    println!("\njson_payload (Pretty Print)");
    println!("{json_payload:#}\n");

    //_________________________________________________________________________
}
