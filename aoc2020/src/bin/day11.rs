use std::fs::File;
use std::io::{BufRead, BufReader};

const DELTAS: [(i32, i32); 8]  = [(-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)];

#[derive(PartialEq, Eq, Copy, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

struct Floor {
    original: Vec<Vec<State>>,
    seats: Vec<Vec<State>>,
    width: usize,
    height: usize,
}

impl Floor {
    fn new() -> Self {
        Floor {
            original: Vec::new(),
            seats: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    fn push_row(&mut self, line: &str) {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '.' => row.push(State::Floor),
                'L' => row.push(State::Empty),
                _ => unreachable!(),
            }
        }
        self.width = row.len();
        self.height += 1;
        self.original.push(row.clone());
        self.seats.push(row);
    }

    fn as_str(&self) -> String {
        let mut ret = String::new();
        for row in self.seats.iter() {
            for s in row.iter() {
                match s {
                    State::Floor => ret.push('.'),
                    State::Empty => ret.push('L'),
                    State::Occupied => ret.push('#'),
                }
            }
        }
        ret
    }

    fn step1(&mut self) {
        let mut next = Vec::new();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                let occ = self.get_neighbours(x, y);
                row.push(match self.seats[y][x] {
                    State::Floor => State::Floor,
                    State::Empty => if occ == 0 { State::Occupied } else { State::Empty },
                    State::Occupied => if occ >= 4 { State::Empty } else { State::Occupied },
                })
            }
            next.push(row);
        }
        self.seats = next;
    }

    fn step2(&mut self) {
        let mut next = Vec::new();
        for y in 0..self.height {
            let mut row = Vec::new();
            for x in 0..self.width {
                let occ = self.get_visible(x, y);
                row.push(match self.seats[y][x] {
                    State::Floor => State::Floor,
                    State::Empty => if occ == 0 { State::Occupied } else { State::Empty },
                    State::Occupied => if occ >= 5 { State::Empty } else { State::Occupied },
                })
            }
            next.push(row);
        }
        self.seats = next;
    }

    fn get_neighbours(&self, x: usize, y: usize) -> usize {
        let mut ret = 0;
        for (dx, dy) in DELTAS.iter() {
            let dx = (x as i32) + dx;
            let dy = (y as i32) + dy;
            if self.get(dx, dy) == State::Occupied {
                ret += 1;
            }
        }
        ret
    }

    fn get_visible(&self, x: usize, y: usize) -> usize {
        let mut ret = 0;
        for &(dx, dy) in DELTAS.iter() {
            if self.cast_ray(x as i32, y as i32, dx, dy) == State::Occupied {
                ret += 1;
            }
        }
        ret
    }

    fn cast_ray(&self, x: i32, y: i32, dx: i32, dy: i32) -> State {
        let mut x = x;
        let mut y = y;
        loop {
            x += dx;
            y += dy;
            if x < 0 || y < 0 {
                return State::Floor;
            } else if y as usize >= self.seats.len() {
                return State::Floor;
            } else if x as usize >= self.seats[y as usize].len() {
                return State::Floor;
            } else {
                let s = self.seats[y as usize][x as usize];
                if s != State::Floor {
                    return s;
                }
            }    
        }
    }

    fn get(&self, x: i32, y: i32) -> State {
        if x < 0 || y < 0 {
            State::Floor
        } else if y as usize >= self.seats.len() {
            State::Floor
        } else if x as usize >= self.seats[y as usize].len() {
            State::Floor
        } else {
            self.seats[y as usize][x as usize]
        }
    }

    fn occupied(&self) -> usize {
        let mut ret = 0;
        for row in self.seats.iter() {
            for &s in row.iter() {
                if s == State::Occupied {
                    ret += 1;
                }
            }
        }
        ret
    }

    fn reset(&mut self) {
        self.seats = self.original.clone();
    }
}

fn main() {
    let file = File::open("input/day11.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut floor = Floor::new();
    for line in file.lines() {
        let line = line.unwrap();
        floor.push_row(&line);
    }

    let mut prev = floor.as_str();
    loop {
        floor.step1();
        let next = floor.as_str();
        if next == prev {
            break;
        }
        prev = next;
    }
    let part1 = floor.occupied();

    floor.reset();
    
    prev = floor.as_str();
    loop {
        floor.step2();
        let next = floor.as_str();
        if next == prev {
            break;
        }
        prev = next;
    }
    let part2 = floor.occupied();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}