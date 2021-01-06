use std::collections::HashSet;
use std::str::FromStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

#[derive(Debug, Clone)]
struct Range {
    name: String,

    min_a: usize,
    max_a: usize,
    min_b: usize,
    max_b: usize,
}

impl FromStr for Range {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        Ok(Range {
            name: captures.get(1).unwrap().as_str().to_string(),
            min_a: captures.get(2).unwrap().as_str().parse::<usize>()?,
            max_a: captures.get(3).unwrap().as_str().parse::<usize>()?,
            min_b: captures.get(4).unwrap().as_str().parse::<usize>()?,
            max_b: captures.get(5).unwrap().as_str().parse::<usize>()?,
        })
    }
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        (self.min_a <= n && n <= self.max_a) || (self.min_b <= n && n <= self.max_b)
    }
}

fn find_valid(ranges: &Vec<Range>, tickets: &Vec<Vec<usize>>, assignments: Vec<usize>) -> Option<Vec<usize>> {
    let seen: HashSet<_> = assignments.iter().collect();
    let index = assignments.len();

    if assignments.len() == ranges.len() {
        return Some(assignments);
    }

    for i in 0..ranges.len() {
        if seen.contains(&i) {
            continue;
        }
        let mut valid = true;
        for t in tickets.iter() {
            if !ranges[i].contains(t[index]) {
                valid = false;
                break;
            }
        }
        if !valid {
            continue;
        }

        let mut assignments = assignments.clone();
        assignments.push(i);
        if let Some(a) = find_valid(ranges, tickets, assignments) {
            return Some(a);
        }
    }

    None
}

fn main() {
    let file = File::open("input/day16.txt").expect("Failed to open input");
    let file = BufReader::new(&file);
    let mut lines = file.lines();

    let mut ranges = Vec::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        let r = line.parse::<Range>().unwrap();
        ranges.push(r);
    }

    let line = lines.next().unwrap().unwrap();
    assert_eq!(line, "your ticket:");

    let your_ticket: Vec<_> = lines.next().unwrap().unwrap().split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    let line = lines.next().unwrap().unwrap();
    assert_eq!(line, "");
    let line = lines.next().unwrap().unwrap();
    assert_eq!(line, "nearby tickets:");

    let mut tickets = Vec::new();
    loop {
        if let Some(line) = lines.next() {
            let line = line.unwrap();
            let ticket: Vec<_> = line.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            tickets.push(ticket);
        } else {
            break;
        }
    }

    let mut part1 = 0;
    let mut valid_tickets = Vec::new();
    for ticket in tickets.iter() {
        let mut ticket_valid = true;
        for &value in ticket.iter() {
            let valid = ranges.iter().any(|r| r.contains(value));
            if !valid {
                part1 += value;
                ticket_valid = false;
            }
        }
        if ticket_valid {
            valid_tickets.push(ticket.clone());
        }
    }

    let mut part2 = 1;
    let mapping = find_valid(&ranges, &valid_tickets, Vec::new()).unwrap();

    for (d, &m) in your_ticket.iter().zip(mapping.iter()) {
        if ranges[m].name.starts_with("departure") {
            part2 *= d;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}