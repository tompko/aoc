extern crate aoc2016;

use std::io::{BufReader, BufRead};
use std::fs::File;
use aoc2016::lightscreen::Screen;

fn main() {
    let file = File::open("input/day08.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut screen = Screen::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();

        screen.execute(line);
    }

    println!("1: {}", screen.count_on());
    screen.print();
}
