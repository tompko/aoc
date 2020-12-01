use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day05.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let jumps = file.lines().
        map(|l| l.unwrap().parse::<i32>().unwrap()).
        collect::<Vec<_>>();

    println!("part 1: {}", steps(&jumps, false));
    println!("part 2: {}", steps(&jumps, true));
}

fn steps(jumps: &Vec<i32>, part2: bool) -> u32 {
    let mut steps = 0;
    let mut index = 0;
    let mut jumps = jumps.clone();
    let len = jumps.len() as i32;

    while index >= 0 && index < len {
        let j = jumps[index as usize];

        jumps[index as usize] += if j >= 3 && part2 { -1 } else { 1 };
        steps += 1;
        index += j;
    }

    steps
}
