extern crate itertools;
extern crate regex;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Vector) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vector {
    fn cmp(&self, other: &Vector) -> Ordering {
        let ssum = self.x.abs() + self.y.abs() + self.z.abs();
        let osum = other.x.abs() + other.y.abs() + other.z.abs();

        ssum.cmp(&osum)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl FromStr for Particle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>"
        ).unwrap();
        let caps = re.captures(s).unwrap();

        Ok(Particle {
            p: Vector {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
                z: caps[3].parse().unwrap(),
            },
            v: Vector {
                x: caps[4].parse().unwrap(),
                y: caps[5].parse().unwrap(),
                z: caps[6].parse().unwrap(),
            },
            a: Vector {
                x: caps[7].parse().unwrap(),
                y: caps[8].parse().unwrap(),
                z: caps[9].parse().unwrap(),
            },
        })
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        let acc = self.a.cmp(&other.a);
        let vel = self.v.cmp(&other.v);
        let pos = self.p.cmp(&other.p);

        if acc != Ordering::Equal {
            acc
        } else if vel != Ordering::Equal {
            vel
        } else {
            pos
        }
    }
}

fn main() {
    let file = File::open("input/day20.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut particles = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();

        particles.push(line.parse::<Particle>().unwrap());
    }

    let mut sorted = particles.iter()
        .enumerate()
        .collect::<Vec<_>>();
    sorted.sort_by_key(|&(_, p)| p);
    let part1 = sorted[0].0;

    println!("part 1: {}", part1);
    // println!("part 2: {}", data);
}
