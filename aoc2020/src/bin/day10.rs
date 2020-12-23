use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn arrangements(diffs: &Vec<u32>) -> usize {
    let mut memo = HashMap::new();
    let last = diffs[diffs.len()-1] + 3;
    memo.insert(last, 1);

    for d in diffs.iter().rev() {
        let mut a = 0;
        for i in 1..4 {
            if let Some(m) = memo.get(&(d + i)) {
                a += m;
            }
        }
        memo.insert(*d, a);
    }

    let mut ret = 0;
    for i in 1..4 {
        if let Some(m) = memo.get(&i) {
            ret += m;
        }
    }

    ret
}

fn main() {
    let file = File::open("input/day10.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut joltages: Vec<_> = file.lines().map(|l| l.unwrap().parse::<u32>().unwrap()).collect();
    joltages.sort();

    let mut prev = 0;
    let mut diffs = HashMap::new();
    diffs.insert(3, 1);
    for &j in joltages.iter() {
        let d = j - prev;
        let e = diffs.entry(d).or_insert(0);
        *e += 1;
        prev = j;
    }

    let part1 = diffs[&1] * diffs[&3];

    let part2 = arrangements(&joltages);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}