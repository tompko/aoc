extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};
use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Nanobot {
    position: (i64, i64, i64),
    range: i64,
}

#[derive(Debug, Clone, Copy)]
struct AABB {
    min: (i64, i64, i64),
    max: (i64, i64, i64),
}


fn main() {
    let file = File::open("input/day23.in").expect("Failed to open input");
    let file = BufReader::new(&file);
    let lines = file.lines();

    let bot_re = Regex::new(r"pos=<([0-9-]+),([0-9-]+),([0-9-]+)>, r=([0-9-]+)").unwrap();

    let mut bots = Vec::new();

    for line in lines {
        let line = line.unwrap();

        let caps = bot_re.captures(&line).unwrap();

        bots.push(Nanobot {
            position: (
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            ),
            range: caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        });
    }

    bots.sort_by_key(|b| -b.range);

    let strong_pos = bots[0].position;
    let strong_range = bots[0].range;

    let mut part1 = 0;

    for b in bots.iter() {
        if distance3(strong_pos, b.position) <= strong_range {
            part1 += 1;
        }
    }

    let mut aabb = AABB::new();
    for b in bots.iter() {
        aabb.extend(b.position);
    }
    aabb.check();

    let mut boundaries = VecDeque::new();
    boundaries.push_back(aabb);
    let mut best_pos = (0,0,0);
    let mut best_score = 928; //score(best_pos, &bots);
    let mut best_dist = 0;

    while !boundaries.is_empty() {
        let bounds = boundaries.pop_front().unwrap();
        let bscore = bounds.score(&bots);
        println!("Ans: {:?} {} {} {} {} {}", bounds, bscore, bounds.max.0 - bounds.min.0, best_score, best_dist, boundaries.len());

        if bscore > best_score {
            for c in bounds.corners() {
                let s = score(c, &bots);
                if s > best_score {
                    best_score = s;
                    best_pos = c;
                    best_dist = distance3((0, 0, 0), c);
                } else if s == best_score && distance3((0,0,0), c) < best_dist {
                    best_score = s;
                    best_pos = c;
                    best_dist = distance3((0, 0, 0), c);
                }
            }

            for b in bounds.split() {
                boundaries.push_back(b);
            }
        } else if bscore == best_score {
            let mut min_dist = best_dist * 2;
            for c in bounds.corners() {
                let s = score(c, &bots);
                let d = distance3((0, 0, 0), c);
                if s > best_score {
                    best_score = s;
                    best_pos = c;
                    best_dist = d;
                } else if s == best_score && d < best_dist {
                    best_score = s;
                    best_pos = c;
                    best_dist = d;
                }
                min_dist = min(min_dist, d);
            }

            if min_dist < best_dist {
                for b in bounds.split() {
                    boundaries.push_back(b);
                }
            }
        }
    }

    println!("{:?} {} {}", best_pos, best_score, best_dist);

    println!("part 1: {}", part1);
    println!("part 2: {}", best_dist);
}

fn score(pos: (i64, i64, i64), bots: &Vec<Nanobot>) -> i64 {
    bots.iter()
        .map(|b| if distance3(b.position, pos) <= b.range { 1 } else { 0 })
        .sum()
}

fn distance3(x: (i64, i64, i64), y: (i64, i64, i64)) -> i64 {
    (x.0 - y.0).abs() +
    (x.1 - y.1).abs() +
    (x.2 - y.2).abs()
}

fn distance2(x: (i64, i64), y: (i64, i64)) -> i64 {
    (x.0 - y.0).abs() +
    (x.1 - y.1).abs()
}

fn distance1(x: i64, y: i64) -> i64 {
    (x - y).abs()
}

impl AABB {
    fn new() -> Self {
        AABB { min: (0, 0, 0), max: (0, 0, 0) }
    }

    fn check(&self) {
        assert!(self.min.0 <= self.max.0);
        assert!(self.min.1 <= self.max.1);
        assert!(self.min.2 <= self.max.2);
    }

