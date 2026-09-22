/*
    ABOUT: Searching by id
*/

use serde_json::{json, Value};

fn main() {
    let mut list_of_orders: Vec<Value> = vec![];

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

    //_________________________________________________________________________

    // EXAMPLE: 1 => How to safely search for a specific order

    let id_to_find: String =
        String::from("#91b2e67a-1120-438c-8ef2-2a91176b91c4");

    let order_exists: bool = list_of_orders
        .iter()
        .any(|element| element["orderId"] == id_to_find);

    if !order_exists {
        println!("\nCould not find order {id_to_find}\n");

        // I don't want to exit from `fn main` immeadiately if there
        // is an error.
        return;
    }

    println!("✅ Success: Order {id_to_find} was found");

    //_________________________________________________________________________
}
