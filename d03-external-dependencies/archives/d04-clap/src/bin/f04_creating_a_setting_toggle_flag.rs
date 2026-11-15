/*
    ABOUT: Creating a `setting toggle` flag

    I want to create a `--uppercase` flag that should work like this.

    cargo bin 04_creating_a_setting_toggle_flag -- \
        --greeting "Good morning" \
        --name Dezly \
        --uppercase

    If the user runs the program with the `--uppercase` flag,
    then tha value of cli_args.uppercase will be set to true.

    //_________________________________________________________________________ 

    If the user runs the program without the `--uppercase` flag,
    like this...

    cargo bin 04_creating_a_setting_toggle_flag -- \
        --greeting "Good morning" \
        --name Dezly \

    ...then the value of `cli_args.uppercase` will be set to false.
    
    //_________________________________________________________________________ 

*/

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "greeter",
    version,
    about = "Greets a user.",
    long_about = "A simple CLI application that demonstrates how to use \
                  Clap's derive API to parse command-line arguments and \
                  display a personalized greeting."
)]
struct CliArgs {
    #[arg(long, short)]
    name: String,

    #[arg(long, short, default_value_t = 200)]
    arrows: u32,

    #[arg(long, short, default_value_t = String::from("Hello"))]
    greeting: String,

    // `action = clap::ArgAction::SetTrue` means that when the program
    // is run with the flag `--uppercase`, or `-u`
    // cli_args.uppercase will be set to true.
    //
    // If the program is run without this flag,
    // then no action will be taken by `clap`,
    // and `cli_args.uppercase` will be set to the default value of a bool,
    // which is false
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    uppercase: bool,
}

fn main() {
    let cli_args = CliArgs::parse();

    let mut greeting_message: String =
        format!("{} {}", cli_args.greeting, cli_args.name);

    if cli_args.uppercase == true {
        greeting_message = greeting_message.to_uppercase();
    }

    println!("\n{greeting_message}\n");

    println!("\nuppercase is set to: {}\n", cli_args.uppercase);
}
