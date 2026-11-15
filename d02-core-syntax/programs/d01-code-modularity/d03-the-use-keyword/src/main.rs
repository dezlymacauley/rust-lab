/*
    ABOUT: The `use` keyword

    This keyword allows you import specific constructs from a module,
    such as a function and structs, into the current file so that you can
    use them as if they were declared in this file.
*/

// First declare the module
mod greetings_module;

//_____________________________________________________________________________

// Import the functions that you want to use from the module
use greetings_module::{evening_greeting, morning_greeting};

fn main() {

    // TIP: In Neovim if you move your cursor to any letter in the name
    // of the function and press `Ctrl ]` it will take you right to the
    // location in the file that contains the declaration of the function.

    // Then you can press `Ctrl o` to jump back to previous file

    morning_greeting("Seth");
    evening_greeting("Seth");
    // Good morning Seth
    // Good evening Seth
}
