/*
    ABOUT: The `Result` enum (Function example)
*/

fn get_team_leader_id() -> Result<u64, String> {
    Ok(42)
}

fn get_enemy_power_level() -> Result<u64, String> {
    Err("Unable to read enemy power level.".to_string())
}

fn main() {
    //_________________________________________________________________________

    // EXAMPLE: 1 => The function successfully returns a value 

    let team_leader_id: Result<u64, String> = get_team_leader_id();

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

    let enemy_power_level: Result<u64, String> = get_enemy_power_level();

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
