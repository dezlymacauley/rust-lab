/*
    ABOUT: Inline modules, `mod` and `pub` 

    A module is used to group constructs that are related.
*/

// First declare the module in the same file.
mod greetings_module {

    // Add the functions to the module

    // Add the keyword `pub` before the name of the function,
    // to allow the function to be imported.

    pub fn morning_greeting(player_name: &str) {
        println!("Good morning {player_name}");
    }

    pub fn evening_greeting(player_name: &str) {
        println!("Good evening {player_name}");
    }
}

//_____________________________________________________________________________

// Import the functions that you want to use from the module
// use greetings_module::{evening_greeting, morning_greeting};

fn main() {
    greetings_module::morning_greeting("Seth");
    greetings_module::evening_greeting("Seth");
    // Good morning Seth
    // Good evening Seth
}
