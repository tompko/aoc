use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Clone)]
struct Counter {
    counts: HashMap<char, u32>,
}

impl Counter {
    fn new() -> Self {
        Counter{
            counts: HashMap::new(),
        }
    }

    fn push(&mut self, ch: &char) {
        *self.counts.entry(*ch).or_insert(0) += 1
    }

    fn majority(&self) -> char {
        let mut freqs: Vec<_> = self.counts.iter().collect();
        freqs.sort_by(|a, b| {
            match a.1.cmp(b.1) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => a.0.cmp(b.0),
            }
        });

        *freqs[0].0
    }

    fn minority(&self) -> char {
        let mut freqs: Vec<_> = self.counts.iter().collect();
        freqs.sort_by(|a, b| {
            match a.1.cmp(b.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => a.0.cmp(b.0),
            }
        });

        *freqs[0].0
    }
}

fn main() {
    let file = File::open("input/day06.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut m = vec![Counter::new(); 8];

    for line in reader.lines() {
        let line = line.unwrap();

        for (i, ch) in line.trim().chars().enumerate() {
            m[i].push(&ch)
        }
    }

    let part1_ans: String = m.iter().map(|m| m.majority()).collect();
    let part2_ans: String = m.iter().map(|m| m.minority()).collect();

    println!("1: {}", part1_ans);
    println!("2: {}", part2_ans);
}
