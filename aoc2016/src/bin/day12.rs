extern crate aoc2016;

use std::io::{BufReader, BufRead};
use std::fs::File;
use aoc2016::assembunny::Cpu;

fn main() {
    let file = File::open("input/day12.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        program.push(line.parse().unwrap());
    }

    let mut cpu = Cpu::new();
    cpu.run(&program);

    println!("1: {}", cpu.registers[0]);

    cpu.reset();
    cpu.registers[2] = 1;
    cpu.run(&program);

    println!("2: {}", cpu.registers[0]);
}
