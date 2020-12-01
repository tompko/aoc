extern crate pcre;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use pcre::Pcre;

enum Instruction {
    Hlf{reg: usize},
    Tpl{reg: usize},
    Inc{reg: usize},
    Jmp{offset: i32},
    Jie{reg: usize, offset: i32},
    Jio{reg: usize, offset: i32},
}

fn reg_offset(reg: &str) -> usize {
    match reg {
        "a" => 0,
        "b" => 1,
        _ => unreachable!(),
    }
}

fn parse(line: &str) -> Instruction {
    let mut re = Pcre::compile(r"(...) ([^,]+),? ?([+\-]\d+)?").unwrap();

    let m = re.exec(line).expect("Failed to parse line");

    match m.group(1) {
        "hlf" => Instruction::Hlf{reg: reg_offset(m.group(2))},
        "tpl" => Instruction::Tpl{reg: reg_offset(m.group(2))},
        "inc" => Instruction::Inc{reg: reg_offset(m.group(2))},
        "jmp" => Instruction::Jmp{offset: m.group(2).parse::<i32>().unwrap()},
        "jie" => Instruction::Jie{reg: reg_offset(m.group(2)), offset: m.group(3).parse::<i32>().unwrap()},
        "jio" => Instruction::Jio{reg: reg_offset(m.group(2)), offset: m.group(3).parse::<i32>().unwrap()},
        _ => unreachable!(),
    }
}

fn simulate(a_start: u32, instr: &Vec<Instruction>) -> (u32, u32) {
    let mut registers = vec![a_start, 0];
    let mut pc: i32 = 0;

    while 0 <= pc && (pc as usize) < instr.len() {
        match instr[pc as usize] {
            Instruction::Hlf{reg: r} => registers[r] = registers[r] / 2,
            Instruction::Tpl{reg: r} => registers[r] = registers[r] * 3,
            Instruction::Inc{reg: r} => registers[r] = registers[r] + 1,
            Instruction::Jmp{offset: o} => pc = pc + o - 1,
            Instruction::Jie{reg: r, offset: o} => {
                if registers[r] % 2 == 0 {
                    pc = pc + o - 1;
                }
            }
            Instruction::Jio{reg: r, offset: o} => {
                if registers[r] == 1 {
                    pc = pc + o - 1;
                }
            }
        }
        pc += 1;
    }

    (registers[0], registers[1])
}

fn main() {
    let f = File::open("day23.in")
        .ok()
        .expect("Failed to open input");
    let file = BufReader::new(&f);

    let mut instructions: Vec<_> = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        instructions.push(parse(&line));
    }

    let (_, b) = simulate(0, &instructions);
    println!("{}", b);

    let (_, b) = simulate(1, &instructions);
    println!("{}", b);
}
