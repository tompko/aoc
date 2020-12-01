use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use itertools::Itertools;

extern crate itertools;

fn main() {
    let file = File::open("input/day04.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let (part1, part2) = file.lines().
        map(|x| x.unwrap()).
        map(|x| solve(&x)).
        fold((0, 0), |(x, y), (m, n)| (x + m, y + n));

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn solve(passphrase: &str) -> (u32, u32) {
    let words = passphrase.split_whitespace();
    let mut part1 = HashSet::new();
    let mut part2 = HashSet::new();

    let (a, b) = words.
        map(|x| (x, x.chars().sorted().iter().collect::<String>())).
        fold((true, true), |(x, y), (r, s)| (x && part1.insert(r), y && part2.insert(s)));

    (a as u32, b as u32)
}
