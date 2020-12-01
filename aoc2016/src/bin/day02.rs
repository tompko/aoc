use std::io::{BufReader, BufRead};
use std::fs::File;

struct Board {
    board: Vec<char>,
    x: usize,
    y: usize,
    size: usize,
    answer: String,
}

impl Board {
    fn new(desc: &str, size: usize) -> Self {
        let mut x = 0;
        let mut y = 0;

        let board: Vec<_> = desc.chars().collect();

        for (index, ch) in board.iter().enumerate() {
            if *ch == '5' {
                x = index % size;
                y = index / size;
            }
        }

        Board{
            board: board,
            x: x,
            y: y,
            size: size,
            answer: String::new(),
        }
    }

    fn simulate(&mut self, moves: &str) {
        let limit = self.size - 1;
        for ch in moves.chars() {
            let mut x = self.x;
            let mut y = self.y;

            match ch {
                'U' => y = if y == 0 { 0 } else { y - 1},
                'D' => y = if y == limit { limit } else { y + 1},
                'L' => x = if x == 0 { 0 } else { x - 1},
                'R' => x = if x == limit { limit } else { x + 1},
                _ => unreachable!(),
            }

            if self.board[self.index(x, y)] != 'X' {
                self.x = x;
                self.y = y;
            }
        }

        let index = self.index(self.x, self.y);
        self.answer.push(self.board[index]);
    }

    fn answer(&self) -> &str {
        &self.answer
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (y * self.size) + x
    }
}

fn main() {
    let file = File::open("input/day02.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut board1 = Board::new("123456789", 3);
    let mut board2 = Board::new("XX1XXX234X56789XABCXXXDXX", 5);

    for line in reader.lines() {
        let line = line.unwrap();

        board1.simulate(&line);
        board2.simulate(&line);
    }

    println!("1: {}", board1.answer());
    println!("2: {}", board2.answer());
}
