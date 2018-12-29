use std::fs::File;
use std::io::Read;
use std::collections::{HashSet, HashMap};
use std::cmp::min;

struct Maze {
    edges: HashMap<(i64, i64), HashSet<(i64, i64)>>,
}

impl Maze {
    fn new() -> Self {
        Maze{ edges: HashMap::new() }
    }

    fn add_edge(&mut self, a: (i64, i64), b: (i64, i64)) {
        self.edges.entry(a).or_default().insert(b);
        self.edges.entry(b).or_default().insert(a);
    }

    fn dijkstra(&self, origin: (i64, i64)) -> HashMap<(i64, i64), u64> {
        let mut unvisited: HashSet<(i64, i64)> = self.edges.keys().map(|&k| k).collect();
        let mut distances: HashMap<_, _> = self.edges.keys().map(|&k| (k, std::u64::MAX)).collect();
        distances.insert(origin, 0);
        let mut current = origin;

        while !unvisited.is_empty() {
            unvisited.remove(&current);
            let base = *distances.get(&current).unwrap();

            for connected in self.edges.get(&current).unwrap().iter() {
                let d = min(base + 1, *distances.get(connected).unwrap());
                distances.insert(*connected, d);
            }

            let mut next_best = current;
            let mut best_dist = std::u64::MAX;

            for u in unvisited.iter() {
                if *distances.get(u).unwrap() < best_dist {
                    next_best = *u;
                    best_dist = *distances.get(u).unwrap();
                }
            }

            current = next_best;
        }

        distances
    }
}

fn main() {
    let mut file = File::open("input/day20.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");

    let mut positions: HashSet<(i64, i64)> = HashSet::new();
    positions.insert((0, 0));
    let mut stack = Vec::new();
    let mut starts = HashSet::new();
    starts.insert((0, 0));
    let mut ends: HashSet<(i64, i64)> = HashSet::new();

    let mut maze = Maze::new();

    for c in contents.chars() {
        match c {
            '|' => {
                for p in positions.iter() {
                    ends.insert(*p);
                }
                positions = starts.clone();
            },
            'N' | 'S' | 'E' | 'W' => {
                let d = match c {
                    'N' => (0, -1),
                    'S' => (0, 1),
                    'E' => (1, 0),
                    'W' => (-1, 0),
                    _ => unreachable!(),
                };
                for p in positions.iter() {
                    let np = (p.0 + d.0, p.1 + d.1);
                    maze.add_edge(*p, np);
                }
                positions = positions.into_iter().map(|p| (p.0 + d.0, p.1 + d.1)).collect();
            }
            '(' => {
                stack.push((starts, ends));
                starts = positions.clone();
                ends = HashSet::new();
            }
            ')' => {
                for e in ends.into_iter() {
                    positions.insert(e);
                }
                let (s, e) = stack.pop().unwrap();
                starts = s;
                ends = e
            }
            _ => (),
        }
    }

    let path_lengths = maze.dijkstra((0,0));

    let part1 = path_lengths.values().max().unwrap();
    let part2 = path_lengths.values().filter(|&&x| x >= 1000).count();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
