extern crate regex;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug)]
struct Bag {
    name: String,

    sub_bags: Vec<(usize, String)>
}

impl Bag {
    fn contains(&self, name: &str) -> bool {
        for (_, n) in self.sub_bags.iter() {
            if n == name {
                return true;
            }
        }
        false
    }
}

fn recur_count(bags: &HashMap<String, Bag>, name: &str, multiplier: usize) -> usize {
    let mut ret = 1;

    let b = &bags[name];
    for (mul, sb) in b.sub_bags.iter() {
        ret += recur_count(bags, &sb, 1)*multiplier*mul;
    }

    ret
}

fn main() {
    let file = File::open("input/day07.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let name_re = Regex::new("^(.*) bags contain").unwrap();
    let sub_re = Regex::new(r"(\d+) (.*?) bags?[,.]").unwrap();

    let mut bags = HashMap::new();
    for line in file.lines() {
        let line = line.unwrap();
 
        let name = name_re.captures(&line).unwrap().get(1).unwrap().as_str();
        if line.contains("contain no other bags.") {
            let b = Bag{ name: name.to_string(), sub_bags: vec![] };
            bags.insert(name.to_string(), b);
        } else {
            let mut sub_bags = Vec::new();
            for cap in sub_re.captures_iter(&line) {
                let count = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let n = cap.get(2).unwrap().as_str();
                sub_bags.push((count, n.to_string()));
            }
            let b = Bag{name: name.to_string(), sub_bags: sub_bags};
            bags.insert(name.to_string(), b);
        }
    }

    let mut queue = vec!{"shiny gold"};
    let mut seen = HashSet::new();
    let mut containers = HashSet::new();

    while !queue.is_empty() {
        let curr = queue.pop().unwrap();

        if seen.contains(curr) {
            continue;
        }
        seen.insert(curr);

        for bag in bags.values() {
            if bag.contains(curr) {
                queue.push(&bag.name);
                containers.insert(bag.name.clone());
            }
        }
    }

    let part2 = recur_count(&bags, "shiny gold", 1) - 1;

    println!("Part 1: {}", containers.len());
    println!("Part 2: {}", part2);
}