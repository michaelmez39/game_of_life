use crate::board::Board;

use termion::raw::IntoRawMode;
use termion::clear;
use termion::screen;
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

    pub fn start_terminal(&mut self) -> Result<(), std::io::Error>{
        let mut screen = screen::AlternateScreen::from(stdout().into_raw_mode()?);
        write!(screen, "{}", termion::cursor::Hide)?;
        self.state = GameState::Running;
        for generation in 0..self.generations {
            write!(screen, "{}\r\n", clear::All)?;
            write!(screen, "{}", self.board)?;
            termion::cursor::Goto(self.board.height as u16, 1);
            write!(screen, "Generation: {}\r\n\r\n", generation)?;
            thread::sleep(Duration::from_millis(1000 / self.gps));
            self.board = self.board.next().unwrap();
            screen.flush().unwrap();
        }
        self.state = GameState::Ended;
        screen.flush().unwrap();
        write!(screen, "{}", termion::clear::All)?;
        write!(screen, "{}", termion::cursor::Show)?;
        write!(stdout(), "{}", clear::All)?;
        Ok(())
    }
}
