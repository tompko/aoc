extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::max;
use regex::Regex;

fn main() {
    let file = File::open("input/day08.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut registers = HashMap::new();
    let inst_regex = Regex::new(r"(\w+) (inc|dec) (-?\d+) if (\w+) ([<>!=]+) (-?\d+)").unwrap();
    let mut part2 = 0;

    for line in file.lines() {
        let line = line.unwrap();

        let caps = inst_regex.captures(&line).unwrap();

        let cond_reg = &caps[4];
        let cond_val = caps[6].parse::<i32>().unwrap();
        let valid = match &caps[5] {
            "==" => *registers.get(cond_reg).unwrap_or(&0) == cond_val,
            "!=" => *registers.get(cond_reg).unwrap_or(&0) != cond_val,
            ">=" => *registers.get(cond_reg).unwrap_or(&0) >= cond_val,
            ">" => *registers.get(cond_reg).unwrap_or(&0) > cond_val,
            "<=" => *registers.get(cond_reg).unwrap_or(&0) <= cond_val,
            "<" => *registers.get(cond_reg).unwrap_or(&0) < cond_val,
            _ => unreachable!(),
        };

        if valid {
            let val = caps[3].parse::<i32>().unwrap();
            let reg = &caps[1];
            let res = match &caps[2] {
                "inc" => registers.get(reg).unwrap_or(&0) + val,
                "dec" => registers.get(reg).unwrap_or(&0) - val,
                _ => unreachable!(),
            };

            registers.insert(reg.to_owned(), res);
            part2 = max(part2, res);
        }
    }

    let part1 = registers.values().
        max().unwrap();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
