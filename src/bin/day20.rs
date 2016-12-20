use std::cmp::max;
use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("input/day20.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut intervals = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        let bounds: Vec<_> = line.split('-').collect();
        let low: u32 = bounds[0].parse().unwrap();
        let high: u32 = bounds[1].parse().unwrap();
        intervals.push((low, high));
    }

    intervals.sort();

    let mut lowest = 0;
    for &(low, high) in &intervals {
        if lowest < low {
            break
        }
        lowest = max(lowest, high + 1);
    }

    let mut length = 0;
    let mut highest = 0;
    for &(low, high) in &intervals {
        if high < highest {
            continue
        }
        if low < highest {
            length += high - highest;
        } else {
            length += high - low + 1;
        }
        highest = high;
    }

    println!("1: {}", lowest);
    println!("2: {}", 4294967295 - length + 1);
}
