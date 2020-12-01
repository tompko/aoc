extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Rect {
    id: u32,
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

fn main() {
    let file = File::open("input/day03.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let line_re = Regex::new(r"#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)").unwrap();

    let mut claims = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();

        let lcap = line_re.captures(line).unwrap();

        let r = Rect {
            id: lcap.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            left: lcap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            top: lcap.get(3).unwrap().as_str().parse::<u32>().unwrap(),
            width: lcap.get(4).unwrap().as_str().parse::<u32>().unwrap(),
            height: lcap.get(5).unwrap().as_str().parse::<u32>().unwrap(),
        };

        claims.push(r);
    }

    let mut squares = HashMap::new();

    for c in claims.iter() {
        for x in c.left..(c.left+c.width) {
            for y in c.top..(c.top+c.height) {
                squares.entry((x, y))
                    .and_modify(|s| *s += 1)
                    .or_insert(1);
            }
        }
    }

    let part1 = squares.values().filter(|&x| *x > 1).count();

    let mut part2 = 0;

    for c in claims.iter() {
        let mut ok = true;
        for x in c.left..(c.left+c.width) {
            for y in c.top..(c.top+c.height) {
                if *squares.get(&(x,y)).unwrap_or(&0) != 1 {
                    ok = false;
                }
            }
        }
        if ok {
            part2 = c.id;
            break;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
