extern crate regex;

use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;

#[derive(Clone)]
struct Cluster {
    nodes: HashMap<(usize, usize), Node>,
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Node {
    x: usize,
    y: usize,
    used: u32,
    avail: u32,
}

struct State{
    target: (usize, usize),
    cluster: Cluster,
    steps: u32,
}

impl State {
    fn neighbours(&self) -> Vec<Self> {
        let mut ret = Vec::new();
        let nodes = &self.cluster.nodes;

        for node in nodes.values() {
            if node.used == 0 {
                continue
            }
            if node.x > 0 {
                let node_b = nodes.get(&(node.x-1, node.y)).unwrap();
                if node.used < node_b.avail {
                    ret.push(self.move_node(node.x, node.y, node_b.x, node_b.y));
                }
            }
            if node.y > 0 {
                let node_b = nodes.get(&(node.x, node.y-1)).unwrap();
                if node.used < node_b.avail {
                    ret.push(self.move_node(node.x, node.y, node_b.x, node_b.y));
                }
            }
            if node.x < self.cluster.x - 1{
                let node_b = nodes.get(&(node.x+1, node.y)).unwrap();
                if node.used < node_b.avail {
                    ret.push(self.move_node(node.x, node.y, node_b.x, node_b.y));
                }
            }
            if node.y < self.cluster.y - 1 {
                let node_b = nodes.get(&(node.x, node.y+1)).unwrap();
                if node.used < node_b.avail {
                    ret.push(self.move_node(node.x, node.y, node_b.x, node_b.y));
                }
            }
        }

        ret
    }

    fn move_node(&self, fx: usize, fy: usize, sx: usize, sy: usize) -> Self {
        let mut cluster = self.cluster.clone();

        let node_a = cluster.nodes.get(&(fx, fy)).unwrap().clone();
        let node_b = cluster.nodes.get(&(sx, sy)).unwrap().clone();

        let target = if self.target == (sx, sy) {
            (fx, fy)
        } else {
            self.target
        };

        cluster.nodes.insert((node_a.x, node_a.y), Node{
            x: node_a.x,
            y: node_a.y,
            used: 0,
            avail: node_a.avail + node_a.used,
        });
        cluster.nodes.insert((node_b.x, node_b.y), Node{
            x: node_b.x,
            y: node_b.y,
            used: node_b.used + node_a.used,
            avail: node_b.avail - node_a.used,
        });

        State{
            target: target,
            cluster: cluster,
            steps: self.steps + 1,
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

fn solve(cluster: Cluster) -> u32 {
    let mut states = VecDeque::new();
    states.push_front(State{
        target: (cluster.x - 1, 0),
        cluster: cluster,
        steps: 0,
    });

    while let Some(state) = states.pop_back() {
        println!("{} {}", state.steps, states.len());
        if state.target == (0, 0) {
            return state.steps;
        }

        for n in state.neighbours() {
            states.push_front(n);
        }
    }

    unreachable!();
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

    let part2 = solve(cluster);
    println!("2: {}", part2);
}
