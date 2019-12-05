use std::fs::File;
use std::io::Read;

fn decode(instr: i32) -> (i32, i32, i32, i32) {
    let opcode = instr % 100;
    let mode1 = (instr / 100) % 10;
    let mode2 = (instr / 1000) % 10;
    let mode3 = (instr / 10000) % 10;
    (opcode, mode1, mode2, mode3)
}

fn get(program: &Vec<i32>, addr: i32, mode: i32) -> i32 {
    match mode {
        0 => program[addr as usize],
        1 => addr,
        _ => unreachable!(),
    }
}

fn run(program: &mut Vec<i32>, input: &Vec<i32>) {
    let mut index = 0;
    let mut in_index = 0;

    loop {
        let (opcode, mode1, mode2, _mode3) = decode(program[index]);

        match opcode {
            1 => { // ADD
                let a = get(program, program[index+1], mode1);
                let b = get(program, program[index+2], mode2);
                let r = program[index+3] as usize;
                program[r] = a + b;
                index += 4;
            }
            2 => { // MUL
                let a = get(program, program[index+1], mode1);
                let b = get(program, program[index+2], mode2);
                let r = program[index+3] as usize;
                program[r] = a * b;
                index += 4;
            }
            3 => { // IN
                let a = program[index+1] as usize;
                program[a] = input[in_index];
                in_index += 1;
                index += 2;
            }
            4 => { // OUT
                let a = get(program, program[index+1], mode1);
                println!("{}", a);
                index += 2;
            }
            5 => { // JNZ
                let a = get(program, program[index+1], mode1);
                let r = get(program, program[index+2], mode2) as usize;
                if a != 0 {
                    index = r;
                } else {
                    index += 3;
                }
            }
            6 => { // JZ
                let a = get(program, program[index+1], mode1);
                let r = get(program, program[index+2], mode2) as usize;
                if a == 0 {
                    index = r;
                } else {
                    index += 3;
                }
            }
            7 => { // LT
                let a = get(program, program[index+1], mode1);
                let b = get(program, program[index+2], mode2);
                let r = program[index+3] as usize;
                let res = if a < b { 1 } else { 0 };
                program[r] = res;
                index += 4;
            }
            8 => { // EQ
                let a = get(program, program[index+1], mode1);
                let b = get(program, program[index+2], mode2);
                let r = program[index+3] as usize;
                let res = if a == b { 1 } else { 0 };
                program[r] = res;
                index += 4;
            }
            99 => {
                return;
            }
            _ => panic!("Unrecognized opcode {}", opcode),
        }
    }

}

fn main() {
    let mut file = File::open("input/day05.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let mut program: Vec<i32> = contents.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Part 1: ");
    run(&mut program, &vec![1]);

    let mut program: Vec<i32> = contents.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
    println!("Part 2: ");
    run(&mut program, &vec![5]);
}
