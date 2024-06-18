mod board;

use board::TicTacToe;
use std::io::{self, stdin};
pub struct Player {
    pub name: String,
    pub alias: char,
}
fn main() {
    let mut player_list: Vec<Player> = Vec::new();
    println!("Welcome to Ric-Rac-Roe");
    print!("Enter Number of players: ");
    let mut str_buff = String::new();
    io::stdin()
        .read_line(&mut str_buff)
        .expect("Something went wrong");
    let player_count: u8 = match str_buff.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    for i in 0..=player_count {
        let mut str_buff = String::new();
        let mut player_name: String = String::new();
        let player_alias: char;
        print!("Enter Player {i} Name: ");
        io::stdin()
            .read_line(&mut player_name)
            .expect("Error Reading Player Name");
        print!("Enter Player {player_name}Alias: ");
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
    print!("Enter width: ");
    io::stdin()
        .read_line(&mut str_buffer)
        .expect("Error Reading Line");
    let board_width: u32 = match str_buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input was not a number"),
    };
    print!("Enter height");
    io::stdin()
        .read_line(&mut str_buffer)
        .expect("Error Reading Line");
    let board_height: u32 = match str_buffer.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Input was not a number"),
    };

    'game_loop: loop {
        let mut game: TicTacToe = TicTacToe::new(board_height, board_width, &player_list);
        for (index, player) in player_list.iter().enumerate() {}
    }
    let mut something = TicTacToe::new(3, 3, &player_list);
    something.set_space(0, 0, 10);
    something.set_space(2, 2, 10);
    something.set_space(1, 1, 10);
    something.show_board();
    something.check_board();
}
