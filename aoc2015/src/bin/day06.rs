extern crate fancy_regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use fancy_regex::Regex;

enum Action {
    On,
    Toggle,
    Off,
}

struct Instruction {
    start: (usize, usize),
    stop: (usize, usize),
    action: Action,
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Instruction, &'static str> {
        let re = Regex::new(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
        let m = re.captures(line).expect("Failed to parse line").expect("No groups found");

        let start = (m.get(2).unwrap().as_str().parse().unwrap(), m.get(3).unwrap().as_str().parse().unwrap());
        let stop = (m.get(4).unwrap().as_str().parse().unwrap(), m.get(5).unwrap().as_str().parse().unwrap());

        Ok(Instruction{
            start: start,
            stop: stop,
            action: match m.get(1).unwrap().as_str() {
                "turn on" => Action::On,
                "toggle" => Action::Toggle,
                "turn off" => Action::Off,
                _ => unreachable!(),
            }
        })
    }
}

struct Board {
    board: Box<[[u32; 1000]]>,
}

impl Board {
    pub fn new() -> Self {
        Board{
            board: vec![[0; 1000]; 1000].into_boxed_slice(),
        }
    }

    pub fn part1(&mut self, instr: &Instruction) {
        let (a, b) = instr.start;
        let (c, d) = instr.stop;
        for x in a..c+1 {
            for y in b..d+1 {
                match instr.action {
                    Action::On => {
                        self.board[y][x] = 1;
                    },
                    Action::Toggle => {
                        self.board[y][x] = 1 - self.board[y][x];
                    },
                    Action::Off => {
                        self.board[y][x] = 0;
                    },
                }
            }
        }
    }

    pub fn part2(&mut self, instr: &Instruction) {
        let (a, b) = instr.start;
        let (c, d) = instr.stop;
        for x in a..c+1 {
            for y in b..d+1 {
                match instr.action {
                    Action::On => {
                        self.board[y][x] += 1;
                    },
                    Action::Toggle => {
                        self.board[y][x] += 2;
                    },
                    Action::Off => {
                        self.board[y][x] = self.board[y][x].saturating_sub(1);
                    },
                }
            }
        }
    }

    pub fn count(&self) -> u32 {
        self.board.iter().map(|x| -> u32 { x.iter().sum() }).sum()
    }
}

fn main() {
    let f = File::open("inputs/day06.in").unwrap();
    let file = BufReader::new(&f);

    let mut part1 = Board::new();
    let mut part2 = Board::new();

    for line in file.lines() {
        let i: Instruction = line.unwrap().parse().unwrap();
        part1.part1(&i);
        part2.part2(&i);
    }

    println!("1: {}", part1.count());
    println!("2: {}", part2.count());
}
