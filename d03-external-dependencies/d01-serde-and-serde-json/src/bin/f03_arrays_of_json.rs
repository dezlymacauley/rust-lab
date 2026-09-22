/*
    ABOUT: Arrays of JSON
*/

// Brings the `Value` data type and the  `json` macro from the `serde_json`
// package into scope.
use serde_json::{json, Value};

fn main() {
    //_________________________________________________________________________

    // EXAMPLE: 1 => How to create the array of JSON

    let mut list_of_orders: Vec<Value> = vec![];
    //_________________________________________________________________________

    // EXAMPLE: 2 => How to add elements to the JSON array

    list_of_orders.push(json!(
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
    ));

    list_of_orders.push(json!(
        {
          "orderId": "91b2e67a-1120-438c-8ef2-2a91176b91c4",
          "status": "preparing",
          "isPickup": true,
          "customer": {
            "name": "Alex Chen",
            "address": "456 Oak Ave, Suite 101"
          },
          "driver": null,
          "items": ["Veggie Burger", "Sweet Potato Fries", "Iced Tea"],
          "discount": 3.50,
          "totalPrice": 16.20
        }
    ));

    println!("Raw payload\n");
    println!("{}\n", json!(list_of_orders));
    
    println!("Pretty Print\n");
    println!("{:#}\n", json!(list_of_orders));
    //_________________________________________________________________________
}
