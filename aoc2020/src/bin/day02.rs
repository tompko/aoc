extern crate fancy_regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use fancy_regex::Regex;

struct Policy {
    min: usize,
    max: usize,
    c: char,
    pass: String,
}

impl FromStr for Policy {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)")?;
        let captures = re.captures(s)?.unwrap();

        let min = captures.get(1).unwrap().as_str().parse()?;
        let max = captures.get(2).unwrap().as_str().parse()?;
        let c = captures.get(3).unwrap().as_str().chars().next().unwrap();
        let pass = captures.get(4).unwrap().as_str().to_owned();

        let p = Policy {
            min: min,
            max:max,
            c: c,
            pass: pass,
        };

        Ok(p)
    }
}

impl Policy {
    fn is_valid1(&self) -> bool {
        let c = self.pass.chars().filter(|&x| x == self.c).count();

        self.min <= c && c <= self.max
    }

    fn is_valid2(&self) -> bool {
        let c: Vec<char> = self.pass.chars().collect();

        (c[self.min-1] == self.c) ^ (c[self.max-1] == self.c)
    }
}

fn main() {
    let file = File::open("input/day02.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let policies: Vec<Policy> = file.lines().map(|x| x.unwrap().parse().unwrap()).collect();

    let part1 = policies.iter().filter(|p| p.is_valid1()).count();
    let part2 = policies.iter().filter(|p| p.is_valid2()).count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}