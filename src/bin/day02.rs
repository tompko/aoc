use std::fs::File;
use std::io::Read;

fn run(program: &mut Vec<i32>) {
    let mut index = 0;
    loop {
        match program[index] {
            1 => {
                let a = program[index+1] as usize;
                let b = program[index+2] as usize;
                let r = program[index+3] as usize;
                program[r] = program[a] + program[b];
                index += 4;
            }
            2 => {
                let a = program[index+1] as usize;
                let b = program[index+2] as usize;
                let r = program[index+3] as usize;
                program[r] = program[a] * program[b];
                index += 4;
            }
            99 => {
                return;
            }
            _ => unimplemented!(),
        }
    }

}

fn main() {
    let mut file = File::open("input/day02.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let mut program: Vec<i32> = contents.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

    program[1] = 12;
    program[2] = 2;

    run(&mut program);

    println!("Part 1: {}", program[0]);
}
