extern crate crypto;

use std::collections::VecDeque;
use crypto::md5::Md5;
use crypto::digest::Digest;

const INPUT: &'static str = "vkjiggvb";

struct State {
    x: u8,
    y: u8,
    path: String,
}

impl State {
    fn get_neighbours(&self) -> Vec<State> {
        let mut hasher = Md5::new();
        hasher.input(self.path.as_bytes());
        let hash: Vec<_> = hasher.result_str().chars().take(4).collect();
        let mut ret = Vec::new();

        if self.y != 0 && self.is_open(hash[0]) {
            ret.push(State{
                x: self.x,
                y: self.y - 1,
                path: self.path.clone() + "U",
            });
        }
        if self.y != 3 && self.is_open(hash[1]) {
            ret.push(State{
                x: self.x,
                y: self.y + 1,
                path: self.path.clone() + "D",
            });
        }

        if self.x != 0 && self.is_open(hash[2]) {
            ret.push(State{
                x: self.x - 1,
                y: self.y,
                path: self.path.clone() + "L",
            });
        }
        if self.x != 3 && self.is_open(hash[3]) {
            ret.push(State{
                x: self.x + 1,
                y: self.y,
                path: self.path.clone() + "R",
            });
        }
        ret
    }

    fn is_goal(&self) -> bool {
        self.x == 3 && self.y == 3
    }

    fn is_open(&self, door: char) -> bool {
        match door {
            'b'|'c'|'d'|'e'|'f' => true,
            _ => false
        }
    }
}

fn solve(base: &str) -> (String, String) {
    let mut potents = VecDeque::new();
    potents.push_back(State{
        x: 0,
        y: 0,
        path: base.to_owned(),
    });

    let mut shortest = None;
    let mut longest = String::new();

    while let Some(curr) = potents.pop_front() {
        if curr.is_goal() {
            if shortest.is_none() {
                // The first path we find will always be a shortest path
                shortest = Some(curr.path.clone());
            }
            // The last path we find will always be a longest path
            longest = curr.path.clone();

            continue
        }

        for n in curr.get_neighbours() {
            potents.push_back(n);
        }
    }

    (shortest.unwrap(), longest)
}

fn main() {
    let (short, long) = solve(INPUT);
    println!("{}", short.split_at(INPUT.len()).1);
    println!("{}", long.len() - INPUT.len());
}
