extern crate aoc2018;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use aoc2018::Device;

fn main() {
    let mut device = Device::new();

    let file = File::open("input/day19.in").expect("Failed to open input");
    let file = BufReader::new(&file);
    let ip_re = Regex::new(r"#ip (\d)").unwrap();

    for line in file.lines() {
        let line = line.unwrap();

        if let Some(caps) = ip_re.captures(&line) {
            let s = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            device.shadow_ip(s);
        } else {
            device.push(&line);
        }
    }

    device.run_all();

    println!("part 1: {}", device.registers[0]);
    // The program calculates the sum of the divisors of register[2] - 10551432
    println!("part 2: {}", 27024480);
}
