/*
    ABOUT: Accessing fields in a JSON
*/

use serde_json::{json, Value};

fn main() {
    let mut list_of_orders: Vec<Value> = vec![];

    list_of_orders.push(json!(
        {
          "orderId": "44873812-9570-48d8-bff7-afa105b15e57",
          "status": "outForDelivery",
          "isPickup": false,
          "discount": null,
          "totalPrice": 24.50,
          "items": ["Pepperoni Pizza", "Spicy nuggets"],
          "customer": {
            "name": "Sarah Jenkins",
            "address": "123 Main St, Apt 4B"
          },
          "driver": {
            "name": "Marcus",
            "phone": "555-0198",
            "vehicle": "Silver Toyota Corolla"
          }
        }
    ));

    list_of_orders.push(json!(
        {
          "orderId": "91b2e67a-1120-438c-8ef2-2a91176b91c4",
          "status": "preparing",
          "isPickup": true,
          "discount": 3.50,
          "totalPrice": 16.20,
          "items": ["Veggie Burger", "Sweet Potato Fries", "Iced Tea"],
          "customer": {
            "name": "Alex Chen",
            "address": "456 Oak Ave, Suite 101"
          },
          "driver": null
        }
    ));

    //_________________________________________________________________________

    let id_to_find: String =
        String::from("91b2e67a-1120-438c-8ef2-2a91176b91c4");

    // NOTE: This time `.find` is used.

    let order: Option<&Value> = list_of_orders
        .iter()
        .find(|element| id_to_find == element["orderId"]);


    println!("\n✅ Success: Order {id_to_find} was found\n");


    //_________________________________________________________________________
    

    //_________________________________________________________________________
}
