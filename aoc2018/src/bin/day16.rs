extern crate regex;
extern crate aoc2018;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use aoc2018::{Device, TestCase};

fn main() {
    let test_cases = parse_test_cases();
    let mut device = Device::new();
    let mut part1 = 0;

    for t in test_cases.iter() {
        let matches = device.test_case(t);

        if matches >= 3 {
            part1 += 1;
        }
    }

    device.reset();
    let prog = parse_program();

    for p in prog {
        device.exec(p[0], p[1], p[2], p[3]);
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", device.registers[0]);
}

fn parse_test_cases() -> Vec<TestCase> {
    let file = File::open("input/day16a.in").expect("Failed to open input");
    let file = BufReader::new(&file);
    let mut lines = file.lines();

    let before_re = Regex::new(r"Before: \[(\d+), (\d+), (\d+), (\d+)]").unwrap();
    let opcode_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();
    let after_re = Regex::new(r"After:  \[(\d+), (\d+), (\d+), (\d+)]").unwrap();

    let mut test_cases = Vec::new();

    loop {
        let line = lines.next();
        let line = line.unwrap().unwrap();
        let bcap = before_re.captures(&line).unwrap();

        let line = lines.next();
        let line = line.unwrap().unwrap();
        let ocap = opcode_re.captures(&line).unwrap();

        let line = lines.next();
        let line = line.unwrap().unwrap();
        let acap = after_re.captures(&line).unwrap();

        test_cases.push(TestCase{
            before: [
                bcap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                bcap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                bcap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                bcap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            ],
            opcode: [
                ocap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                ocap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ocap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                ocap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            ],
            after: [
                acap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                acap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                acap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                acap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
            ],
        });

        let line = lines.next();
        if line.is_none() {
            break;
        }
    }

    test_cases
}

fn parse_program() -> Vec<[i64; 4]> {
    let file = File::open("input/day16b.in").expect("Failed to open input");
    let file = BufReader::new(&file);
    let lines = file.lines();

    let opcode_re = Regex::new(r"(\d+) (\d+) (\d+) (\d+)").unwrap();

    let mut program = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let ocap = opcode_re.captures(&line).unwrap();

        program.push([
            ocap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            ocap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
            ocap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            ocap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        ]);
    }

    program
}
