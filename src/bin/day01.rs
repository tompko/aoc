use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("input/day01.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let changes: Vec<i32> = file.lines()
        .map(|x| x.unwrap().parse::<i32>().unwrap())
        .collect();

    let part1: i32 = changes.iter().sum();

    let mut seen = HashSet::new();
    let mut part2 = 0;
    seen.insert(0);

    for cf in changes.iter().cycle().scan(0, |s,&x| {*s+=x; Some(*s)}) {
        if !seen.insert(cf) {
            part2 = cf;
            break;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
