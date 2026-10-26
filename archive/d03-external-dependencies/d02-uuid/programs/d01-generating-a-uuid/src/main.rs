/*
    ABOUT: Generating a uuid

    This is what a `uuid` looks like:
    04ad6566-d3f6-4fbf-b52f-98c56deb4da7

    `uuid` stands for universally unique indentifier

    This allows each row in a database to have a unique id.

    E.g. If you had a `player database` and each row was the information
    of a specific player, then you would need a uuid.

    Without a uuid, you would run into problems if you had two users with
    the same name.

*/

use uuid::Uuid;

fn main() {
    let player_one_id: Uuid = Uuid::new_v4();
    let player_two_id: Uuid = Uuid::new_v4();

    println!("player_one_id: {player_one_id}");
    println!("player_two_id: {player_two_id}");
}
