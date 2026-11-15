/*
    ABOUT: The `Result` enum (Variable example)

    ___________________________________________________________________________

    The `Result` enum is a built-in data type,
    that is frequently used in error handling. It has two primary use cases.
    ___________________________________________________________________________

    Use case 1:
    Rust uses it to mark a variable that has the potential for its value
    to be unusable because of a potential error when setting its value.
    ___________________________________________________________________________

    Use case 2:
    The `Result` enum is also used to mark functions that have the
    potential to fail.
    ___________________________________________________________________________

    This is the syntax of the Result enum.
    Note: You do not need to create it yourself.

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    ___________________________________________________________________________

    `OK(T)` and `Err(E)` explained.

    ___________________________________________________________________________
*/

fn main() {
    //_________________________________________________________________________

    // EXAMPLE: 1 => The value of the variable was set successfully

    let team_leader_id: Result<u64, String> = Ok(42);

    match team_leader_id {
        Ok(data) => {
            println!("team_leader_id: {data}");
        }
        Err(error_message) => {
            println!("Error: {error_message}");
        }
    }

    // team_leader_id: 42

    //_________________________________________________________________________

    // EXAMPLE: 2 => There was an error setting the value of the variable

    let enemy_power_level: Result<u64, String> =
        Err("Error: Unable to read enemy power level.".to_string());

    match enemy_power_level {
        Ok(data) => {
            println!("enemy_power_level: {data}");
        }
        Err(error_message) => {
            println!("Error: {error_message}");
        }
    }

    // Error: Error: Unable to read enemy power level.

    //_________________________________________________________________________
}
