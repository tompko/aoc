#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate aoc2016;

use std::io::{BufReader, BufRead};
use std::fs::File;
use aoc2016::assembunny::Cpu;

fn main() {
    let file = File::open("input/day25.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        program.push(line.parse().unwrap());
    }

    let mut cpu = Cpu::new();
    let mut input = 0;

    loop {
        cpu.reset();
        cpu.registers[0] = input;
        let output = cpu.run(&program);

        let mut good = output.len() % 2 == 0;
        for (i, &o) in output.iter().enumerate() {
            if i % 2 == 0 {
                good = good && o == 0;
            } else {
                good = good && o == 1;
            }
        }

        if good {
            break;
        }
        input += 1;
    }

    println!("1: {:?}", input);
}
