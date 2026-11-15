/*  
    
    ABOUT: Creating a struct for passing command line arguments
    
*/

/*
    This allows you to add `Parser` trait from `clap`,
    onto structs by using the #[derive()] macro like this:

    #[derive(Parser)]

    The derive macro is used to add `traits` to to struct.
*/
use clap::Parser;

/*
    This macro automatically generates code to parse command line arguments
    by using the fields of the struct.

    This adds the `Parser` trait to the struct called `CliArgs`.
    When a struct that has the `Parser` trait,
    you will be able to use helper attributes like
    `#[command()]` and `#[arg()]`

    `#[command]` lets you add metadata to your CLI program
    `#[arg]` lets you configure the behavior of CLI arguments
    
    Each field inside the body of the is essentially a CLI argument.
*/
#[derive(Parser)]
#[command(

    // The name of the program
    name = "archer-stats",

    // `version` doesn't need a value because `clap` will get the version
    // number from the `Cargo.toml` file
    // Clap will get this from the `Cargo.toml` file
    version,

    about = "Print archer name and arrow count",

    long_about = r#"

    🏹 Archer Stats
    _________________________________________________________________
    This CLI tool will print the following information:
    - The name of an archer
    - The number of arrows they have
    _________________________________________________________________

    "#
)]
struct CliArgs {
    // This will automatically generate the flags
    // --name, and -n
    // Now you can run this program by using the following:
    // name-program --name "Dezly"
    // or
    // name-program --n "Dezly"
    #[arg(long = "name", short = 'n')]
    name: String,

    /*
        If you don't specify a value for `long`,
        then the fieldname will be used for the long flag.
        So in this case, the long flag for this value will be `--arrows`

        If you don't specify a value for `short`,
        then the fieldname will be used for the short flag.
        So in this case the short flag for this value will be `-a`
    */
    #[arg(long, short)]
    arrows: u32,
}

fn main() {
    // Create an instance of the CliArgs struct so that you access the
    // stored user input.
    let cli_args = CliArgs::parse();

    // This is how to access the data stored in the args struct
    println!("\ncli_args.name = {}", cli_args.name);
    println!("cli_args.arrows = {}\n", cli_args.arrows);
}

//_____________________________________________________________________________

// EXAMPLE: 1 => To run this program using the long flags
/*

cargo bin f01_creating_a_struct_for_parsing_arguments \
    -n "Dezly Macauley" \
    -a 123

*/

//_____________________________________________________________________________

// EXAMPLE: 2 => To run this program using the short flags

/*

cargo bin 02_creating_a_struct_for_parsing_arguments -- \
    -n "Dezly Macauley" \
    -a 123

*/

//_____________________________________________________________________________

// EXAMPLE: 3 => To check the version of the program (long flag)

/*

cargo bin 02_creating_a_struct_for_parsing_arguments -- \
    --version

*/

//_____________________________________________________________________________

// EXAMPLE: 4 => To check the version of the program (short flag)

/*

cargo bin 02_creating_a_struct_for_parsing_arguments -- \
    -V

*/

//_____________________________________________________________________________

// EXAMPLE: 5 => To display an overview of the help menu

/*

cargo bin 02_creating_a_struct_for_parsing_arguments -- \
    -h

*/

//_____________________________________________________________________________

// EXAMPLE: 6 => To display the full help menu

/*

cargo bin 02_creating_a_struct_for_parsing_arguments -- \
    --help

*/

//_____________________________________________________________________________
