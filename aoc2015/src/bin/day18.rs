extern crate itertools;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coord {
    x: i32,
    y: i32,
}

fn step(lights: &HashSet<Coord>) -> HashSet<Coord> {
    let mut new_state: HashSet<Coord> = HashSet::new();

    for (x, y) in (0..100).cartesian_product(0..100) {
        let mut neighbours = 0;
        for (c, d) in (-1..2).cartesian_product(-1..2) {
            if c == 0 && d == 0 {
                continue;
            }
            if lights.contains(&Coord{x: x+c, y: y+d}) {
                neighbours += 1;
            }
        }

        let c = Coord{x:x, y:y};
        if lights.contains(&c) && (neighbours == 2 || neighbours == 3) {
            new_state.insert(c);
        } else if !lights.contains(&c) && neighbours == 3 {
            new_state.insert(c);
        }
    }

    new_state
}

fn main() {
    let f = File::open("day18.in")
        .ok()
        .expect("Failed to open input");
    let file = BufReader::new(&f);

    let mut input: HashSet<Coord> = HashSet::new();

    for (y, line) in file.lines().enumerate() {
        let line = line.unwrap();
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                input.insert(Coord{ x: x as i32, y: y as i32});
            }
        }
    }

    let mut state = input.clone();
    for _ in 0..100 {
        state = step(&state);
    }
    println!("{}", state.len());

    let corners: HashSet<_> = [
        Coord{x: 0, y: 0},
        Coord{x: 0, y: 99},
        Coord{x: 99, y: 0},
        Coord{x: 99, y: 99}
    ].iter().cloned().collect();
    state = input.clone().union(&corners).cloned().collect();
    for _ in 0..100 {
        state = step(&state).union(&corners).cloned().collect();
    }
    println!("{}", state.len());
}
