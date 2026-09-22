/*  
    
    ABOUT: package-name
    
    ___________________________________________________________________________

    SECTION: Introduction to JSON:

    JSON stands for "JavaScript Object Notation". 

    It is a language used for sending data.
    ___________________________________________________________________________

    SECTION: JSON syntax:

    {
        "key": value,
        "anotherKey": value
    }
    ___________________________________________________________________________
    
    SECTION: Valid JSON data types:

    ___________________________________________________________________________
    
    1. string

    {
      "username": "dezlymacauley",
      "displayName": "Dezly Macauley"
    }
    ___________________________________________________________________________
    
    2. number

    {
      "items": 30,
      "cost": 52.75
    }
    ___________________________________________________________________________
    
    3. bool
    
    {
      "isLoggedIn": true,
      "hasAdminAcess": false
    }
    ___________________________________________________________________________
    
    4. null (Used to show that a value deliberately empty)

    E.g. You have an database where it is optional for a user to add their
    phone number.

    {
      "phoneNumber": null
    }
    ___________________________________________________________________________
    
    5. arrays

    {
      "recoveryEmails": [ "kate@gmail.com", "kate@proton.me" ]
    }

    ___________________________________________________________________________
  
    6. object

    This is used for nested data. E.g. A food delivery app
    {
      "orderId": "PZ-1024",
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
      "totalPrice": 24.50
    }
    ___________________________________________________________________________
*/

fn main() {
    println!("\nf02\n");
}
