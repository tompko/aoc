extern crate aoc2016;

use std::io::{BufReader, BufRead};
use std::fs::File;
use aoc2016::assembunny::Cpu;
use aoc2016::lightscreen::Screen;

fn main() {
    let file = File::open("input/bonus.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        program.push(line.parse().unwrap());
    }

    let mut cpu = Cpu::new();
    let output = cpu.run(&program);
    let output = output.iter().map(|&x| std::char::from_u32(x as u32).unwrap()).collect::<String>();

    let mut screen = Screen::new();

    for line in output.split("\n") {
        screen.execute(line);
    }

    screen.print();
}
