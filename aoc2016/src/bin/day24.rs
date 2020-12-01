extern crate permutohedron;

use std::cmp::min;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{VecDeque, HashMap, HashSet};
use permutohedron::Heap;

struct Maze {
    maze: Vec<Vec<char>>,
    locations: HashMap<char, (usize, usize)>,
}

impl Maze {
    fn parse(reader: &mut BufRead) -> Self {
        let mut maze = Vec::new();
        let mut locations = HashMap::new();

        for (y, line) in reader.lines().enumerate() {
            let line = line.unwrap();

            let line: Vec<_> = line.chars().collect();

            for (x, cell) in line.iter().enumerate() {
                if cell.is_digit(10) {
                    locations.insert(*cell, (x, y));
                }
            }

            maze.push(line);
        }

        Maze{
            maze: maze,
            locations: locations,
        }
    }

    fn is_wall(&self, x: usize, y: usize) -> bool {
        self.maze[y][x] == '#'
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut ret = Vec::new();

        if x > 0 && !self.is_wall(x-1, y) {
            ret.push((x-1, y));
        }
        if y > 0 && !self.is_wall(x, y-1) {
            ret.push((x, y-1));
        }
        if x < self.maze[0].len() && !self.is_wall(x+1, y) {
            ret.push((x+1, y));
        }
        if y < self.maze.len() && !self.is_wall(x, y+1) {
            ret.push((x, y+1));
        }
        ret
    }

    fn get_distances(&self) -> HashMap<(char, char), u32> {
        let mut ret = HashMap::new();
        for (start, &(start_x, start_y)) in &self.locations {
            let mut frontier = VecDeque::new();
            frontier.push_front((start_x, start_y, 0));
            let mut seen = HashSet::new();

            while let Some(curr) = frontier.pop_back() {
                let (x, y, steps) = curr;

                if seen.contains(&(x, y)) {
                    continue
                }
                seen.insert((x, y));

                for (dx, dy) in self.get_neighbours(x, y) {
                    frontier.push_front((dx, dy, steps + 1));
                }

                let cell = self.maze[y][x];
                if cell.is_digit(10) {
                    ret.insert((*start, cell), steps);
                }
            }
        }

        ret
    }

    fn get_non_zero(&self) -> Vec<char> {
        self.locations.keys().filter(|&&x| x != '0').cloned().collect()
    }

    fn get_area(&self) -> u32 {
        (self.maze[0].len() as u32) * (self.maze.len() as u32)
    }
}

fn main() {
    let file = File::open("input/day24.in").expect("Failed to open input");
    let mut reader = BufReader::new(file);

    let maze = Maze::parse(&mut reader);
    let distances = maze.get_distances();

    let mut data = maze.get_non_zero();
    let heap = Heap::new(&mut data);

    let mut best_distance = maze.get_area();

    for data in heap {
        let mut curr = '0';

        let mut dist = 0;
        for next in &data {
            let next = *next;
            dist += distances[&(curr, next)];
            curr = next;
        }
        best_distance = min(dist, best_distance);
    }

    println!("1: {}", best_distance);

    best_distance = maze.get_area();
    let mut data = maze.get_non_zero();
    let heap = Heap::new(&mut data);

    for data in heap {
        let mut curr = '0';

        let mut dist = 0;
        for next in &data {
            let next = *next;
            dist += distances[&(curr, next)];
            curr = next;
        }
        dist += distances[&(curr, '0')];

        best_distance = min(dist, best_distance);
    }

    println!("2: {}", best_distance);
}
