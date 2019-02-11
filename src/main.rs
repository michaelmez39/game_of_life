mod board;
mod game;

use self::board::Board;
use self::game::Game;



fn main() {
    let mut board = Board::new(20, 20);
    board.flip_point(11, 11);
    board.flip_point(10, 11);
    board.flip_point(10, 12);
    board.flip_point(10, 13);
    
    // board.flip_point(18, 10);
    // board.flip_point(13, 13);
    // board.flip_point(16, 10);
    //board.fill_rand();
    let mut quick_game = Game::new(7, board, 2);
    quick_game.start();

}

