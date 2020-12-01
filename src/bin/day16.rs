extern crate regex;

use std::fs::File;
use std::io::Read;
use regex::Regex;
use std::collections::VecDeque;

const START: &'static str = "abcdefghijklmnop";

fn main() {
    let mut file = File::open("input/day16.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let mut programs = START.chars().collect::<VecDeque<_>>();

    dance(&mut programs, contents);

    println!("part 1: {}", programs.iter().collect::<String>());

    let mut cycle_length = 1;

    loop {
        if START == &programs.iter().collect::<String>() {
            break
        }

        dance(&mut programs, contents);
        cycle_length += 1;
    }

    let limit = 1000000000 % cycle_length;

    programs = START.chars().collect::<VecDeque<_>>();

    for _ in 0..limit {
        dance(&mut programs, contents);
    }

    println!("part 2: {}", programs.iter().collect::<String>());
}

fn dance(programs: &mut VecDeque<char>, steps: &str) {
    let spin = Regex::new(r"s(\d+)").unwrap();
    let exchange = Regex::new(r"x(\d+)/(\d+)").unwrap();
    let partner = Regex::new(r"p(\w+)/(\w+)").unwrap();

    for step in steps.split(",") {
        if let Some(caps) = spin.captures(step) {
            let i = caps[1].parse::<usize>().unwrap();

            for _ in 0..i {
                let n = programs.pop_back().unwrap();
                programs.push_front(n);
            }
        } else if let Some(caps) = exchange.captures(step) {
            let a = caps[1].parse::<usize>().unwrap();
            let b = caps[2].parse::<usize>().unwrap();

            programs.swap(a, b);
        } else if let Some(caps) = partner.captures(step) {
            let a = caps[1].chars().nth(0).unwrap();
            let b = caps[2].chars().nth(0).unwrap();

            let x = programs.iter().position(|&x| x == a).unwrap();
            let y = programs.iter().position(|&x| x == b).unwrap();

            programs.swap(x, y);
        }
    }
}
