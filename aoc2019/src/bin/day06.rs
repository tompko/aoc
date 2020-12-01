use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

fn distances(edges: &HashMap<String, Vec<String>>, start: &str) -> HashMap<String, usize> {
    let mut visits = VecDeque::new();
    visits.push_back(start.to_string());
    let mut dists: HashMap<String, usize> = HashMap::new();
    dists.insert(start.to_string(), 0);

    while visits.len() > 0 {
        let curr = visits.pop_front().unwrap();
        let curr_dist = *dists.get(&curr).unwrap();
        for dest in edges.get(&curr).unwrap() {
            let d = *dists.get(dest).unwrap_or(&edges.len());
            if curr_dist+1 < d {
                dists.insert(dest.to_owned(), curr_dist+1);
                visits.push_back(dest.to_owned());
            }
        }
    }
    dists
}

fn main() {
    let file = File::open("input/day06.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut orbits = HashMap::new();

    for line in file.lines() {
        let line = line.unwrap();
        let mut parts = line.split(')');
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        orbits.entry(a.to_owned()).or_insert(Vec::new()).push(b.to_owned());
        orbits.entry(b.to_owned()).or_insert(Vec::new()).push(a.to_owned());
    }

    let com_distances = distances(&orbits, "COM");
    let you_distances = distances(&orbits, "YOU");

    println!("Part 1: {}", com_distances.values().sum::<usize>());
    println!("Part 2: {}", you_distances.get("SAN").unwrap()-2);
}
