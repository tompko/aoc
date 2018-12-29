use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("input/day25.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut points = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        let parts = line.split(",").collect::<Vec<_>>();
        points.push((
            parts[0].parse::<i64>().unwrap(),
            parts[1].parse::<i64>().unwrap(),
            parts[2].parse::<i64>().unwrap(),
            parts[3].parse::<i64>().unwrap(),
        ));
    }

    let mut open: HashSet<_> = (0..points.len()).into_iter().collect();
    let mut candidates = HashSet::new();
    let mut constellations = Vec::new();
    let mut current = Vec::new();

    while !open.is_empty() {
        let next = (0..points.len()).into_iter().filter(|x| open.contains(x)).next().unwrap();
        let np = points[next];
        open.remove(&next);
        current.push(next);

        for (i, p) in points.iter().enumerate() {
            if open.contains(&i) && distance(np, *p) <= 3 {
                candidates.insert(i);
            }
        }

        while !candidates.is_empty() {
            let next = (0..points.len()).into_iter().filter(|x| candidates.contains(x)).next().unwrap();
            let np = points[next];
            open.remove(&next);
            candidates.remove(&next);
            current.push(next);

            for (i, p) in points.iter().enumerate() {
                if open.contains(&i) && distance(np, *p) <= 3 {
                    candidates.insert(i);
                }
            }
        }

        constellations.push(current);
        current = Vec::new();
    }

    println!("part 1: {}", constellations.len());
}

fn distance(a: (i64, i64, i64, i64), b: (i64, i64, i64, i64)) -> i64 {
    (a.0 - b.0).abs() +
    (a.1 - b.1).abs() +
    (a.2 - b.2).abs() +
    (a.3 - b.3).abs()
}
