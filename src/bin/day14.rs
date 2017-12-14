extern crate aoc2017;

use std::collections::{HashMap, HashSet};
use aoc2017::knot::hash;

fn main() {
    let input = "wenycdww";

    let layout = (0..16).into_iter().
        map(|x| (format!("{:x}", x).chars().nth(0).unwrap(), format!("{:04b}", x))).
        collect::<HashMap<_, _>>();
    let grid = (0..128).into_iter().
        map(|x| hash(&format!("{}-{}", input, x))).
        map(|x| x.chars().map(|x| layout[&x].to_owned()).collect::<String>()).
        map(|x| x.chars().map(|x| x == '1').collect::<Vec<_>>()).
        collect::<Vec<Vec<bool>>>();
    let part1: u32 = grid.iter().
        map(|x| x.iter().map(|&c| if c { 1 } else { 0 }).sum::<u32>()).
        sum();

    let mut coords = HashSet::new();

    for (y, g) in grid.iter().enumerate() {
        for (x, &c) in g.iter().enumerate() {
            if c {
                coords.insert((x as i32, y as i32));
            }
        }
    }

    let mut regions = 0;

    while coords.len() > 0 {
        let mut pending = Vec::new();
        let c = *coords.iter().nth(0).unwrap();
        pending.push(c);
        coords.remove(&c);

        while pending.len() > 0 {
            let s = pending.pop().unwrap();

            for n in neighbours(s) {
                if coords.remove(&n) {
                    pending.push(n);
                }
            }
        }

        regions += 1;
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", regions);
}

fn neighbours(c: (i32, i32)) -> Vec<(i32, i32)> {
    vec![(c.0+1, c.1), (c.0-1, c.1), (c.0, c.1+1), (c.0, c.1-1)]
}
