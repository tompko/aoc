use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
pub enum Operand {
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

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Cpy(Operand, Operand),
    Jnz(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Tgl(Operand),
    Out(Operand),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::Cpy(a, b) => write!(f, "cpy {} {}", a, b),
            Instruction::Jnz(a, b) => write!(f, "jnz {} {}", a, b),
            Instruction::Inc(a) => write!(f, "inc {}", a),
            Instruction::Dec(a) => write!(f, "dec {}", a),
            Instruction::Tgl(a) => write!(f, "tgl {}", a),
            Instruction::Out(a) => write!(f, "out {}", a),
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
            static ref OUT: Regex = Regex::new(r"out (a|b|c|d|\d+)").unwrap();
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
        if let Some(caps) = OUT.captures(string) {
            let oper = caps.at(1).unwrap().parse().unwrap();

            return Ok(Instruction::Out(oper));
        }

        Err("Unrecognized instruction")
    }
}

pub struct Cpu {
    pub registers: Vec<i32>,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: vec![0; 4],
        }
    }

    pub fn reset(&mut self) {
        for x in 0..4 {
            self.registers[x] = 0;
        }
    }

    pub fn run(&mut self, program: &[Instruction]) -> Vec<i32> {
        let mut pc = 0;
        let mut program: Vec<Instruction> = program.iter().cloned().collect();
        let mut output: Vec<_> = Vec::new();
        let mut seen = HashSet::new();

        while pc < program.len() {
            let state = (pc, self.registers[0], self.registers[1], self.registers[2], self.registers[3]);
            if seen.contains(&state) {
                break;
            }

            seen.insert(state);

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
                            Instruction::Dec(oper) | Instruction::Tgl(oper) | Instruction::Out(oper) => Instruction::Inc(oper),
                        };
                        program[index] = new_instr;
                    }
                },
                Instruction::Out(o) => {
                    let out = self.get(o);
                    output.push(out);
                }
            }

            pc = new_pc;
        }

        output
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

