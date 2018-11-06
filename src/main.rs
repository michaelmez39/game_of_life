use std::fmt::Display;
use std::iter::Iterator;
struct Point(usize, usize);
struct Board {
    width: usize,
    height: usize,
    contents: Vec<Vec<bool>>
}

impl Display for Board {
    fn fmt(&self ,format: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut buffer: String = String::with_capacity(self.height * self.width + self.height);
        let board = self.contents.clone();
        for row in board {
            for item in row {
                match item {
                    true => buffer.push_str("*"),
                    false => buffer.push_str(" ")
            
                }
            }
            buffer.push_str("\n");
        }
        format.write_str(&buffer)
    }
}

impl Iterator for Board {
    type Item = Board;
    fn next(&mut self) -> Option<Self::Item> {
        let grid = self.contents.clone();
        let mut future = grid.clone();
        for column in 1..self.height {
            for row in 1..self.width {
                let neighbors = self.count_neighbors(row, column);
                match neighbors {
                    1 => future[column][row] = false,
                    2 => future[column][row] = grid[column][row],
                    3 => future[column][row] = true,
                    _ => future[column][row] = false,
                }
            }
        }
        Some(Board {width: self.width, height: self.height, contents: future})
    }
}

impl Board {
    fn new(width: usize, height: usize) -> Board {
        Board {width: width, height: height, contents: vec!(vec!(false; width + 1); height+1)}
    }

    fn flip_point(&mut self, point: Point) -> Result<(), &str> {
        Ok(self.contents[point.0][point.1] = !self.contents[point.0][point.1])
    }

    fn count_neighbors(&self, row: usize, column: usize) -> u32{
        let mut neighbors = 0;
        for i in 0..2 {
            for j in 0..2 {
                if self.contents[column + i - 1][row + j - 1]{
                    neighbors += 1;
                }
            }
        }
        return neighbors;
    }
}

fn main() {
    let mut board = Board::new(20, 20);
    board.flip_point(Point(10, 10));
    board.flip_point(Point(11, 10));
    board.flip_point(Point(12, 10));
    board.flip_point(Point(11, 11));
    board.flip_point(Point(11, 9));
    print!("{}", board);
    println!("");
    print!("{}", board.next().unwrap());
    println!("");
    print!("{}", board.next().unwrap());
    println!("");
    print!("{}", board.next().unwrap());
}

