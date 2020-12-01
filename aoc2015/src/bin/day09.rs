extern crate permutohedron;
extern crate pcre;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};
use permutohedron::Heap;
use pcre::Pcre;

fn main() {
    let f = File::open("day9.in").unwrap();
    let file = BufReader::new(&f);

    let mut distances : HashMap<(String,String), i32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();
    let mut re = Pcre::compile(r"(\w+) to (\w+) = (\d+)").unwrap();
    for line in file.lines() {
        let line = line.unwrap();

        let m = re.exec(&line).unwrap();
        let c1 = m.group(1).to_string();
        let c2 = m.group(2).to_string();

        let d = m.group(3).parse().unwrap();

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
