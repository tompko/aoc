use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet, HashMap};
use std::num::ParseIntError;
use std::str::FromStr;
use std::ops::AddAssign;
use reduce::Reduce;

#[derive(Debug, Clone, Copy)]
enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'R' => Direction::RIGHT,
            'L' => Direction::LEFT,
            'U' => Direction::UP,
            'D' => Direction::DOWN,
            _ => unreachable!(),
        }
    }
}

impl Into<Point> for Direction {
    fn into(self) -> Point {
        match self {
            Direction::RIGHT => Point{x: 1, y: 0},
            Direction::LEFT => Point{x: -1, y: 0},
            Direction::UP => Point{x: 0, y: 1},
            Direction::DOWN => Point{x: 0, y: -1},
        }
    }
}

#[derive(Debug)]
struct Wire {
    direction: Direction,
    length: usize,
}

impl FromStr for Wire {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.to_owned();
        let direction = s.remove(0).into();
        let length = s.parse::<usize>()?;
        Ok(Wire{ direction, length})
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

fn main() {
    let file = File::open("input/day03.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let paths: Vec<Vec<_>> = file.lines().map(
        |line| line.unwrap().split(',').map(|l| l.parse::<Wire>().unwrap()).collect()
    ).collect();

    let mut counts = Vec::new();
    for path in paths.iter() {
        let mut current = Point{x: 0, y: 0};
        let mut count_map = HashMap::new();
        let mut index = 1;
        for wire in path.iter() {
            let delta: Point = wire.direction.into();
            for _ in 0..wire.length {
                current += delta;
                count_map.entry(current).or_insert(index);
                index += 1;
            }
        }
        counts.push(count_map);
    }

    let collisions = counts.iter().
        map(|c| c.keys().cloned().collect::<HashSet<_>>()).
        reduce(|a, b| a.intersection(&b).cloned().collect()).
        unwrap();
    let part1 = collisions.iter().map(|p| p.x.abs() + p.y.abs()).min().unwrap();
    let part2 = collisions.iter().map(|p| counts.iter().map(|c| c.get(p).unwrap()).sum::<usize>()).min().unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
