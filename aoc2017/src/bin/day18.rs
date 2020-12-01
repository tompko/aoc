extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use std::string::ParseError;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Target {
    Constant(i64),
    Register(char),
}

fn as_target(input: &str) -> Target {
    let first = input.chars().nth(0).unwrap();

    if first.is_alphabetic() {
        Target::Register(first)
    } else {
        Target::Constant(input.parse::<i64>().unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    SND(Target),
    SET(Target, Target),
    ADD(Target, Target),
    MUL(Target, Target),
    MOD(Target, Target),
    RCV(Target),
    JGZ(Target, Target),
}

impl FromStr for Instruction {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err>  {
        let re = Regex::new(r"(...)\s([-a-z0-9]+)\s?([-a-z0-9]+)?").unwrap();

        let caps = re.captures(s).unwrap();

        let reg = as_target(&caps[2]);

        let i = match &caps[1] {
            "snd" => Instruction::SND(reg),
            "set" => Instruction::SET(reg, as_target(&caps[3])),
            "add" => Instruction::ADD(reg, as_target(&caps[3])),
            "mul" => Instruction::MUL(reg, as_target(&caps[3])),
            "mod" => Instruction::MOD(reg, as_target(&caps[3])),
            "rcv" => Instruction::RCV(reg),
            "jgz" => Instruction::JGZ(reg, as_target(&caps[3])),
            _ => unreachable!(),
        };

        Ok(i)
    }
}

struct VM {
    registers: HashMap<char, i64>,
    pc: i64,
    instrs: Vec<Instruction>,

    send_buffer: VecDeque<i64>,
    recv_buffer: VecDeque<i64>,

    reading: bool,
    finished: bool,
    first_recv: Option<i64>,

    num_sends: u64,
}

impl VM {
    fn new(id: i64, instrs: Vec<Instruction>) -> VM {
        let mut registers: HashMap<_,_> = "abcdefghijklmnopqrstuvwxyz".chars().map(|c| (c, 0)).collect();
        registers.insert('p', id);
        VM {
            registers: registers,
            pc: 0,
            instrs: instrs,

            send_buffer: VecDeque::new(),
            recv_buffer: VecDeque::new(),

            reading: false,
            finished: false,
            first_recv: None,

            num_sends: 0,
        }
    }

    fn step(&mut self) {
        let mut next_pc = self.pc + 1;
        match self.instrs[self.pc as usize] {
            Instruction::SND(a) => {
                let snd = self.deref(a);
                self.send_buffer.push_back(snd);
                self.num_sends += 1;
            },
            Instruction::SET(a, b) => {
                if let Target::Register(c) = a {
                    let s = self.deref(b);
                    self.registers.insert(c, s);
                } else {
                    unreachable!();
                }
            },
            Instruction::ADD(a, b) => {
                if let Target::Register(c) = a {
                    let s = self.registers[&c];
                    let t = self.deref(b);
                    self.registers.insert(c, t + s);
                } else {
                    unreachable!();
                }
            },
            Instruction::MUL(a, b) => {
                if let Target::Register(c) = a {
                    let s = self.registers[&c];
                    let t = self.deref(b);
                    self.registers.insert(c, t * s);
                } else {
                    unreachable!();
                }
            },
            Instruction::MOD(a, b) => {
                if let Target::Register(c) = a {
                    let s = self.registers[&c];
                    let t = self.deref(b);
                    self.registers.insert(c, s % t);
                } else {
                    unreachable!();
                }
            },
            Instruction::RCV(a) => {
                if self.first_recv.is_none() {
                    if self.send_buffer.len() > 0 {
                        self.first_recv = Some(*self.send_buffer.iter().last().unwrap());
                    }
                }

                if self.recv_buffer.len() > 0 {
                    let r = self.recv_buffer.pop_front().unwrap();
                    if let Target::Register(c) = a {
                        self.registers.insert(c, r);
                    } else {
                        unreachable!();
                    }
                } else {
                    self.reading = true;
                    next_pc = self.pc;
                }
            },
            Instruction::JGZ(a, b) => {
                if self.deref(a) > 0 {
                    next_pc = self.pc + self.deref(b);
                }
            },
        }

        self.pc = next_pc;
    }

    fn deref(&self, a: Target) -> i64 {
        match a {
            Target::Constant(x) => x,
            Target::Register(x) => self.registers[&x],
        }
    }

}

fn main() {
    let file = File::open("input/day18.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut program = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();

        program.push(line.parse::<Instruction>().unwrap());
    }

    let mut part1_vm = VM::new(0, program.clone());
    while part1_vm.first_recv.is_none() {
        part1_vm.step();
    }

    let mut part2_vm0 = VM::new(0, program.clone());
    let mut part2_vm1 = VM::new(1, program.clone());

    loop {
        if (part2_vm0.finished && part2_vm1.finished) ||
            (part2_vm0.reading && part2_vm1.reading) {
                break
            }

        while part2_vm0.send_buffer.len() > 0 {
            let s = part2_vm0.send_buffer.pop_front().unwrap();
            part2_vm1.recv_buffer.push_back(s);
        }
        while part2_vm1.send_buffer.len() > 0 {
            let s = part2_vm1.send_buffer.pop_front().unwrap();
            part2_vm0.recv_buffer.push_back(s);
        }

        part2_vm0.step();
        part2_vm1.step();
    }

    println!("part 1: {:?}", part1_vm.first_recv.unwrap());
    println!("part 2: {}", part2_vm1.num_sends);
}
