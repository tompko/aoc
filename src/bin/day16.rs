extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

struct Device {
    registers: [i64; 4]
}

#[derive(Debug, Clone, Copy)]
struct TestCase {
    before: [i64; 4],
    opcode: [i64; 4],
    after: [i64; 4],
}

impl Device {
    fn new() -> Self {
        Device { registers: [0; 4] }
    }

    fn reset(&mut self) {
        self.registers = [0,0,0,0];
    }

    fn test_case(&mut self, t: &TestCase) -> i64 {
        let mut matches = 0;

        self.registers = t.before;
        self.addr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.addi(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.mulr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.muli(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.banr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.bani(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.borr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.bori(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.setr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.seti(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.gtir(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.gtri(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.gtrr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.eqir(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.eqri(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        self.registers = t.before;
        self.eqrr(t.opcode[1], t.opcode[2], t.opcode[3]);
        matches += if self.registers == t.after { 1 } else { 0 };

        matches
    }

    fn exec(&mut self, o: i64, a: i64, b: i64, c: i64) {
        match o {
            0  => self.banr(a, b, c),
            1  => self.eqrr(a, b, c),
            2  => self.setr(a, b, c),
            3  => self.eqir(a, b, c),
            4  => self.bori(a, b, c),
            5  => self.muli(a, b, c),
            6  => self.bani(a, b, c),
            7  => self.borr(a, b, c),
            8  => self.gtir(a, b, c),
            9  => self.gtrr(a, b, c),
            10 => self.addi(a, b, c),
            11 => self.gtri(a, b, c),
            12 => self.eqri(a, b, c),
            13 => self.addr(a, b, c),
            14 => self.mulr(a, b, c),
            15 => self.seti(a, b, c),
            _ => panic!("Unrecognized opcode {}", o),
        }
    }

    fn addr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va + vb;
    }

    fn addi(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va + vb;
    }

    fn mulr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va * vb;
    }

    fn muli(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va * vb;
    }

    fn banr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va & vb;
    }

    fn bani(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va & vb;
    }

    fn borr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = va | vb;
    }

    fn bori(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = va | vb;
    }

    fn setr(&mut self, a: i64, _b: i64, c: i64) {
        let va = self.registers[a as usize];
        self.registers[c as usize] = va;
    }

    fn seti(&mut self, a: i64, _b: i64, c: i64) {
        let va = a;
        self.registers[c as usize] = va;
    }

    fn gtir(&mut self, a: i64, b: i64, c: i64) {
        let va = a;
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va > vb { 1 } else { 0 };
    }

    fn gtri(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = if va > vb { 1 } else { 0 };
    }

    fn gtrr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va > vb { 1 } else { 0 };
    }

    fn eqir(&mut self, a: i64, b: i64, c: i64) {
        let va = a;
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va == vb { 1 } else { 0 };
    }

    fn eqri(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = b;
        self.registers[c as usize] = if va == vb { 1 } else { 0 };
    }

    fn eqrr(&mut self, a: i64, b: i64, c: i64) {
        let va = self.registers[a as usize];
        let vb = self.registers[b as usize];
        self.registers[c as usize] = if va == vb { 1 } else { 0 };
    }
}

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
