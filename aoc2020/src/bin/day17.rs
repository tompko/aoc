use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Board2d {
    map: HashSet<(isize, isize, isize)>
}

struct Board3d {
    map: HashSet<(isize, isize, isize, isize)>
}

impl Board2d {
    fn new() -> Self{
        Board2d {
            map: HashSet::new(),
        }
    }

    fn add(&mut self, x: isize, y: isize, z: isize) {
        self.map.insert((x, y, z));
    }

    fn step(&mut self) {
        let mut to_consider = HashSet::new();
        for coords in self.map.iter() {
            for point in Board2d::neighbours(*coords) {
                to_consider.insert(point);
            }
            to_consider.insert(*coords);
        }

        let mut next_map = HashSet::new();
        for point in to_consider {
            let count = Board2d::neighbours(point).iter().filter(|n| self.map.contains(n)).count();
            if self.map.contains(&point) {
                if count == 2 || count == 3 {
                    next_map.insert(point);
                }
            } else {
                if count == 3 {
                    next_map.insert(point);
                }
            }
        }

        self.map = next_map;
    }

    fn neighbours(point: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        let mut ret = Vec::new();
        let (x, y, z) = point;

        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue
                    }
                    ret.push((x + dx, y + dy, z + dz));
                }
            }
        }

        ret
    }
}

impl Board3d {
    fn new() -> Self{
        Board3d {
            map: HashSet::new(),
        }
    }

    fn add(&mut self, x: isize, y: isize, z: isize, w: isize) {
        self.map.insert((x, y, z, w));
    }

    fn step(&mut self) {
        let mut to_consider = HashSet::new();
        for coords in self.map.iter() {
            for point in Board3d::neighbours(*coords) {
                to_consider.insert(point);
            }
            to_consider.insert(*coords);
        }

        let mut next_map = HashSet::new();
        for point in to_consider {
            let count = Board3d::neighbours(point).iter().filter(|n| self.map.contains(n)).count();
            if self.map.contains(&point) {
                if count == 2 || count == 3 {
                    next_map.insert(point);
                }
            } else {
                if count == 3 {
                    next_map.insert(point);
                }
            }
        }

        self.map = next_map;
    }

    fn neighbours(point: (isize, isize, isize, isize)) -> Vec<(isize, isize, isize, isize)> {
        let mut ret = Vec::new();
        let (x, y, z, w) = point;

        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    for dw in -1..2 {
                        if dx == 0 && dy == 0 && dz == 0 && dw == 0{
                            continue
                        }
                        ret.push((x + dx, y + dy, z + dz, w + dw));
                    }
                }
            }
        }
        ret
    }
}

fn main() {
    let mut board2d = Board2d::new();
    let mut board3d = Board3d::new();

    let file = File::open("input/day17.txt").expect("Failed to open input");
    let file = BufReader::new(&file);
    for (y, line) in file.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                board2d.add(x as isize, y as isize, 0);
                board3d.add(x as isize, y as isize, 0, 0);
            }
        }
    }

    for _ in 0..6 {
        board2d.step();
        board3d.step();
    }

    println!("Part 1: {}", board2d.map.len());
    println!("Part 2: {}", board3d.map.len());
}