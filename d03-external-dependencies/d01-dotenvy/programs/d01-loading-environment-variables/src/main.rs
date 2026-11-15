/*
    ABOUT: Loading Environment Variables

    1. Before creating a `.env` file,
    first ensure that `.env` is listed in the `.gitignore` file of
    the project.

    2. For this project, create a .env file with the following enviroment
    variables:

    PGUSER="1"
    PGPASSWORD="2"
    PGHOST="3"
    PGPORT="4"
    PGDATABASE="5"

    3. Add `dotenvy` to your project:
    cargo add dotenvy
*/

//_____________________________________________________________________________

// SECTION: 1 => Standard Library Imports

use std::path::PathBuf;

//_____________________________________________________________________________

// SECTION: 2 => External Imports

//_____________________________________________________________________________

fn main() {
    // STEP: 1 => Get the absolute path to the project

    let project_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // println!("project_root: {}", project_root.display());
    //_________________________________________________________________________

    // STEP: 2 => Get the absolute path to the `.env` file
    
    let file_path_of_dot_env: PathBuf = project_root.join(".env");
    // println!("dot_env_file_path: {}", dot_env_file_path.display());
    //_________________________________________________________________________
    
    // STEP: 3 => Load the environment variables from the `.env` file

    // `dotenvy` will be used to load the environment variables listed in
    // the `.env` file into the process environment. 

    // let dotenvy_status: Result<(), dotenvy::Error> =
    //     dotenvy::from_path(file_path_of_dot_env);
    //
    // match dotenvy_status {
    //     Ok(()) => {
    //         println!("\nSuccessfully loaded `.env` variables\n");
    //     }
    //     Err(error_message) => {
    //         println!("\nFailed to load `.env` variables: {error_message}\n");
    //         // Exit the program after showing the error message above.
    //         return;
    //     }
    // }

    //_________________________________________________________________________
}
