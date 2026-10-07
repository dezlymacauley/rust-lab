/*
    ABOUT: Generating multiple `uuid` values
*/

use std::io;
use uuid::Uuid;

#[derive(Debug)]
struct Player {
    // The value of the `id` field will be automatically generated when each
    // instance of `Player` is creted.
    id: Uuid,
    user_name: String,
}

impl Player {
    fn new(user_name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_name: user_name.to_string(),
        }
    }

    fn print_player_info(list_of_players: &[Player], player_uuid: &str) {
        let uuid: Uuid = Uuid::parse_str(player_uuid).unwrap();

        for player in list_of_players {
            if player.id == uuid {
                println!("\nPlayer Info of: {}", player.id);
                println!("{}", player.user_name);
                return;
            }
        }

        println!("Player not found.");
    }
}

fn main() {
    let mut list_of_players: Vec<Player> = vec![];
    list_of_players.push(Player::new("Cassie"));
    list_of_players.push(Player::new("Seth"));
    list_of_players.push(Player::new("Kevin"));

    println!("\nlist_of_players: {list_of_players:#?}");

    loop {
        let mut raw_search_input = String::new();

        println!("\nEnter player UUID:");
        io::stdin().read_line(&mut raw_search_input).unwrap();

        let search_input = raw_search_input.trim();

        if search_input == "exit" {
            break;
        }

        Player::print_player_info(&list_of_players, search_input);
    }
}
