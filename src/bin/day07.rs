use std::fs::File;
use std::io::Read;
use std::cmp::max;

struct Computer {
    program: Vec<i32>,
    ip: usize,
    input: Vec<i32>,
    output: Vec<i32>,
    halted: bool,
}

impl Computer {
    fn new(program: &Vec<i32>) -> Self {
        Computer{
            program: program.clone(),
            ip: 0,
            input: Vec::new(),
            output: Vec::new(),
            halted: false,
        }
    }

    fn push(&mut self, a: i32) {
        self.input.push(a);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.output.len() > 0 {
            Some(self.output.remove(0))
        } else {
            None
        }
    }

    fn has_output(&self) -> bool {
        self.output.len() > 0
    }

    fn run(&mut self) {
        while !self.halted {
            self.step()
        }
    }

    fn step(&mut self) {
        if self.halted {
            return;
        }
        let (opcode, mode1, mode2, _mode3) = Computer::decode(self.program[self.ip]);

        match opcode {
            1 => { // ADD
                let a = self.get(self.program[self.ip+1], mode1);
                let b = self.get(self.program[self.ip+2], mode2);
                let r = self.program[self.ip+3] as usize;
                self.program[r] = a + b;
                self.ip += 4;
            }
            2 => { // MUL
                let a = self.get(self.program[self.ip+1], mode1);
                let b = self.get(self.program[self.ip+2], mode2);
                let r = self.program[self.ip+3] as usize;
                self.program[r] = a * b;
                self.ip += 4;
            }
            3 => { // IN
                let a = self.program[self.ip+1] as usize;
                self.program[a] = self.input.remove(0);
                self.ip += 2;
            }
            4 => { // OUT
                let a = self.get(self.program[self.ip+1], mode1);
                self.output.push(a);
                self.ip += 2;
            }
            5 => { // JNZ
                let a = self.get(self.program[self.ip+1], mode1);
                let r = self.get(self.program[self.ip+2], mode2) as usize;
                if a != 0 {
                    self.ip = r;
                } else {
                    self.ip += 3;
                }
            }
            6 => { // JZ
                let a = self.get(self.program[self.ip+1], mode1);
                let r = self.get(self.program[self.ip+2], mode2) as usize;
                if a == 0 {
                    self.ip = r;
                } else {
                    self.ip += 3;
                }
            }
            7 => { // LT
                let a = self.get(self.program[self.ip+1], mode1);
                let b = self.get(self.program[self.ip+2], mode2);
                let r = self.program[self.ip+3] as usize;
                let res = if a < b { 1 } else { 0 };
                self.program[r] = res;
                self.ip += 4;
            }
            8 => { // EQ
                let a = self.get(self.program[self.ip+1], mode1);
                let b = self.get(self.program[self.ip+2], mode2);
                let r = self.program[self.ip+3] as usize;
                let res = if a == b { 1 } else { 0 };
                self.program[r] = res;
                self.ip += 4;
            }
            99 => {
                self.halted = true;
            }
            _ => panic!("Unrecognized opcode {}", opcode),
        }
    }

    fn get(&self, addr: i32, mode: i32) -> i32 {
        match mode {
            0 => self.program[addr as usize],
            1 => addr,
            _ => unreachable!(),
        }
    }    

    fn decode(instr: i32) -> (i32, i32, i32, i32) {
        let opcode = instr % 100;
        let mode1 = (instr / 100) % 10;
        let mode2 = (instr / 1000) % 10;
        let mode3 = (instr / 10000) % 10;
        (opcode, mode1, mode2, mode3)
    }    
}

fn run_params(program: &Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let mut comp_a = Computer::new(program);
    let mut comp_b = Computer::new(program);
    let mut comp_c = Computer::new(program);
    let mut comp_d = Computer::new(program);
    let mut comp_e = Computer::new(program);

    comp_a.push(a);
    comp_b.push(b);
    comp_c.push(c);
    comp_d.push(d);
    comp_e.push(e);

    comp_a.push(0);

    let mut last_e = 0;

    loop {
        while !comp_a.halted && !comp_a.has_output() {
            comp_a.step();
        }
        if comp_a.halted { return last_e };
        let a_out = comp_a.pop().unwrap();
        comp_b.push(a_out);
        while !comp_b.halted && !comp_b.has_output() {
            comp_b.step();
        }
        if comp_b.halted { return last_e };
        let b_out = comp_b.pop().unwrap();
        comp_c.push(b_out);
        while !comp_c.halted && !comp_c.has_output() {
            comp_c.step();
        }
        if comp_c.halted { return last_e };
        let c_out = comp_c.pop().unwrap();
        comp_d.push(c_out);
        while !comp_d.halted && !comp_d.has_output() {
            comp_d.step();
        }
        if comp_d.halted { return last_e };
        let d_out = comp_d.pop().unwrap();
        comp_e.push(d_out);
        while !comp_e.halted && !comp_e.has_output() {
            comp_e.step();
        }
        if comp_e.halted { return last_e };
        let e_out = comp_e.pop().unwrap();
        comp_a.push(e_out);
        last_e = e_out;
    }
}

fn main() {
    let mut file = File::open("input/day07.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let program: Vec<i32> = contents.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for a in 0..5 {
        for b in 0..5 {
            if a == b { continue; }
            for c in 0..5 {
                if a == c || b == c { continue; }
                for d in 0..5 {
                    if a == d || b ==  d || c == d { continue; }
                    for e in 0..5 {
                        if a == e || b == e || c == e || d == e { continue; }
                        let thrust = run_params(&program, a, b, c, d, e);
                        part1 = max(part1, thrust);
                    }
                }
            }
        }
    }
    for a in 5..10 {
        for b in 5..10 {
            if a == b { continue; }
            for c in 5..10 {
                if a == c || b == c { continue; }
                for d in 5..10 {
                    if a == d || b ==  d || c == d { continue; }
                    for e in 5..10 {
                        if a == e || b == e || c == e || d == e { continue; }
                        let thrust = run_params(&program, a, b, c, d, e);
                        part2 = max(part2, thrust);
                    }
                }
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
