use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let file = File::open("input/day03.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut valid1 = 0;
    let mut valid2 = 0;
    let mut col0 = Vec::with_capacity(3);
    let mut col1 = Vec::with_capacity(3);
    let mut col2 = Vec::with_capacity(3);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut sides: Vec<_> = line.trim().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        col0.push(sides[0]);
        col1.push(sides[1]);
        col2.push(sides[2]);

        sides.sort();

        if sides[0] + sides[1] > sides[2] {
            valid1 += 1
        }

        if col0.len() == 3 {
            col0.sort();
            col1.sort();
            col2.sort();

            if col0[0] + col0[1] > col0[2] {
                valid2 += 1;
            }
            if col1[0] + col1[1] > col1[2] {
                valid2 += 1;
            }
            if col2[0] + col2[1] > col2[2] {
                valid2 += 1;
            }

            col0.truncate(0);
            col1.truncate(0);
            col2.truncate(0);
        }
    }

    println!("1: {}", valid1);
    println!("2: {}", valid2);
}
