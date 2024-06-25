mod board;

use board::TicTacToe;
use std::io::{self, stdin, Read};
pub struct Player {
    pub name: String,
    pub alias: char,
}
fn main() {
    let mut player_list: Vec<Player> = Vec::new();
    println!("Welcome to Ric-Rac-Roe");
    println!("Enter Number of players: ");
    let mut str_buff = String::new();
    io::stdin()
        .read_line(&mut str_buff)
        .expect("Something went wrong");
    let player_count: u8 = match str_buff.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    for i in 0..=player_count - 1 {
        let mut str_buff = String::new();
        let mut player_name: String = String::new();
        let player_alias: char;
        println!("Enter Player {i} Name: ");
        io::stdin()
            .read_line(&mut player_name)
            .expect("Error Reading Player Name");
        player_name = player_name.trim_end().to_string();
        println!("Enter Player {player_name} sAlias: ");
        match io::stdin().read_line(&mut str_buff) {
            Ok(_) => {
                let first = str_buff.chars().next();
                match first {
                    Some(c) => player_alias = c,
                    None => player_alias = '_',
                }
            }
            Err(_) => {
                panic!("Error getting Alias");
            }
        }
        player_list.push(Player {
            name: player_name,
            alias: player_alias,
        });
    }
    let mut str_buffer = String::new();
    println!("Enter board dimension");
    println!("Enter width: ");
    io::stdin()
        .read_line(&mut str_buffer)
        .expect("Error Reading Line");
    let board_width: u32 = match str_buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input was not a number"),
    };
    println!("Enter height");
    str_buffer.clear();
    io::stdin()
        .read_line(&mut str_buffer)
        .expect("Error Reading Line");
    let board_height: u32 = match str_buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input was not a number"),
    };

    let mut game: TicTacToe = TicTacToe::new(board_height, board_width, &player_list);
    'game_loop: loop {
        for (index, player) in player_list.iter().enumerate() {
            //print turn
            println!("{}'s turn : X--  Y| ", player.name);
            let mut coords: Vec<u32>;
            let mut temp_str: String = String::new();
            // get coords
            loop {
                temp_str.clear();
                match io::stdin().read_line(&mut temp_str) {
                    Ok(_) => {}
                    Err(_) => {
                        continue;
                    }
                }
                temp_str = temp_str.trim_end().to_string();
                coords = temp_str
                    .split(' ')
                    .map(|str| str.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                if coords.len() != 2 {
                    println!("Please Enter Two Coordinates");
                    continue;
                }
                break;
            }
            // place mark
            game.set_space(coords[0], coords[1], (index + 1) as u8);
            // print board
            game.show_board();
            let winner: u8 = game.check_board();
            println!("Winner {winner}");
        }
    }
}
