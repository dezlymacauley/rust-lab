/*
    ABOUT: Making the function return an error

*/

mod custom_functions;
use custom_functions::read_user_config;

fn main() {
    read_user_config();

    // This code will always run regardless of what happens when
    // the function `read_user_config` is called because `read_user_config`
    // does not return any data to the caller. 
    println!("Program Completed");    
}
