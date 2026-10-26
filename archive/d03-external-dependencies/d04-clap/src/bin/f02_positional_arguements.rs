/*
    ABOUT: Positional arguments

    I'll be creating a CLI program called `does-it-exist`

    The program will be run like this:
    does-it-exist path_to_file_or_directory

    The program will then display a message to indicate if the file
    or directory exists.

    `path_to_file_or_directory` is called a positional argument.

    A positional argument is a value that a CLI tool uses at runtime,
    to complete a task.

*/

// The Rust standard library has PathBuf struct,
// which is a data type specifically for working with file paths.
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "does-it-exist",
    version,
    about = "Checks if a file or directory exists",
    long_about = r#"

    ❔ Does it exist
    _________________________________________________________________
    This CLI tool will print the following information:
    - If a file or directory exists
    _________________________________________________________________

    "#
)]
struct CliArgs {
    // `clap` treats fields as positional arguments by default.
    path_to_file_or_directory: PathBuf,
}

fn main() {
    let cli_args = CliArgs::parse();

    if cli_args.path_to_file_or_directory.exists() {
        println!("{} exists", cli_args.path_to_file_or_directory.display());
    } else {
        println!(
            "{} does not exist",
            cli_args.path_to_file_or_directory.display()
        );
    }
}

//_____________________________________________________________________________

// EXAMPLE: 1 => Checking a relative path

/*

cargo bin 03_positional_arguments -- \
    example.txt

*/

//_____________________________________________________________________________

// EXAMPLE: 2 => Checking an absolute path

/*

cargo bin 03_positional_arguments -- \
          "$HOME/.bashrc"

*/

//_____________________________________________________________________________
