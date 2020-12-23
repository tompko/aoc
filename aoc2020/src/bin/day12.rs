use std::fs::File;
use std::io::{BufRead, BufReader};

struct Ship {
    facing: i32,
    x: i32,
    y: i32,
}

struct Waypoint {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

impl Ship {
    fn new() -> Self {
        Ship {
            facing: 90,
            x: 0,
            y: 0,
        }
    }

    fn go(&mut self, line: &str) {
        let (dir, dist) = line.split_at(1);
        let dist = dist.parse::<i32>().unwrap();

        match dir {
            "N" => { self.y += dist; },
            "S" => { self.y -= dist; },
            "E" => { self.x += dist; },
            "W" => { self.x -= dist; },
            "L" => { self.facing -= dist; },
            "R" => { self.facing += dist; },
            "F" => { match self.facing {
                0 => self.y += dist,
                90 => self.x += dist,
                180 => self.y -= dist,
                270 => self.x -= dist,
                _ => unreachable!(),
            }},
            _ => unreachable!(),
        }

        while self.facing < 0 {
            self.facing += 360;
        }
        while self.facing >= 360 {
            self.facing -= 360;
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Waypoint {
    fn new() -> Self {
        Waypoint {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }

    fn go(&mut self, line: &str) {
        let (dir, dist) = line.split_at(1);
        let dist = dist.parse::<i32>().unwrap();

        match dir {
            "N" => { self.wy += dist; },
            "S" => { self.wy -= dist; },
            "E" => { self.wx += dist; },
            "W" => { self.wx -= dist; },
            "L" => {
                for _ in 0..(dist/90) {
                    let wx = -self.wy;
                    let wy = self.wx;
                    self.wx = wx;
                    self.wy = wy;
                }
            },
            "R" => {
                for _ in 0..(dist/90) {
                    let wx = self.wy;
                    let wy = -self.wx;
                    self.wx = wx;
                    self.wy = wy;
                }
            },
            "F" => { 
                self.x += dist * self.wx;
                self.y += dist * self.wy;
            },
            _ => unreachable!(),
        }
    }

    fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let file = File::open("input/day12.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    for line in file.lines() {
        let line = line.unwrap();
        ship.go(&line);
        waypoint.go(&line);
    }

    let part1 = ship.distance();
    let part2 = waypoint.distance();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}