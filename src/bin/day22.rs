extern crate astar;
extern crate regex;

use std::cmp::max;
use std::io::{BufReader, BufRead};
use std::fs::File;
use astar::{SearchProblem, astar};
use regex::Regex;

struct Cluster {
    nodes: HashMap<(usize, usize), Node>,
    x: usize,
    y: usize,
}

impl Cluster {
    fn to_problem(&self) -> Problem {
        let mut maze = Vec::new();
        let mut empty = (0,0);
        for y in 0..self.y {
            let mut row = Vec::new();
            for x in 0..self.x {
                let ref node = self.nodes[&(x, y)];
                if node.used > 200 {
                    row.push(false);
                } else {
                    row.push(true);
                }
                if node.used == 0 {
                    empty = (x, y);
                }
            }
            maze.push(row);
        }
        Problem{
            cells: maze,
            empty: empty,
        }
    }
}

struct Problem {
    cells: Vec<Vec<bool>>,
    empty: (usize, usize),
}

impl SearchProblem for Problem {
    type Node = State;
    type Cost = usize;
    type Iter = std::vec::IntoIter<(State, usize)>;

    fn start(&self) -> State {
        State{
            target: (self.cells[0].len()-1, 0),
            empty: self.empty.clone(),
        }
    }

    fn is_end(&self, state: &State) -> bool {
        state.target == (0, 0)
    }

    fn heuristic(&self, state: &State) -> usize {
        abssub(state.target.0, state.empty.0) +
        abssub(state.target.1, state.empty.1) +
        state.target.0 + state.target.1 - 1
    }

    fn neighbors(&mut self, state: &State) -> std::vec::IntoIter<(State, usize)> {
        let mut ret = Vec::new();
        if state.empty.0 > 0 && self.cells[state.empty.1][state.empty.0-1] {
            ret.push((state.step(-1, 0), 1));
        }
        if state.empty.0 < self.cells[0].len() - 1 && self.cells[state.empty.1][state.empty.0+1] {
            ret.push((state.step(1, 0), 1));
        }

        if state.empty.1 > 0 && self.cells[state.empty.1-1][state.empty.0] {
            ret.push((state.step(0, -1), 1));
        }
        if state.empty.1 < self.cells.len() - 1 && self.cells[state.empty.1+1][state.empty.0] {
            ret.push((state.step(0, 1), 1));
        }

        ret.into_iter()
    }
}

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    used: u32,
    avail: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct State{
    target: (usize, usize),
    empty: (usize, usize),
}

impl State {
    fn step(&self, x: i32, y: i32) -> State {
        let new_empty = ((self.empty.0 as i32 + x) as usize, (self.empty.1 as i32 + y) as usize);
        let new_target = if new_empty == self.target { self.empty } else { self.target };
        State {
            target: new_target,
            empty: new_empty,
        }
    }
}

fn parse(reader: &mut BufRead) -> Cluster {
    let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+\d+T\s+(\d+)T\s+(\d+)T\s+\d+%").unwrap();
    let mut hm = HashMap::new();
    let mut mx = 0;
    let mut my = 0;

    for line in reader.lines().skip(2) {
        let line = line.unwrap();
        let caps = re.captures(line.trim()).unwrap();

        let x = caps.at(1).unwrap().parse().unwrap();
        let y = caps.at(2).unwrap().parse().unwrap();
        let used = caps.at(3).unwrap().parse().unwrap();
        let avail = caps.at(4).unwrap().parse().unwrap();

        hm.insert((x,y), Node{
            x: x,
            y: y,
            used: used,
            avail: avail,
        });
        mx = max(mx, x + 1);
        my = max(my, y + 1);
    }

    Cluster{
        nodes: hm,
        x: mx,
        y: my,
    }
}

fn solve(mut problem: Problem) -> u32 {
    println!("{:?}", problem.start());
    let path = astar(&mut problem);

    path.unwrap().len() as u32 - 1
}

fn main() {
    let file = File::open("input/day22.in").expect("Failed to open input");
    let mut reader = BufReader::new(&file);

    let cluster = parse(&mut reader);
    let mut count = 0;

    for node_a in cluster.nodes.values() {
        for node_b in cluster.nodes.values() {
            if node_a.used == 0 {
                continue;
            }

            if node_a.x == node_b.x && node_a.y == node_b.y {
                continue;
            }

            if node_a.used <= node_b.avail {
                count += 1;
            }
        }
    }

    println!("1: {}", count);

    let part2 = solve(cluster.to_problem());
    println!("2: {}", part2);
}

fn abssub(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
