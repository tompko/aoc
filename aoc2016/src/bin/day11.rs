extern crate astar;

use std::cmp::min;
use std::vec::IntoIter;
use astar::{SearchProblem, astar};

struct Problem {
    start: Vec<(u32, u32)>,
}

impl Problem {
    fn new(start: Vec<(u32, u32)>) -> Self {
        Problem{
            start: start,
        }
    }
}

impl SearchProblem for Problem {
    type Node = State;
    type Cost = u32;
    type Iter = IntoIter<(State, u32)>;

    fn start(&self) -> State {
        let mut start = State{
            state: self.start.clone(),
            floor: 1,
        };
        start.state.sort();
        start
    }

    fn is_end(&self, state: &State) -> bool {
        state.state.iter().all(|&x| x == (4, 4))
    }

    fn heuristic(&self, state: &State) -> u32 {
        let mut floors = vec![0; 4];
        for &(a, b) in state.state.iter() {
            floors[a as usize - 1] += 1;
            floors[b as usize - 1] += 1;
        }

        min(
            (floors[0]*3 + floors[1]*2 + floors[2]) / 2,
            1,
        )
    }

    fn neighbors(&mut self, state: &State) -> IntoIter<(State, u32)> {
        state.neighbours().into_iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    state: Vec<(u32, u32)>,
    floor: u32,
}

impl State {
    fn is_valid(&self) -> bool {
        let mut generators = vec![false; 5];

        for &(a, _) in self.state.iter() {
            generators[a as usize] = true;
        }

        for &(a, b) in self.state.iter() {
            if a == b { continue; }
            if generators[b as usize] {
                return false;
            }
        }

        true
    }

    fn neighbours(&self) -> Vec<(State, u32)> {
        let mut ret = Vec::new();

        for (i, &a) in self.state.iter().enumerate() {
            if a.0 != self.floor && a.1 != self.floor {
                continue
            }

            if a.0 == a.1 {
                if a.0 < 4 {
                    let mut new_state = self.clone();
                    new_state.state.remove(i);
                    new_state.state.push((a.0+1, a.1+1));
                    new_state.state.sort();
                    new_state.floor += 1;
                    if new_state.is_valid() {
                        ret.push((new_state, 1));
                    }
                }

                if a.0 > 1 {
                    let mut new_state = self.clone();
                    new_state.state.remove(i);
                    new_state.state.push((a.0-1, a.1-1));
                    new_state.state.sort();
                    new_state.floor -= 1;
                    if new_state.is_valid() {
                        ret.push((new_state, 1));
                    }
                }
            }

            if a.0 == self.floor {
                if a.0 < 4 {
                    let mut new_state = self.clone();
                    new_state.state.remove(i);
                    new_state.state.push((a.0+1, a.1));
                    new_state.state.sort();
                    new_state.floor += 1;
                    if new_state.is_valid() {
                        ret.push((new_state, 1));
                    }
                }

                if a.0 > 1 {
                    let mut new_state = self.clone();
                    new_state.state.remove(i);
                    new_state.state.push((a.0-1, a.1));
                    new_state.state.sort();
                    new_state.floor -= 1;
                    if new_state.is_valid() {
                        ret.push((new_state, 1));
                    }
                }
            }

            if a.1 == self.floor {
                if a.1 < 4 {
                    let mut new_state = self.clone();
                    new_state.state.remove(i);
                    new_state.state.push((a.0, a.1+1));
                    new_state.state.sort();
                    new_state.floor += 1;
                    if new_state.is_valid() {
                        ret.push((new_state, 1));
                    }
                }

                if a.1 > 1 {
                    let mut new_state = self.clone();
                    new_state.state.remove(i);
                    new_state.state.push((a.0, a.1-1));
                    new_state.state.sort();
                    new_state.floor -= 1;
                    if new_state.is_valid() {
                        ret.push((new_state, 1));
                    }
                }
            }

            for (j, &b) in self.state.iter().enumerate() {
                if j <= i { continue }
                if b.0 != self.floor && b.1 != self.floor {
                    continue
                }

                if a.0 == b.0 && a.0 == self.floor {
                    if a.0 < 4 {
                        let mut new_state = self.clone();
                        new_state.state.remove(j);
                        new_state.state.remove(i);
                        new_state.state.push((a.0+1, a.1));
                        new_state.state.push((b.0+1, b.1));
                        new_state.state.sort();
                        new_state.floor += 1;
                        if new_state.is_valid() {
                            ret.push((new_state, 1));
                        }
                    }

                    if a.0 > 1 {
                        let mut new_state = self.clone();
                        new_state.state.remove(j);
                        new_state.state.remove(i);
                        new_state.state.push((a.0-1, a.1));
                        new_state.state.push((b.0-1, b.1));
                        new_state.state.sort();
                        new_state.floor -= 1;
                        if new_state.is_valid() {
                            ret.push((new_state, 1));
                        }
                    }
                }

                if a.1 == b.1 && a.1 == self.floor {
                    if a.1 < 4 {
                        let mut new_state = self.clone();
                        new_state.state.remove(j);
                        new_state.state.remove(i);
                        new_state.state.push((a.0, a.1+1));
                        new_state.state.push((b.0, b.1+1));
                        new_state.state.sort();
                        new_state.floor += 1;
                        if new_state.is_valid() {
                            ret.push((new_state, 1));
                        }
                    }

                    if a.1 > 1 {
                        let mut new_state = self.clone();
                        new_state.state.remove(j);
                        new_state.state.remove(i);
                        new_state.state.push((a.0, a.1-1));
                        new_state.state.push((b.0, b.1-1));
                        new_state.state.sort();
                        new_state.floor -= 1;
                        if new_state.is_valid() {
                            ret.push((new_state, 1));
                        }
                    }
                }
            }
        }

        ret
    }
}

fn main() {
    let mut part1 = Problem::new(vec![(1, 2), (1, 1), (1, 2), (1, 1), (1, 1)]);
    let path1 = astar(&mut part1).unwrap();

    println!("1: {}", path1.len() - 1);

    let mut part2 = Problem::new(vec![(1, 2), (1, 1), (1, 2), (1, 1), (1, 1), (1,1), (1,1)]);
    let path2 = astar(&mut part2).unwrap();

    println!("2: {}", path2.len() - 1);
}
