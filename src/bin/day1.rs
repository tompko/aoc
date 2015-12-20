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
        match c {
            ')' => floor -= 1,
            '(' => floor += 1,
            _ => {}
        }

        if floor == -1 && basement.is_none() {
            basement = Some(i + 1);
        }
    }
    println!("{}", floor);
    println!("{}", basement.unwrap());
}
