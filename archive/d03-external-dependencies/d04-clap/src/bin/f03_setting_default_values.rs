/*
    ABOUT: Setting default values
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
    #[arg(long = "name", short = 'n')]
    user_name: String,

    /*
        If you don't specify a value for `long`,
        then the fieldname will be used for the long flag.
        So in this case, the long flag for this value will be `--arrows`

        If you don't specify a value for `short`,
        then the fieldname will be used for the short flag.
        So in this case the short flag for this value will be `-a`

        If the user does not give an argument for `arrows` when using the CLI
        then the default value will be set to 200
        The `t` in `default_value_t`, ensures that the default value matches
        the data type set in the field.
    */
    #[arg(long, short, default_value_t = 200)]
    arrows: u32,

    // If you don't specify a value for `short`,
    // then then the first letter of the field name will be used for the flag.
    #[arg(long, short, default_value_t = String::from("Hello"))]
    greeting: String,
}

fn main() {
    let cli_args = CliArgs::parse();

    let greeting_message: String =
        format!("{} {}", cli_args.greeting, cli_args.user_name);

    println!("\n{greeting_message}\n");
}

//_____________________________________________________________________________

/*
To run this program:

cargo bin 03_setting_default_values -- \
    --name Dezly 
*/

/*
To test out the custom greeting:

cargo bin 03_setting_default_values -- \
    --greeting "Good morning" \
    --name Dezly 
*/

//_____________________________________________________________________________
