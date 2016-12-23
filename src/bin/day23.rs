#[macro_use] extern crate lazy_static;
extern crate regex;

use std::fmt;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Operand {
    Register(usize),
    Constant(i32),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operand::Register(0) => write!(f, "a"),
            Operand::Register(1) => write!(f, "b"),
            Operand::Register(2) => write!(f, "c"),
            Operand::Register(3) => write!(f, "d"),
            Operand::Register(_) => unreachable!(),
            Operand::Constant(a) => write!(f, "{}", a),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Cpy(Operand, Operand),
    Jnz(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Tgl(Operand),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Cpy(a, b) => write!(f, "cpy {} {}", a, b),
            Instruction::Jnz(a, b) => write!(f, "jnz {} {}", a, b),
            Instruction::Inc(a) => write!(f, "inc {}", a),
            Instruction::Dec(a) => write!(f, "dec {}", a),
            Instruction::Tgl(a) => write!(f, "tgl {}", a),
        }
    }
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
            static ref CPY: Regex = Regex::new(r"cpy (a|b|c|d|-?\d+) (a|b|c|d)").unwrap();
            static ref INC: Regex = Regex::new(r"inc (a|b|c|d)").unwrap();
            static ref DEC: Regex = Regex::new(r"dec (a|b|c|d)").unwrap();
            static ref JNZ: Regex = Regex::new(r"jnz (a|b|c|d|\d+) (a|b|c|d|-?\d+)").unwrap();
            static ref TGL: Regex = Regex::new(r"tgl (a|b|c|d)").unwrap();
        );

        if let Some(caps) = CPY.captures(string) {
            let source = caps.at(1).unwrap().parse().unwrap();
            let dest = caps.at(2).unwrap().parse().unwrap();

            return Ok(Instruction::Cpy(source, dest));
        }
        if let Some(caps) = INC.captures(string) {
            let reg = caps.at(1).unwrap().parse().unwrap();

            return Ok(Instruction::Inc(reg));
        }
        if let Some(caps) = DEC.captures(string) {
            let reg = caps.at(1).unwrap().parse().unwrap();

            return Ok(Instruction::Dec(reg));
        }
        if let Some(caps) = JNZ.captures(string) {
            let operand = caps.at(1).unwrap().parse().unwrap();
            let offset = caps.at(2).unwrap().parse().unwrap();

            return Ok(Instruction::Jnz(operand, offset));
        }
        if let Some(caps) = TGL.captures(string) {
            let oper = caps.at(1).unwrap().parse().unwrap();

            return Ok(Instruction::Tgl(oper));
        }

        println!("{}", string);

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
        // let mut program: Vec<Instruction> = program.iter().map(|x| *x).collect();
        let mut program: Vec<Instruction> = program.iter().cloned().collect();

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
                    let offset = self.get(o);
                    if jmp != 0 {
                        new_pc = ((pc as i32) + offset) as usize;
                    }
                },
                Instruction::Tgl(o) => {
                    let offset = self.get(o);
                    let index = pc as i32 + offset;
                    if index >= 0 && index < program.len() as i32 {
                        let index = index as usize;

                        let instr = program[index];
                        let new_instr = match instr {
                            Instruction::Cpy(oper, offset) => Instruction::Jnz(oper, offset),
                            Instruction::Jnz(oper, offset) => Instruction::Cpy(oper, offset),
                            Instruction::Inc(oper) => Instruction::Dec(oper),
                            Instruction::Dec(oper) | Instruction::Tgl(oper) => Instruction::Inc(oper),
                        };
                        program[index] = new_instr;
                    }
                },
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

    fn cpy(&mut self, oper: Operand, num: i32) {
        match oper {
            Operand::Register(u) => self.registers[u] = num,
            Operand::Constant(_) => {}, // Invalid instructions are ignored
        }
    }

    fn inc(&mut self, oper: Operand) {
        match oper {
            Operand::Register(u) => self.registers[u] += 1,
            Operand::Constant(_) => {}, // Invalid instructions are ignored
        }
    }

    fn dec(&mut self, oper: Operand) {
        match oper {
            Operand::Register(u) => self.registers[u] -= 1,
            Operand::Constant(_) => {}, // Invalid instructions are ignored
        }
    }
}

fn main() {
    let file = File::open("input/day23.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        program.push(line.parse().unwrap());
    }

    let mut cpu = Cpu::new();
    cpu.registers[0] = 7;
    cpu.run(&program);

    println!("1: {}", cpu.registers[0]);

    cpu.reset();
    cpu.registers[0] = 12;
    cpu.run(&program);

    println!("2: {}", cpu.registers[0]);
}
