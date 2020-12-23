extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

struct MemIter {
    mask: String,
    index: usize,

    counter: usize,
    num_xs: usize,
}

impl Iterator for MemIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter > (1 << self.num_xs) {
            return None;
        }

        let mut res = 0;
        let mut ci = 0;
        for (i, c) in self.mask.chars().enumerate() {
            match c {
                '0' => {
                    if self.index & (1 << 35-i) != 0 {
                        res = (res << 1) + 1;
                    } else {
                        res = res << 1;
                    }
                }
                '1' => res = (res << 1) + 1,
                'X' => {
                    if self.counter & (1 << ci) != 0 {
                        res = (res << 1) + 1;
                    } else {
                        res = res << 1;
                    }
                    ci += 1;
                }
                _ => unreachable!(),
            }
        }

        self.counter += 1;
        Some(res)
    }
}

fn index_mask(mask: &str, index: usize) -> MemIter {
    MemIter{
        mask: mask.to_string(),
        index: index,
        counter: 0,
        num_xs: mask.chars().filter(|&x| x == 'X').count(),
    }
}

fn value_mask(mask: &str, n: usize) -> usize {
    let mut res = 0;

    for (i, c) in mask.chars().enumerate() {
        match c {
            '0' => res = res << 1,
            '1' => res = (res << 1) + 1,
            'X' => {
                if n & (1 << 35-i) != 0 {
                    res = (res << 1) + 1;
                } else {
                    res = res << 1;
                }
            }
            _ => unreachable!(),
        }
    }

    res
}

fn main() {
    let file = File::open("input/day14.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut memory1: HashMap<usize, usize> = HashMap::new();
    let mut memory2: HashMap<usize, usize> = HashMap::new();
    let mut mask = String::new();

    let mask_re = Regex::new(r"mask = ([01X]+)").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)] = (\d+)").unwrap();
    
    for line in file.lines() {
        let line = line.unwrap();
        if let Some(cs) = mask_re.captures(&line) {
            mask = cs.get(1).unwrap().as_str().to_string();
        } else if let Some(cs) = mem_re.captures(&line) {
            let index = cs.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let value = cs.get(2).unwrap().as_str().parse::<usize>().unwrap();

            memory1.insert(index, value_mask(&mask, value));

            for i in index_mask(&mask, index) {
                memory2.insert(i, value);
            }
        } else {
            unreachable!();
        }
    }

    let part1: usize = memory1.values().sum();
    let part2: usize = memory2.values().sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}