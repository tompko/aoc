use std::cmp::max;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

const DEPTH: usize = 6084;
const TARGET: (usize, usize) = (14, 709);
const MOD: usize = 20183;

const ROCKY: usize = 0;
const WET: usize = 1;
const NARROW: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    Climbing,
    Neither,
}

struct Game {
    erosion: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy)]
struct State {
    pos: (usize, usize),
    tool: Tool,
    cost: usize,
}

impl Game {
    fn search(&self, target: &State) -> Option<usize> {
        let mut paths = Vec::new();
        let mut closed = HashSet::new();
        paths.push(State::new());

        while paths.len() > 0 {
            let current = paths.pop().unwrap();
            if !closed.insert(current) {
                continue;
            }

            for n in self.neighbours(current) {
                if self.is_goal(&n) {
                    return Some(n.cost);
                }
                paths.push(n);
            }

            paths.sort_by(|s, t| (t.cost + Game::distance(t, target)).cmp(&(s.cost + Game::distance(s, target))));
        }

        None
    }

    fn neighbours(&self, s: State) -> Vec<State> {
        let mut ret = Vec::new();

        match (s.tool, self.risk(s.pos)) {
            (Tool::Torch, ROCKY) => ret.push(s.with_tool(Tool::Climbing)),
            (Tool::Climbing, ROCKY) => ret.push(s.with_tool(Tool::Torch)),
            (Tool::Climbing, WET) => ret.push(s.with_tool(Tool::Neither)),
            (Tool::Neither, WET) => ret.push(s.with_tool(Tool::Climbing)),
            (Tool::Torch, NARROW) => ret.push(s.with_tool(Tool::Neither)),
            (Tool::Neither, NARROW) => ret.push(s.with_tool(Tool::Torch)),
            _ => panic!("Bad state: {:?} {}", s.tool, self.risk(s.pos)),
        }

        for delta in [(1, 0), (-1, 0), (0, 1), (0, -1)].into_iter() {
            let x = (s.pos.0 as isize + delta.0);
            if x < 0 {
                continue
            }
            let y = (s.pos.1 as isize + delta.1);
            if y < 0 {
                continue
            }
            let new_pos = (x as usize, y as usize);
            if Game::is_valid_tool(self.risk(new_pos), s.tool) {
                ret.push(s.to_pos(new_pos));
            }
        }

        ret
    }

    fn distance(s: &State, t: &State) -> usize {
        let x = if s.pos.0 > t.pos.0 { s.pos.0 - t.pos.0 } else { t.pos.0 - s.pos.0 };
        let y = if s.pos.1 > t.pos.1 { s.pos.1 - t.pos.1 } else { t.pos.1 - s.pos.1 };
        let s = if s.tool == t.tool { 0 } else { 7 };
        x + y + s
    }

    fn is_goal(&self, s: &State) -> bool {
        s.pos == TARGET && s.tool == Tool::Torch
    }

    fn risk(&self, pos: (usize, usize)) -> usize {
        self.erosion[pos.1][pos.0] % 3
    }

    fn is_valid_tool(risk: usize, tool: Tool) -> bool {
        match (tool, risk) {
            (Tool::Torch, ROCKY) => true,
            (Tool::Climbing, ROCKY) => true,
            (Tool::Climbing, WET) => true,
            (Tool::Neither, WET) => true,
            (Tool::Torch, NARROW) => true,
            (Tool::Neither, NARROW) => true,
            _ => false,
        }
    }
}

impl State {
    fn new() -> Self {
        State{ pos: (0, 0), tool: Tool::Torch, cost: 0 }
    }

    fn with_tool(&self, t: Tool) -> Self {
        State {
            pos: self.pos,
            tool: t,
            cost: self.cost + 7,
        }
    }

    fn to_pos(&self, pos: (usize, usize)) -> Self {
        State {
            pos: pos,
            tool: self.tool,
            cost: self.cost + 1,
        }
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.tool.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (self.pos == other.pos) && (self.tool == other.tool)
    }
}

impl Eq for State {}

fn main() {
    let mut erosion: Vec<Vec<usize>> = Vec::new();
    let limit = max(TARGET.0, TARGET.1) * 3;

    for y in 0..limit {
        let mut line = Vec::with_capacity(limit);
        for x in 0..limit {
            if x ==0 && y == 0 {
                line.push(DEPTH % MOD);
            } else if (x, y) == TARGET {
                line.push(DEPTH % MOD);
            } else if x == 0 {
                line.push(((y * 48271) + DEPTH) % MOD);
            } else if y == 0 {
                line.push(((x * 16807) + DEPTH) % MOD);
            } else {
                let geo = line[x-1] * erosion[y-1][x];
                line.push((geo + DEPTH) % MOD);
            }
        }
        erosion.push(line);
    }

    let mut risk = 0;
    for y in 0..=TARGET.1 {
        for x in 0..=TARGET.0 {
            risk += erosion[y][x] % 3;
        }
    }

    let g = Game{ erosion };
    let part2 = g.search(&State{ pos: TARGET, tool: Tool::Torch, cost: 0 }).unwrap();

    println!("part 1: {}", risk);
    println!("part 2: {}", part2);
}
