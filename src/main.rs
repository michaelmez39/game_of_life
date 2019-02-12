mod board;
mod game;

use self::board::Board;
use self::game::Game;



fn main() {
    let mut board = Board::new(100, 100);
    board.fill_rand();
    let mut quick_game = Game::new(100, board, 10);
    quick_game.start_terminal().expect("error writing to the terminal. Make sure to use a terminal that supports unicode...");
}
