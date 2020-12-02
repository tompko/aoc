extern crate permutohedron;
extern crate fancy_regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use permutohedron::Heap;
use fancy_regex::Regex;

fn main() {
    let f = File::open("inputs/day09.in").unwrap();
    let file = BufReader::new(&f);

    let mut distances : HashMap<(String,String), i32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    for line in file.lines() {
        let line = line.unwrap();

        let m = re.captures(&line).unwrap().unwrap();
        let c1 = m.get(1).unwrap().as_str().to_owned();
        let c2 = m.get(2).unwrap().as_str().to_owned();

        let d = m.get(3).unwrap().as_str().parse().unwrap();

        distances.insert((c1.clone(), c2.clone()), d);
        distances.insert((c2.clone(), c1.clone()), d);
        cities.insert(c1);
        cities.insert(c2);
    }

    let mut min_dist = 1000000;
    let mut max_dist = 0;

    let mut cities_vec: Vec<String> = cities.into_iter().collect();

    for p in Heap::new(&mut cities_vec) {
        let mut dist = 0;
        for (a,b) in p.iter().zip(p[1..p.len()].iter()) {
            dist += distances[&(a.to_string(),b.to_string())];
        }

        min_dist = min(min_dist, dist);
        max_dist = max(max_dist, dist);
    }

    println!("{}", min_dist);
    println!("{}", max_dist);
}
