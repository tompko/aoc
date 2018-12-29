use std::cmp::Ordering;
use std::collections::{HashSet, HashMap, VecDeque};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Team {
    Elf,
    Goblin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Unit {
    team: Team,
    pos: Point,
    hp: usize,
}

struct Board {
    grid: Vec<Vec<char>>,
    units: Vec<Unit>,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }

    fn neighbours(&self) -> Vec<Point> {
        [(0, -1), (-1, 0), (1, 0), (0, 1)].into_iter()
            .map(|(dx, dy)| Point{ x: self.x + dx, y: self.y + dy })
            .collect()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y == other.x {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Unit {
    fn new(t: Team, p: Point) -> Self {
        Unit{ team: t, pos: p, hp: 200 }
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Unit) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Unit) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.team {
            Team::Goblin => write!(f, "G({})", self.hp),
            Team::Elf => write!(f, "E({})", self.hp),
        }
    }
}

impl Board {
    fn new<T: AsRef<Path>>(p: T) -> Self {
        let file = File::open(p).expect("Failed to open input");
        let file = BufReader::new(&file);
        let mut grid = Vec::new();
        let mut units = Vec::new();

        for (y, line) in file.lines().enumerate() {
            let line = line.unwrap();
            let mut row = Vec::new();
            let y = y as isize;

            for (x, cell) in line.chars().enumerate() {
                let x = x as isize;
                match cell {
                    'G' => { row.push('.'); units.push(Unit::new(Team::Goblin, Point::new(x, y))); },
                    'E' => { row.push('.'); units.push(Unit::new(Team::Elf, Point::new(x, y))); },
                    _ => row.push(cell),
                }
            }

            grid.push(row);
        }

        Board { grid, units }
    }

    fn at(&self, p: &Point) -> char {
        self.grid[p.y as usize][p.x as usize]
    }

    fn play(&mut self) -> usize {
        let mut rounds = 0;
        loop {
            if self.play_round() {
                break;
            }
            rounds += 1;
        }
        rounds * self.units.iter().map(|u| u.hp).sum::<usize>()
    }

    fn play_round(&mut self) -> bool {
        self.units.sort();

        for u in 0..self.units.len() {
            if self.units[u].hp > 0 {
                let no_target = self.unit_round(u);
                if no_target {
                    return true;
                }
            }
        }

        false
    }

    fn unit_round(&mut self, u: usize) -> bool {
        let targets = self.units.iter().enumerate()
            .filter(|&(_, e)| e.hp > 0 && e.team != self.units[u].team)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let occupied = self.units.iter()
            .map(|u| u.pos)
            .filter(|&p| p != self.units[u].pos)
            .collect::<HashSet<_>>();

        if targets.is_empty() {
            return true;
        }

        let in_range = targets.iter()
            .map(|&t| self.units[t].pos.neighbours())
            .flatten()
            .filter(|p| !occupied.contains(p) && self.at(p) != '#')
            .collect::<HashSet<_>>();

        if !in_range.contains(&self.units[u].pos) {
            if let Some(m) = self.find_move(self.units[u].pos, &in_range, &occupied) {
                self.units[u].pos = m;
            }

        }

        let mut opponents = targets.into_iter()
            .filter(|&t| self.units[u].pos.neighbours().iter().any(|&n| n == self.units[t].pos))
            .collect::<Vec<_>>();

        if !opponents.is_empty() {
            opponents.sort_by_key(|&o| (self.units[o].hp, self.units[o].pos));
            let t = opponents[0];

            self.units[t].hp = self.units[t].hp.saturating_sub(3);
        }
        false
    }

    fn find_move(&self, position: Point, targets: &HashSet<Point>, occupied: &HashSet<Point>) -> Option<Point> {
        let mut visiting = VecDeque::new();
        visiting.push_front((position, 0));
        let mut paths = HashMap::new();
        paths.insert(position, (0, position));
        let mut seen = HashSet::new();

        while let Some((p, d)) = visiting.pop_back() {
            for n in p.neighbours() {
                if self.at(&n) == '#' || occupied.contains(&n) {
                    continue;
                }
                if !paths.contains_key(&n) || paths[&n].0 > d + 1 {
                    paths.insert(n, (d + 1, p));
                }
                if seen.contains(&n) {
                    continue;
                }
                if !visiting.iter().any(|v| v.0 == n) {
                    visiting.push_front((n, d + 1));
                }
            }
            seen.insert(p);
        };

        let mut candidates = targets.iter().filter_map(|t| paths.get(t)).collect::<Vec<_>>();
        candidates.sort();

        return if let Some((_, closest)) = candidates.iter().next() {
            let mut closest = *closest;
            while paths[&closest].0 > 1 {
                closest = paths[&closest].1;
            }
            Some(closest)
        } else {
            None
        };
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut units = self.units.clone();
        let mut index = 0;
        units.sort();

        for (y, line) in self.grid.iter().enumerate() {
            for (x, cell) in line.iter().enumerate() {
                let p = Point::new(x as isize, y as isize);
                if index < units.len() && units[index].pos == p && units[index].hp > 0 {
                    match units[index].team {
                        Team::Goblin => write!(f, "G")?,
                        Team::Elf => write!(f, "E")?,
                    }
                    index += 1;
                } else {
                    write!(f, "{}", self.at(&p))?;
                }
            }

            for u in units.iter() {
                if u.hp > 0 && u.pos.y == y as isize {
                    write!(f, " {:?}", u);
                }
            }

            write!(f, "\n");
        }
        Ok(())
    }
}

fn main() {
    let mut board = Board::new("input/day15.in");

    // let rounds = board.play();
    println!("Initially:");
    println!("{:?}", board);
    board.play_round();
    println!();
    println!("After 1 round:");
    println!("{:?}", board);

    // println!("part 1: {}", rounds);
}
