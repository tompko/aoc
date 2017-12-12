extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use regex::Regex;

fn main() {
    let file = File::open("input/day12.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let line_regex = Regex::new(r"(\d+) <-> ([0-9, ]+)").unwrap();
    let mut graph = HashMap::new();

    for line in file.lines() {
        let line = line.unwrap();
        let caps = line_regex.captures(&line).unwrap();

        let program = caps[1].parse::<u32>().unwrap();
        let comms = caps[2].split(", ").
            map(|x| x.parse::<u32>().unwrap()).
            collect::<Vec<_>>();

        graph.insert(program, comms);
    }

    let mut num_groups = 0;
    let mut seen = HashSet::new();

    for i in graph.keys() {
        if !seen.contains(i) {
            num_groups += 1;
            for n in group_for(*i, &graph) {
                seen.insert(n);
            }
        }
    }

    println!("part 1: {}", group_for(0, &graph).len());
    println!("part 2: {}", num_groups);
}

fn group_for(node: u32, graph: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
    let mut pending = vec![node];
    let mut group = HashSet::new();
    group.insert(node);

    while let Some(next) = pending.pop() {
        for &c in graph[&next].iter() {
            if group.insert(c) {
                pending.push(c);
            }
        }
    }

    group
}
