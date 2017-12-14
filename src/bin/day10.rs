extern crate aoc2017;

use std::fs::File;
use std::io::Read;
use aoc2017::knot::{round, hash};

fn main() {
    let mut file = File::open("input/day10.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    println!("part 1: {}", part1(contents));
    println!("part 2: {}", hash(&contents));
}

fn part1(contents: &str) -> usize {
    let mut input = (0..256).collect();
    let part1_lengths = contents.split(",").
        map(|x| x.parse().unwrap()).
        collect::<Vec<_>>();

    round(&mut input, 0, 0, &part1_lengths);

    input[0] * input[1]
}

