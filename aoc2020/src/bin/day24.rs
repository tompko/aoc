use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Copy, Clone)]
enum Direction {
    E,
    Se,
    Sw,
    W,
    Nw,
    Ne,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Colour {
    Black,
    White,
}

struct Grid {
    cells: HashMap<(isize, isize, isize), Colour>
}

struct Cell {
    coord: (isize, isize, isize)
}

impl Grid {
    fn new() -> Self {
        Grid {
            cells: HashMap::new(),
        }
    }

    fn flip(&mut self, coord: (isize, isize, isize)) {
        if self.cells.contains_key(&coord) {
            let new_colour = match self.cells.get(&coord).unwrap() {
                Colour::Black => Colour::White,
                Colour::White => Colour::Black,
            };
            self.cells.insert(coord, new_colour);
        } else {
            self.cells.insert(coord, Colour::Black);
        }
    }

    fn count(&self, colour: Colour) -> usize {
        self.cells.values().filter(|v| **v == colour).count()
    }

    fn step(&mut self) {
        let mut to_consider: HashSet<(isize, isize, isize)> = HashSet::new();
        for (coord, colour) in self.cells.iter() {
            if *colour != Colour::Black {
                continue;
            }
            to_consider.insert(*coord);
            for c in self.neighbours(*coord) {
                to_consider.insert(c);
            }
        }

        let mut new_cells = HashMap::new();
        for coord in to_consider.into_iter() {
            let count = self.neighbours(coord).iter().filter(|c| *self.cells.get(c).unwrap_or(&Colour::White) == Colour::Black).count();
            if *self.cells.get(&coord).unwrap_or(&Colour::White) == Colour::Black {
                if count == 1 || count == 2 {
                    new_cells.insert(coord, Colour::Black);
                }
            } else {
                if count == 2 {
                    new_cells.insert(coord, Colour::Black);
                }
            }
        }
        self.cells = new_cells;
    }

    fn neighbours(&self, coord: (isize, isize, isize)) -> Vec<(isize, isize, isize)> {
        vec![
            (coord.0 + 1, coord.1 - 1, coord.2),
            (coord.0, coord.1 - 1, coord.2 + 1),
            (coord.0 - 1, coord.1, coord.2 + 1),
            (coord.0 - 1, coord.1 + 1, coord.2),
            (coord.0, coord.1 + 1, coord.2 - 1),
            (coord.0 + 1, coord.1, coord.2 - 1),
        ]
    }
}

impl Cell {
    fn new() -> Self {
        Cell {
            coord: (0, 0, 0),
        }
    }

    fn go(&mut self, dir: Direction) {
        self.coord = match dir {
            Direction::E => (self.coord.0 + 1, self.coord.1 - 1, self.coord.2),
            Direction::Se => (self.coord.0, self.coord.1 - 1, self.coord.2 + 1),
            Direction::Sw => (self.coord.0 - 1, self.coord.1, self.coord.2 + 1),
            Direction::W => (self.coord.0 - 1, self.coord.1 + 1, self.coord.2),
            Direction::Nw => (self.coord.0, self.coord.1 + 1, self.coord.2 - 1),
            Direction::Ne => (self.coord.0 + 1, self.coord.1, self.coord.2 - 1),
        }
    }
}
fn to_directions(s: &str) -> Vec<Direction> {
    let mut ret = Vec::new();
    let mut ds = s.chars();

    loop {
        let next = ds.next();
        if next.is_none() {
            break;
        }
        let next = next.unwrap();
        match next {
            'e' => { ret.push(Direction::E); },
            'w' => { ret.push(Direction::W); },
            'n' => {
                match ds.next().unwrap() {
                    'e' => { ret.push(Direction::Ne); },
                    'w' => { ret.push(Direction::Nw); },
                    _ => unreachable!(),
                }
            }
            's' => {
                match ds.next().unwrap() {
                    'e' => { ret.push(Direction::Se); },
                    'w' => { ret.push(Direction::Sw); },
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }


    ret
}

fn main() {
    let file = File::open("input/day24.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut grid = Grid::new();

    for line in file.lines() {
        let line = line.unwrap();
        let directions = to_directions(&line);
        let mut cell = Cell::new();
        for &d in directions.iter() {
            cell.go(d);
        }
        grid.flip(cell.coord);
    }
    let part1 = grid.count(Colour::Black);

    for _ in 1..101 {
        grid.step();
    }
    let part2 = grid.count(Colour::Black);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}