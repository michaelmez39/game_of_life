use crate::board::Board;

use termion::raw::IntoRawMode;
use termion::clear;
use std::thread;
use std::time::Duration;
use std::io::{Write, stdout};

pub enum GameState {
    Running,
    Paused,
    Ended
}

pub struct Game {
    state: GameState,
    board: Board,
    generations: u32,
    gps: u64
}

impl Game {

    pub fn new(generations: u32, board: Board, gps: u64) -> Game {
        Game{
            state: GameState::Paused,
            board: board,
            generations: generations,
            gps: gps
        }
    }

    pub fn start(&mut self) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        
        self.state = GameState::Running;
        for _ in 0..self.generations {
            //write!(stdout, "{}", clear::All).unwrap();
            write!(stdout, "{:^count$}", self.board, count=(termion::terminal_size().unwrap().0 as usize)).unwrap();
            thread::sleep(Duration::from_millis(1000 / self.gps));
            self.board = self.board.next().unwrap();
            //write!(stdout, "{}", termion::clear::All);
        }
        self.state = GameState::Ended;
        stdout.flush().unwrap();
    }
}
