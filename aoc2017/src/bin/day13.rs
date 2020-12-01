use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input/day13.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut layers = HashMap::new();

    for line in file.lines() {
        let line = line.unwrap();
        let line = line.split(": ").collect::<Vec<_>>();
        let layer = line[0].parse::<u32>().unwrap();
        let depth = line[1].parse::<u32>().unwrap();

        layers.insert(layer, depth);
    }

    let mut part2 = 0;

    while caught(part2, &layers) {
        part2 += 1;
    }

    println!("part 1: {}", severity(0, &layers));
    println!("part 2: {}", part2);
}

fn severity(delay: u32, layers: &HashMap<u32, u32>) -> u32 {
    layers.iter().
        map(|(&l, &d)| (l, d, pos_at(delay + l, d))).
        filter(|&(_, _, p)| p == 0).
        map(|(l, d, _)| l*d).
        sum()
}

fn caught(delay: u32, layers: &HashMap<u32, u32>) -> bool {
    layers.iter().
        any(|(&l, &d)| pos_at(delay + l, d) == 0)
}

fn pos_at(time: u32, depth: u32) -> u32 {
    let pos = time % (2 * depth - 2);
    if pos > (depth - 1) {
        2 * depth - pos - 2
    } else {
        pos
    }
}
