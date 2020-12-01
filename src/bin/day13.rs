use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
enum Int {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Elf {
    pos: (usize, usize),
    dir: Dir,
    int: Int,
}

struct Game {
    grid: Vec<Vec<char>>,
    elves: Vec<Elf>,
    first_crash: Option<(usize, usize)>,
}

impl Game {
    fn parse(path: &'static str) -> Self {
        let file = File::open(path).expect("Failed to open input");
        let file = BufReader::new(&file);

        let mut grid = Vec::new();
        let mut elves = Vec::new();

        for (y, line) in file.lines().enumerate() {
            let line = line.unwrap();

            let mut grid_line = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    'v' => {
                        elves.push(Elf{ pos: (x, y), dir: Dir::Down, int: Int::Left });
                        grid_line.push('|');
                    }
                    '^' => {
                        elves.push(Elf{ pos: (x, y), dir: Dir::Up, int: Int::Left });
                        grid_line.push('|');
                    }
                    '<' => {
                        elves.push(Elf{ pos: (x, y), dir: Dir::Left, int: Int::Left });
                        grid_line.push('-');
                    }
                    '>' => {
                        elves.push(Elf{ pos: (x, y), dir: Dir::Right, int: Int::Left });
                        grid_line.push('-');
                    }
                    c => grid_line.push(c),
                }
            }
            grid.push(grid_line);
        }

        Game { grid, elves, first_crash: None }
    }

    fn step(&mut self) {
        let width = self.grid[0].len();
        self.elves.sort_by_key(|e| (e.pos.1 * width) + e.pos.0);
        let mut crashes = HashSet::new();
        let mut positions = HashSet::new();

        for e in self.elves.iter_mut() {
            if positions.contains(&e.pos) {
                if self.first_crash.is_none() {
                    self.first_crash = Some(e.pos);
                }
                crashes.insert(e.pos);
                continue;
            }

            match (self.grid[e.pos.1][e.pos.0], e.dir, e.int) {
                ('|', Dir::Up, _) => e.pos = (e.pos.0, e.pos.1-1),
                ('|', Dir::Down, _) => e.pos = (e.pos.0, e.pos.1+1),
                ('-', Dir::Left, _) => e.pos = (e.pos.0-1, e.pos.1),
                ('-', Dir::Right, _) => e.pos = (e.pos.0+1, e.pos.1),

                ('/', Dir::Up, _) => { e.dir = Dir::Right; e.pos = (e.pos.0+1, e.pos.1); }
                ('/', Dir::Down, _) => { e.dir = Dir::Left; e.pos = (e.pos.0-1, e.pos.1); }
                ('/', Dir::Left, _) => { e.dir = Dir::Down; e.pos = (e.pos.0, e.pos.1+1); }
                ('/', Dir::Right, _) => { e.dir = Dir::Up; e.pos = (e.pos.0, e.pos.1-1); }

                ('\\', Dir::Up, _) => { e.dir = Dir::Left; e.pos = (e.pos.0-1, e.pos.1); }
                ('\\', Dir::Down, _) => { e.dir = Dir::Right; e.pos = (e.pos.0+1, e.pos.1); }
                ('\\', Dir::Left, _) => { e.dir = Dir::Up; e.pos = (e.pos.0, e.pos.1-1); }
                ('\\', Dir::Right, _) => { e.dir = Dir::Down; e.pos = (e.pos.0, e.pos.1+1); }

                ('+', Dir::Up, Int::Left) => {
                    e.dir = Dir::Left;
                    e.pos = (e.pos.0-1, e.pos.1);
                    e.int = Int::Straight;
                }
                ('+', Dir::Down, Int::Left) => {
                    e.dir = Dir::Right;
                    e.pos = (e.pos.0+1, e.pos.1);
                    e.int = Int::Straight;
                }
                ('+', Dir::Left, Int::Left) => {
                    e.dir = Dir::Down;
                    e.pos = (e.pos.0, e.pos.1+1);
                    e.int = Int::Straight;
                }
                ('+', Dir::Right, Int::Left) => {
                    e.dir = Dir::Up;
                    e.pos = (e.pos.0, e.pos.1-1);
                    e.int = Int::Straight;
                }
                ('+', Dir::Up, Int::Straight) => {
                    e.pos = (e.pos.0, e.pos.1-1);
                    e.int = Int::Right;
                }
                ('+', Dir::Down, Int::Straight) => {
                    e.pos = (e.pos.0, e.pos.1+1);
                    e.int = Int::Right;
                }
                ('+', Dir::Left, Int::Straight) => {
                    e.pos = (e.pos.0-1, e.pos.1);
                    e.int = Int::Right;
                }
                ('+', Dir::Right, Int::Straight) => {
                    e.pos = (e.pos.0+1, e.pos.1);
                    e.int = Int::Right;
                }
                ('+', Dir::Up, Int::Right) => {
                    e.dir = Dir::Right;
                    e.pos = (e.pos.0+1, e.pos.1);
                    e.int = Int::Left;
                }
                ('+', Dir::Down, Int::Right) => {
                    e.dir = Dir::Left;
                    e.pos = (e.pos.0-1, e.pos.1);
                    e.int = Int::Left;
                }
                ('+', Dir::Left, Int::Right) => {
                    e.dir = Dir::Up;
                    e.pos = (e.pos.0, e.pos.1-1);
                    e.int = Int::Left;
                }
                ('+', Dir::Right, Int::Right) => {
                    e.dir = Dir::Down;
                    e.pos = (e.pos.0, e.pos.1+1);
                    e.int = Int::Left;
                }

                _ => panic!("Unhandled state ({} {:?} {:?})", self.grid[e.pos.1][e.pos.0], e.dir, e.int),
            }

            if positions.contains(&e.pos) {
                if self.first_crash.is_none() {
                    self.first_crash = Some(e.pos);
                }
                crashes.insert(e.pos);
            }
            positions.insert(e.pos);
        }

        self.elves.retain(|e| !crashes.contains(&e.pos));
    }
}

fn main() {
    let mut game = Game::parse("input/day13.in");

    while game.elves.len() > 1 {
        game.step();
    }

    println!("part1: {:?}", game.first_crash.unwrap());
    println!("part2: {:?}", game.elves[0].pos);
}
