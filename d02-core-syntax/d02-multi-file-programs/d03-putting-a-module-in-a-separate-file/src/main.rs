/*
    ABOUT: Putting a module in a separate file

*/

// First declare the module:
// This is the file `greetings_module.rs`
mod greetings_module;

//_____________________________________________________________________________

// Import the functions that you want to use from the module
// use greetings_module::{evening_greeting, morning_greeting};

fn main() {
    greetings_module::morning_greeting("Seth");
    greetings_module::evening_greeting("Seth");
    // Good morning Seth
    // Good evening Seth
}
