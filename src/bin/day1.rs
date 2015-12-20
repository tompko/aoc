use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("day1.in")
        .ok()
        .expect("Failed to open input");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .ok()
        .expect("Failed to read input");

    let mut floor = 0;
    let mut basement: Option<usize> = None;
    for (i, c) in contents.chars().enumerate() {
        floor += match c {
            ')' => -1,
            '(' => 1,
            _ => 0,
        };

        if floor == -1 && basement.is_none() {
            basement = Some(i + 1);
        }
    }
    println!("{}", floor);
    println!("{}", basement.unwrap());
}
