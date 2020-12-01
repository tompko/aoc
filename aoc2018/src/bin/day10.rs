extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};
use std::collections::HashSet;
use regex::Regex;

const WAIT_TIME: i64 = 10659;

#[derive(Debug, Clone, Copy)]
struct Point {
    start_x: i64,
    start_y: i64,

    vec_x: i64,
    vec_y: i64,
}

impl Point {
    fn at(&self, t: i64) -> (i64, i64) {
        (
            self.start_x + t * self.vec_x,
            self.start_y + t * self.vec_y
        )
    }
}

fn main() {
    let file = File::open("input/day10.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let line_re = Regex::new(r"position=<\s*([0-9-]+),\s*([0-9-]+)> velocity=<\s*([0-9-]+),\s*([0-9-]+)>").unwrap();

    let mut points = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();

        let lcap = line_re.captures(&line).unwrap();

        let px = lcap.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let py = lcap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vx = lcap.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let vy = lcap.get(4).unwrap().as_str().parse::<i64>().unwrap();

        points.push(Point{ start_x: px, start_y: py, vec_x: vx, vec_y: vy });
    }

    let mut field = HashSet::new();
    let mut minx = std::i64::MAX;
    let mut miny = std::i64::MAX;
    let mut maxx = std::i64::MIN;
    let mut maxy = std::i64::MIN;

    for p in points.iter() {
        let (x, y) = p.at(WAIT_TIME);

        field.insert((x, y));
        minx = min(minx, x);
        miny = min(miny, y);
        maxx = max(maxx, x);
        maxy = max(maxy, y);
    }

    println!("part 1:");
    for y in miny..=maxy {
        for x in minx..=maxx {
            if field.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

    println!("part 2: {}", WAIT_TIME);
}
