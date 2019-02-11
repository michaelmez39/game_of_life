mod board;
mod game;

use self::board::Board;
use self::game::Game;



fn main() {
    let mut board = Board::new(40, 40);
    // board.flip_point(11, 10);
    // board.flip_point(12, 10);
    // board.flip_point(13, 10);
    
    // board.flip_point(18, 10);
    // board.flip_point(13, 13);
    // board.flip_point(16, 10);
    board.fill_rand();
    let mut quick_game = Game::new(50, board, 5);
    quick_game.start();

}