    fn extend(&mut self, p: (i64, i64, i64)) {
        let min = (
            min(self.min.0, p.0),
            min(self.min.1, p.1),
            min(self.min.2, p.2),
        );
        let max = (
            max(self.max.0, p.0),
            max(self.max.1, p.1),
            max(self.max.2, p.2),
        );

        self.min = min;
        self.max = max;
    }

    fn corners(&self) -> Vec::<(i64, i64, i64)> {
        let mut ret = Vec::new();
        for bx in [self.min.0, self.max.0].iter() {
            for by in [self.min.1, self.max.1].iter() {
                for bz in [self.min.2, self.max.2].iter() {
                    ret.push((*bx, *by, *bz));
                }
            }
        }
        ret
    }

    fn split(self) -> Vec<AABB> {
        let mut ret = Vec::new();
        let mx = (self.max.0 + self.min.0) / 2;
        let my = (self.max.1 + self.min.1) / 2;
        let mz = (self.max.2 + self.min.2) / 2;

        println!("Split: {:?} {} {} {}", self, mx, my, mz);


        for x in [(self.min.0, mx-1), (mx, self.max.0)].iter() {
            for y in [(self.min.1, my-1), (my, self.max.1)].iter() {
                for z in [(self.min.2, mz-1), (mz, self.max.2)].iter() {
                    let a = AABB {
                        min: (x.0, y.0, z.0),
                        max: (x.1, y.1, z.1),
                    };
                    a.check();
                    ret.push(a);
                }
            }
        }

        ret
    }

    fn score(&self, bots: &Vec::<Nanobot>) -> i64 {
        let mut ret = 0;
        for b in bots.iter() {
            if self.distance(b.position) <= b.range {
                ret += 1;
            }
        }
        ret
    }

    fn distance(&self, (x, y, z): (i64, i64, i64)) -> i64 {
        if self.min.0 <= x && x <= self.max.0 {
            if self.min.1 <= y && y <= self.max.1 {
                if self.min.2 <= z && z <= self.max.2 {
                    0
                } else {
                    min((self.min.2 - z).abs(), (self.max.2 - z).abs())
                }
            } else if self.min.2 <= z && z <= self.max.2 {
                    min((self.min.1 - y).abs(), (self.max.1 - y).abs())
            } else {
                let mut best_dist = std::i64::MAX;
                for by in [self.min.1, self.max.1].iter() {
                    for bz in [self.min.2, self.max.2].iter() {
                        let d = distance2((*by, *bz), (y, z));
                        best_dist = min(best_dist, d);
                    }
                }
                best_dist
            }
        } else if self.min.1 <= y && y <= self.max.1 {
            if self.min.2 <= z && z <= self.max.2 {
                min((self.min.0 - y).abs(), (self.max.0 - y).abs())
            } else {
                let mut best_dist = std::i64::MAX;
                for bx in [self.min.0, self.max.0].iter() {
                    for bz in [self.min.2, self.max.2].iter() {
                        let d = distance2((*bx, *bz), (x, z));
                        best_dist = min(best_dist, d);
                    }
                }
                best_dist
            }
        } else if self.min.2 <= z && z <= self.max.2 {
            let mut best_dist = std::i64::MAX;
            for bx in [self.min.0, self.max.0].iter() {
                for by in [self.min.1, self.max.1].iter() {
                    let d = distance2((*bx, *by), (x, y));
                    best_dist = min(best_dist, d);
                }
            }
            best_dist
        } else {
            let mut best_dist = std::i64::MAX;
            for bx in [self.min.0, self.max.0].iter() {
                for by in [self.min.1, self.max.1].iter() {
                    for bz in [self.min.2, self.max.2].iter() {
                        let d = distance3((*bx, *by, *bz), (x, y, z));
                        best_dist = min(best_dist, d);
                    }
                }
            }
            best_dist
        }
    }
}
