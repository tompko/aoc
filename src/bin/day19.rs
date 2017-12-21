use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day19.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut maze: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();

        maze.push(line.chars().filter(|&x| x != '\n').collect());
    }

    let mut coord: (i32, i32) = (0, 0);

    for (i, c) in maze[0].iter().enumerate() {
        if *c == '|' {
            coord = (i as i32, 0);
        }
    }

    let mut dir: (i32, i32) = (0, 1);
    let mut part1 = String::new();
    let mut part2 = 1;

    loop {
        let curr = maze[coord.1 as usize][coord.0 as usize];
        let next_coord = (coord.0 + dir.0, coord.1 + dir.1);
        let next = maze[next_coord.1 as usize][next_coord.0 as usize];

        if next.is_alphabetic() {
            part1.push(next);
            coord = next_coord;
            part2 += 1;
        } else if next == ' ' && curr == '+' {
            for n in rotate(dir) {
                let cand_coord = (coord.0 + n.0, coord.1 + n.1);
                let cand = maze[cand_coord.1 as usize][cand_coord.0 as usize];

                if cand != ' ' {
                    dir = n;
                }
            }
        } else if next == ' ' {
            break
        } else {
            coord = next_coord;
            part2 += 1;
        }
    }

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn rotate(dir: (i32, i32)) -> Vec<(i32, i32)> {
    match dir {
        (0, 1) => vec![(1,0), (-1, 0)],
        (0, -1) => vec![(1,0), (-1, 0)],
        (1, 0) => vec![(0,1), (0, -1)],
        (-1, 0) => vec![(0,1), (0, -1)],
        _ => unreachable!(),
    }
}
