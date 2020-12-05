extern crate fancy_regex;
extern crate permutohedron;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use fancy_regex::Regex;
use permutohedron::Heap;

struct Table {
    happiness: HashMap<(String, String), i32>,
    names: HashSet<String>,
    re: Regex,
}

impl Table {
    fn new() -> Self {
        Table {
            happiness: HashMap::new(),
            names: HashSet::new(),
            re: Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)").unwrap(),
        }
    }

    fn add(&mut self, line: &str) {
        let capture = self.re.captures(line).unwrap().unwrap();
        let a = capture.get(1).unwrap().as_str();
        let b = capture.get(4).unwrap().as_str();
        let gain_lose = capture.get(2).unwrap().as_str();
        let absh: i32 = capture.get(3).unwrap().as_str().parse().unwrap();

        let h: i32 = if gain_lose == "lose" { -absh } else { absh };
        self.happiness.insert((a.to_string(), b.to_string()), h);
        self.names.insert(a.to_string());
        self.names.insert(b.to_string());
    }

    fn seat(&self) -> i32 {
        let mut max_score = 0;
        let mut names: Vec<&String> = self.names.iter().collect();
        let permutations = Heap::new(&mut names);
    
        for p in permutations {
            let mut s = 0;
            for i in 0..(p.len()-1) {
                s += self.happiness.get(&(p[i].to_string(), p[i+1].to_string())).unwrap();
                s += self.happiness.get(&(p[i+1].to_string(), p[i].to_string())).unwrap();
            }
            s += self.happiness.get(&(p[p.len()-1].to_string(), p[0].to_string())).unwrap();
            s += self.happiness.get(&(p[0].to_string(), p[p.len()-1].to_string())).unwrap();

            max_score = max(s, max_score)
        }
        max_score
    }
}

fn main() {
    let f = File::open("inputs/day13.in").unwrap();
    let file = BufReader::new(&f);

    let mut table = Table::new();
    for line in file.lines() {
        let line = line.unwrap();
        table.add(&line);
    }

    println!("Part 1: {}", table.seat());

    table.names.insert("me".to_string());
    for n in table.names.iter() {
        table.happiness.insert(("me".to_string(), n.to_string()), 0);
        table.happiness.insert((n.to_string(), "me".to_string()), 0);
    }

    println!("Part 2: {}", table.seat());
}


