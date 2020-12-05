use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day05.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut seats: Vec<usize> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let mut seat_id = 0;
        for c in line.chars() {
            match c {
                'F' | 'L' => seat_id *= 2,
                'B' | 'R' => seat_id = (seat_id * 2) + 1,
                _ => unreachable!(),
            }
        }
        seats.push(seat_id);
    }

    seats.sort();

    let part1 = seats[seats.len()-1];
    let mut part2 = 0;
    for i in 0..(seats.len()-1) {
        if seats[i] + 1 != seats[i + 1] {
            part2 = seats[i] + 1;
            break;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}