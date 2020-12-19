use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day06.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut group_answers = HashMap::new();
    let mut group_size = 0;
    let mut part1 = 0;
    let mut part2 = 0;
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line == "" {
            part1 += group_answers.len();
            for &v in group_answers.values() {
                if v == group_size {
                    part2 += 1;
                }
            }
            group_answers = HashMap::new();
            group_size = 0;
        } else {
            for c in line.chars() {
                let e = group_answers.entry(c).or_insert(0);
                *e += 1;
            }
            group_size += 1;
        }
    }
    part1 += group_answers.len();
    for &v in group_answers.values() {
        if v == group_size {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}