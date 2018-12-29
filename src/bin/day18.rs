use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

const WIDTH: usize = 50;
const HEIGHT: usize = 50;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cell {
    Open,
    Tree,
    Lumberyard,
}

fn main() {
    let file = File::open("input/day18.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut board = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();

        let mut row = Vec::new();
        for c in line.trim().chars() {
            match c {
                '.' => row.push(Cell::Open),
                '|' => row.push(Cell::Tree),
                '#' => row.push(Cell::Lumberyard),
                _ => panic!("Unrecognised character: {}", c),
            }
        }

        board.push(row);
    }

    let mut seen = HashMap::new();
    let mut scores = HashMap::new();
    let mut base = 0;
    let mut repeat = 0;

    for i in 0..10000 {
        let c = board.iter().flat_map(|x| x.iter()).map(|x| *x).collect();

        if let Some(index) = seen.get(&c) {
            base = *index;
            repeat = i - index;
            break;
        }

        let mut tree = 0;
        let mut lumber = 0;

        for row in board.iter() {
            for cell in row.iter() {
                match cell {
                    Cell::Open => (),
                    Cell::Tree => tree += 1,
                    Cell::Lumberyard => lumber += 1,
                }
            }
        }

        if i == 10 {
            println!("part 1: {}", tree * lumber);
        }

        seen.insert(c, i);
        scores.insert(i, tree * lumber);

        board = step(board);
    }

    let target = 1000000000;
    let index = ((target - base) % repeat) + base;

    println!("part 2: {}", scores.get(&index).unwrap());

}

fn step(board: Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    let mut ret = Vec::new();

    for (y, row) in board.iter().enumerate() {
        let mut new_row = Vec::new();
        for (x, cell) in row.iter().enumerate() {
            let (_, tree, lumber) = count_cells(&board, neighbours(x, y));
            match cell {
                Cell::Open => {
                    new_row.push(if tree >= 3 { Cell::Tree } else { Cell::Open });
                },
                Cell::Tree => {
                    new_row.push(if lumber >= 3 { Cell::Lumberyard } else { Cell::Tree });
                },
                Cell::Lumberyard => {
                    new_row.push(if lumber >= 1 && tree >= 1 { Cell::Lumberyard } else { Cell::Open });
                },
            };
        }
        ret.push(new_row);
    }

    ret
}

fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let x = x as isize;
    let y = y as isize;
    let mut ret = Vec::new();

    for dx in [-1, 0, 1].iter() {
        for dy in [-1, 0, 1].iter() {
            ret.push((x + *dx, y + *dy));
        }
    }

    ret.into_iter()
        .filter(|&(a, b)| a >= 0 && a < WIDTH as isize && b >= 0 && b < HEIGHT as isize)
        .filter(|&(a, b)| a != x || b != y)
        .map(|(a, b)| (a as usize, b as usize))
        .collect()
}

fn count_cells(board: &Vec<Vec<Cell>>, ns: Vec<(usize, usize)>) -> (usize, usize, usize) {
    let mut open = 0;
    let mut tree = 0;
    let mut lumber = 0;

    for n in ns.iter() {
        match board[n.1][n.0] {
            Cell::Open => open += 1,
            Cell::Tree => tree += 1,
            Cell::Lumberyard => lumber += 1,
        }
    }

    (open, tree, lumber)
}
