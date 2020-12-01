#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate permutohedron;

use std::collections::VecDeque;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use regex::Regex;
use permutohedron::Heap;

const PART1: &'static str = "abcdefgh";
const PART2: &'static str = "fbgdceah";

#[derive(Copy, Clone)]
enum Instruction {
    Swap(usize, usize),
    Swapc(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    Rotatec(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(string: &str) ->Result<Instruction, &'static str> {
        lazy_static!(
            static ref SWAP: Regex = Regex::new(r"swap position (\d) with position (\d)").unwrap();
            static ref SWAPC: Regex = Regex::new(r"swap letter (\w) with letter (\w)").unwrap();
            static ref ROTR: Regex = Regex::new(r"rotate right (\d) steps?").unwrap();
            static ref ROTL: Regex = Regex::new(r"rotate left (\d) steps?").unwrap();
            static ref ROTC: Regex = Regex::new(r"rotate based on position of letter (\w)").unwrap();
            static ref MOV: Regex = Regex::new(r"move position (\d) to position (\d)").unwrap();
            static ref REV: Regex = Regex::new(r"reverse positions (\d) through (\d)").unwrap();
        );

        if let Some(caps) = SWAP.captures(string) {
            let a = caps.at(1).unwrap().parse().unwrap();
            let b = caps.at(2).unwrap().parse().unwrap();

            return Ok(Instruction::Swap(a, b));
        }
        if let Some(caps) = SWAPC.captures(string) {
            let a = caps.at(1).unwrap().chars().nth(0).unwrap();
            let b = caps.at(2).unwrap().chars().nth(0).unwrap();

            return Ok(Instruction::Swapc(a, b));
        }
        if let Some(caps) = ROTR.captures(string) {
            let steps = caps.at(1).unwrap().parse().unwrap();

            return Ok(Instruction::RotateRight(steps));
        }
        if let Some(caps) = ROTL.captures(string) {
            let steps = caps.at(1).unwrap().parse().unwrap();

            return Ok(Instruction::RotateLeft(steps));
        }
        if let Some(caps) = ROTC.captures(string) {
            let ch = caps.at(1).unwrap();

            return Ok(Instruction::Rotatec(ch.chars().nth(0).unwrap()));
        }
        if let Some(caps) = REV.captures(string) {
            let a = caps.at(1).unwrap().parse().unwrap();
            let b = caps.at(2).unwrap().parse().unwrap();

            return Ok(Instruction::Reverse(a, b));
        }
        if let Some(caps) = MOV.captures(string) {
            let a = caps.at(1).unwrap().parse().unwrap();
            let b = caps.at(2).unwrap().parse().unwrap();

            return Ok(Instruction::Move(a, b));
        }

        panic!("Unrecognized instruction: {}", string);
    }
}

fn parse(reader: &mut BufRead) -> Vec<Instruction> {
    let mut ret = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        ret.push(line.trim().parse().unwrap());
    }

    ret
}

fn apply(input: &str, instr: &[Instruction]) -> String {
    let mut input: VecDeque<_> = input.chars().collect();

    for inst in instr.iter() {
        match *inst {
            Instruction::Swap(a, b) => { input.swap(a, b) },
            Instruction::Swapc(a, b) => {
                let a = input.iter().position(|x| *x == a).unwrap();
                let b = input.iter().position(|x| *x == b).unwrap();

                input.swap(a, b);
            },
            Instruction::RotateLeft(a) => {
                for _ in 0..a {
                    let tmp = input.pop_front().unwrap();
                    input.push_back(tmp);
                }
            }
            Instruction::RotateRight(a) => {
                for _ in 0..a {
                    let tmp = input.pop_back().unwrap();
                    input.push_front(tmp);
                }
            }
            Instruction::Rotatec(a) => {
                let mut a = input.iter().position(|x| *x == a).unwrap();
                if a >= 4 {
                    a += 1;
                }
                a += 1;

                for _ in 0..a {
                    let tmp = input.pop_back().unwrap();
                    input.push_front(tmp);
                }
            }
            Instruction::Reverse(a, b) => {
                input = input.iter().take(a).chain(
                    input.iter().skip(a).take(b - a + 1).collect::<Vec<_>>().into_iter().rev()
                ).chain(
                    input.iter().skip(b + 1)
                ).cloned().collect();
            }
            Instruction::Move(a, b) => {
                let tmp = input.remove(a).unwrap();
                input.insert(b, tmp);
            }
        }
    }

    input.into_iter().collect()
}

fn main() {
    let file = File::open("input/day21.in").expect("Failed to open input");
    let mut reader = BufReader::new(&file);

    let instructions = parse(&mut reader);

    let part1 = apply(PART1, &instructions);
    println!("1: {}", part1);

    let mut part2: Vec<char> = PART2.chars().collect();
    let heap = Heap::new(&mut part2);

    for data in heap {
        let d: String = data.into_iter().collect();
        if PART2 == apply(&d, &instructions) {
            println!("2: {}", d);
        }
    }
}
