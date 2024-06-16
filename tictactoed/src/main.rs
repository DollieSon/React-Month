mod board;

use board::TicTacToe;
fn main() {
    println!("Hello, world!");
    let mut something = TicTacToe::new(3, 3);
    something.set_space(0, 0, 10);
    something.set_space(2, 2, 10);
    something.set_space(1, 1, 10);
    something.show_board();
    something.check_board();
}
