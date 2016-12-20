#[macro_use] extern crate lazy_static;
extern crate regex;

use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use regex::Regex;

#[derive(Copy, Clone)]
enum Operand {
    Register(usize),
    Constant(i32),
}

#[derive(Copy, Clone)]
enum Instruction {
    Cpy(Operand, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Operand, i32),

}

struct Cpu {
    registers: Vec<i32>,
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(string: &str) ->Result<Self, &'static str> {
        match string {
            "a" => Ok(Operand::Register(0)),
            "b" => Ok(Operand::Register(1)),
            "c" => Ok(Operand::Register(2)),
            "d" => Ok(Operand::Register(3)),
            x => {
                let y = x.parse().unwrap();
                Ok(Operand::Constant(y))
            },
        }
    }
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(string: &str) ->Result<Instruction, &'static str> {
        lazy_static!(
            static ref CPY: Regex = Regex::new(r"cpy (a|b|c|d|\d+) (a|b|c|d)").unwrap();
            static ref INC: Regex = Regex::new(r"inc (a|b|c|d)").unwrap();
            static ref DEC: Regex = Regex::new(r"dec (a|b|c|d)").unwrap();
            static ref JNZ: Regex = Regex::new(r"jnz (a|b|c|d|\d+) (-?\d+)").unwrap();
        );

        if let Some(caps) = CPY.captures(string) {
            let source = caps.at(1).unwrap().parse().unwrap();
            let dest = caps.at(2).unwrap().parse().unwrap();

            if let Operand::Register(x) = dest {
                return Ok(Instruction::Cpy(source, x));
            } else {
                return Err("Cpy dest not a register");
            }
        }
        if let Some(caps) = INC.captures(string) {
            let reg = caps.at(1).unwrap().parse().unwrap();

            if let Operand::Register(x) = reg {
                return Ok(Instruction::Inc(x));
            } else {
                return Err("Inc dest not a register");
            }
        }
        if let Some(caps) = DEC.captures(string) {
            let reg = caps.at(1).unwrap().parse().unwrap();

            if let Operand::Register(x) = reg {
                return Ok(Instruction::Dec(x));
            } else {
                return Err("Dec dest not a register");
            }
        }
        if let Some(caps) = JNZ.captures(string) {
            let operand = caps.at(1).unwrap().parse().unwrap();
            let offset = caps.at(2).unwrap().parse().unwrap();

            return Ok(Instruction::Jnz(operand, offset));
        }


        Err("Unrecognized instruction")
    }
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            registers: vec![0; 4],
        }
    }

    fn reset(&mut self) {
        for x in 0..4 {
            self.registers[x] = 0;
        }
    }

    fn run(&mut self, program: &[Instruction]) {
        let mut pc = 0;

        while pc < program.len() {
            let instr = program[pc];
            let mut new_pc = pc + 1;

            match instr {
                Instruction::Cpy(a, b) => {
                    let source = self.get(a);
                    self.cpy(b, source);
                },
                Instruction::Inc(a) => {
                    self.inc(a);
                },
                Instruction::Dec(a) => {
                    self.dec(a);
                },
                Instruction::Jnz(a, o) => {
                    let jmp = self.get(a);
                    if jmp != 0 {
                        new_pc = ((pc as i32) + o) as usize;
                    }
                }
            }

            pc = new_pc;
        }
    }

    fn get(&self, oper: Operand) -> i32 {
        match oper {
            Operand::Register(x) => self.registers[x],
            Operand::Constant(x) => x,
        }
    }

    fn cpy(&mut self, reg: usize, num: i32) {
        self.registers[reg] = num;
    }

    fn inc(&mut self, reg: usize) {
        self.registers[reg] += 1;
    }

    fn dec(&mut self, reg: usize) {
        self.registers[reg] -= 1;
    }
}

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
