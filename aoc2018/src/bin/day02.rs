use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    let file = File::open("input/day02.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let ids: Vec<_> = file.lines().map(|x| x.unwrap()).collect();

    let part1 = checksum(&ids);

    let mut part2 = String::new();
    for id in ids.iter() {
        for cmp in ids.iter() {
            if let Some(ans) = diff(id, cmp) {
                part2 = ans;
                break
            }
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn checksum(ids: &Vec<String>) -> u32 {
    let (mut count2, mut count3) = (0, 0);

    for id in ids.iter() {
        let mut counts = HashMap::new();
        for c in id.chars() {
            counts.entry(c)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        let (mut has2, mut has3) = (false, false);

        for v in counts.values() {
            has2 = has2 || *v == 2;
            has3 = has3 || *v == 3;
        }

        if has2 { count2 += 1; }
        if has3 { count3 += 1; }
    }

    count2 * count3
}

fn diff(id: &str, cmp: &str) -> Option<String> {
    let mut ds = 0;
    let mut ret = String::new();

    for (a, b) in id.chars().zip(cmp.chars()) {
        if a == b {
            ret.push(a);
        } else {
            ds += 1;
        }
    }

    if ds == 1 { Some(ret) } else { None }
}
