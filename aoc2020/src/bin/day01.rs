use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day01.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let expenses: Vec<i32> = file.lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 0..expenses.len() {
        for j in i..expenses.len() {
            if expenses[i] + expenses[j] == 2020 {
                part1 = expenses[i] * expenses[j]
            }

            for k in j..expenses.len() {
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    part2 = expenses[i] * expenses[j] * expenses[k];
                }
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}