use std::fmt::Display;
use std::iter::Iterator;
use std;

use termion::color;
use rand::prelude::*;

pub struct Board {
    width: usize,
    height: usize,
    contents: Vec<Vec<bool>>
}

impl Display for Board {
    fn fmt(&self ,format: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut buffer: String = String::with_capacity(self.height * self.width + self.height);
        let board = &self.contents;
        for row in 1..self.height {
            for column in 1..self.width {
                if board[row][column] {
                    buffer.push_str(&format!("{}", color::Bg(color::Red)));
                    buffer.push_str(" ");
                   
                } else {
                     buffer.push_str(&format!("{}", color::Bg(color::Reset)));
                      buffer.push_str(" ");
                   
                }
                
            }buffer.push_str(&format!("{}", color::Bg(color::Reset)));
            buffer.push_str("\n\r");
        }
        buffer.push_str("\n\r");
        format.write_str(&buffer)
    }
}

impl Iterator for Board{
    type Item = Board;

    fn next(&mut self) -> Option<Self::Item> {
        let mut future = self.contents.clone();
        for row in 1..self.height {
            for column in 1..self.width {
                let neighbors = self.neighbors(row, column);
                match neighbors {
                    0 | 1 => future[column][row] = false,
                    2 => future[row][column] = self.contents[row][column],
                    3 => future[row][column] = true,
                    _ => future[row][column] = false,
                }
            }
        }
        Some(Board {width: self.width, height: self.height, contents: future})
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        Board {width: width, height: height, contents: vec!(vec!(false; width + 1); height+1)}
    }

    pub fn flip_point(&mut self, y: usize, x: usize) -> Result<(), &str> {
        Ok(self.contents[y+1][x+1] = !self.contents[y+1][x+1])
    }

    fn neighbors(&self, row: usize, column: usize) -> u32 {
        self.adding_get(row+1, column - 1)
        + self.adding_get(row+1, column)
        + self.adding_get(row+1, column + 1)
        + self.adding_get(row, column - 1)
        + self.adding_get(row, column + 1)
        + self.adding_get(row-1, column - 1)
        + self.adding_get(row-1, column)
        + self.adding_get(row-1, column + 1)
    }

    fn adding_get(&self, row: usize, column: usize) -> u32{
        if let Some(r) = self.contents.get(row) {
            if let Some(c) = r.get(column) {
                if *c {return 1}
            }
        }
        0
    }
    pub fn fill_rand(&mut self) {
        let mut rng = thread_rng();
        for y in 0..self.height {
            for x in 0..self.width {
                self.contents[y][x] = rng.gen_bool(0.3);
            }
        }
    }
}
