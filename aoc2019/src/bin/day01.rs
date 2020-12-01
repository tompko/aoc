use std::fs::File;
use std::io::{BufRead, BufReader};

fn total_fuel(a: i32) -> i32 {
    let fuel = a/3 - 2;
    if fuel <= 0 { 0 } else { fuel + total_fuel(fuel) }
}

fn main() {
    let file = File::open("input/day01.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let masses: Vec<i32> = file.lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();

    let fuel: i32 = masses.iter().map(|x| x/3-2).sum();
    println!("Part 1: {}", fuel);
    let tot: i32 = masses.iter().map(|x| total_fuel(*x)).sum();
    println!("Part 2: {}", tot);
}
