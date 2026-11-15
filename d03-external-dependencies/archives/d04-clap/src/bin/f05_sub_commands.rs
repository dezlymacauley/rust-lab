/*
    ABOUT: `Sub-commands`

    Most cli programs use sub-commands to structure the functionality
    of the cli program.

    Each sub-command will have its own flags.

    The syntax is:

    command sub-command flags_that_only_work_with_that_sub_command

    A popular example of this would be the `deno` cli,
    which is a Rust-powered JavaScript and TypeScript runtime.

    ___________________________________________________________________________

    E.g.

    mkdir deno-typescript-project
    cd deno-typescript-project

    deno init
    ✅ Project initialized

    Run these commands to get started

      # Run the server
      deno run --allow-net main.ts

      # Run the server and watch for file changes
      deno task dev

      # Run the tests
      deno test

    ___________________________________________________________________________

    `deno` is the root command that runs the cli tool
    ___________________________________________________________________________

    If you examine the command `deno init`

    `init` is a sub-command that creates a new Deno project.
    ___________________________________________________________________________

    If you examine the command `deno run --allow-net main.ts`

    `run` is a sub-command that runs a file.

    `--allow-net` is a setting toggle flag that only works
    with the sub-command `run`

    `main.ts` is a positional argument (the name of the file to run)

    It runs a file while giving that file permission
    to make network requests.

    ___________________________________________________________________________

    If you examine the command `deno task dev`

    `task` is a sub-command,
    and `dev` is a positional argument (The name of a script listed in a
    package.json or deno.json file that should be run)

    This runs a script called `dev`

    ___________________________________________________________________________

    If you examine the command `deno test`

    `test` is a sub-command that runs the tests in the project

    ___________________________________________________________________________

    In this guide I will be creating a mock version of the Deno command.

    ___________________________________________________________________________
*/

use std::path::PathBuf;

// The first thing you need to do is bring the `Subcommand` trait into scope.
//
// This trait works in a similar way to the `Parser` trait,
// except that it is attached to enums instead of structs.
use clap::{Parser, Subcommand};

// This adds the `Subcommand` trait to the enum below.
// Think of this as a list of all the sub-commands of
// the `mock-deno` root command.
// Each enum variant is a sub-command
#[derive(Subcommand)]
enum ValidSubCommand {
    // This is `mock-deno init` command
    Init,

    // This is the `mock-deno run` command
    Run {
        // this is the positional argument for the `mock-deno run` command
        // Usage:
        // mock-deno run file_path
        //
        // E.g.
        // mock-deno run "src/main.ts"
        file_path: PathBuf,

        // A setting toggle flag.
        #[arg(long = "allow-net")]
        allow_net: bool,
    },
   // This is `mock-deno dev`
    Task {
        // this is the positional argument for the `mock-deno task` command
        // Usage:
        // mock-deno run name_of_task
        //
        // E.g.
        // mock-deno run build
        task_name: String,
    },
   // This is `mock-deno test`
    Test,
}

#[derive(Parser)]
#[command(
    name = "mock-deno",
    version,
    about = "A mock version of the Deno Cli",
    long_about = r#"

    Mock Deno
    _________________________________________________________________
    This is a mock version of the Deno CLI
    Here are some commands I've implemented:
    
    mock-deno init
    
    mock-deno run --allow-net(this is optional)  <file name>

    mock-deno task <task name>

    mock-deno test
    _________________________________________________________________

    "#
)]
struct CliArgs {
    // This is how you add the sub-command functionality to the `mock-deno`
    // command.
    //
    // `sub_command: ValidSubCommand` ensures that only valid sub-commands
    // from the `ValidSubCommand` enum can be used.
    #[command(subcommand)]
    sub_command: ValidSubCommand,
}

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.sub_command {
        ValidSubCommand::Init => {
            println!("\n✅ Project initialized\n");
        }
        // `file_path, ..` Is just a way of telling Rust that I only want
        // to access
        // to the value `file_path` field so that I can print it.
        // The other fields are not needed for the println! statement below
        ValidSubCommand::Run {
            file_path,
            allow_net,
        } => {
            if allow_net {
                println!(
                    "\nRunning {} with network access.\n",
                    file_path.display()
                );
            } else {
                println!("\nRunning {}\n", file_path.display());
            }
        },
        ValidSubCommand::Task {
            task_name
        } => {
            println!("\nRunning task: {task_name}\n");
        },
        ValidSubCommand::Test => {
            println!("\n🧪 Running tests\n");
        }

    }
}

//_____________________________________________________________________________

// EXAMPLE: 1 => Using the `init` sub-command

/*

cargo bin 06_sub_commands -- \
    init

*/

//_____________________________________________________________________________

// EXAMPLE: 2 => Using the `run` sub-command

/*

cargo bin 06_sub_commands -- \
    run "src/main.ts"

*/

//_____________________________________________________________________________

// EXAMPLE: 3 => Using the `run` sub-command with the `--allow-net` flag

/*

cargo bin 06_sub_commands -- \
    run --allow-net "src/main.ts"

*/

//_____________________________________________________________________________

// EXAMPLE: 4 => Using the `task` sub-command

/*

cargo bin 06_sub_commands -- \
    task dev

*/

//_____________________________________________________________________________

// EXAMPLE: 5 => Using the `test` sub-command

/*

cargo bin 06_sub_commands -- \
    test

*/

//_____________________________________________________________________________
