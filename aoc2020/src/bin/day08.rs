use std::collections::HashSet;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

enum SimResult {
    Looped(i32),
    Ended(i32),
}

struct Console {
    instrs: Vec<Instruction>,
    pc: usize,
    acc: i32,
}

impl Console {
    fn new(v: Vec<Instruction>) -> Self {
        Console {
            instrs: v,
            pc: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        let mut next_pc = self.pc + 1;
        match self.instrs[self.pc] {
            Instruction::Acc(inc) => { self.acc += inc; },
            Instruction::Jmp(inc) => { next_pc = ((self.pc as i32) + inc) as usize; }
            Instruction::Nop(_) => {},
        } 
        self.pc = next_pc;
    }
}

impl FromStr for Instruction {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\w\w\w) ([+-])(\d+)")?;
        let c = re.captures(s).unwrap();
        let mut offset: i32 = c.get(3).unwrap().as_str().parse()?;
        let sign = c.get(2).unwrap().as_str();
        if sign == "-" {
            offset = -offset;
        }

        match c.get(1).unwrap().as_str() {
            "acc" => Ok(Instruction::Acc(offset)),
            "jmp" => Ok(Instruction::Jmp(offset)),
            "nop" => Ok(Instruction::Nop(offset)),
            _ => unreachable!(),
        }
    }
}

fn simulate(instrs: Vec<Instruction>) -> SimResult {
    let length = instrs.len();
    let mut console = Console::new(instrs);
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&console.pc) {
            return SimResult::Looped(console.acc);
        }
        if console.pc == length {
            return SimResult::Ended(console.acc);
        }
        seen.insert(console.pc);
        console.step();
    }
}

fn calc_part2(instructions: Vec<Instruction>) -> i32 {
    for i in 0..instructions.len() {
        if let Instruction::Nop(offset) = instructions[i] {
            let mut new_instrs = instructions.clone();
            new_instrs[i] = Instruction::Jmp(offset);
            if let SimResult::Ended(acc) = simulate(new_instrs) {
                return acc;
            }
        }
        if let Instruction::Jmp(offset) = instructions[i] {
            let mut new_instrs = instructions.clone();
            new_instrs[i] = Instruction::Nop(offset);
            if let SimResult::Ended(acc) = simulate(new_instrs) {
                return acc;
            }
        }
    }

    unreachable!()
}

fn main() {
    let file = File::open("input/day08.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let instructions: Vec<Instruction> = file.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let part1 = match simulate(instructions.clone()) {
        SimResult::Looped(acc) => acc,
        _ => unreachable!(),
    };
    let part2 = calc_part2(instructions);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}