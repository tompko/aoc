extern crate aoc2018;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use aoc2018::Device;

fn main() {
    let mut device = Device::new();

    let file = File::open("input/day21.in").expect("Failed to open input");
    let file = BufReader::new(&file);
    let ip_re = Regex::new(r"#ip (\d)").unwrap();

    device.registers[0] = 10332277;
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

    println!("part 1: {}", 10332277);
    println!("part 2: {}", 13846724);
}
