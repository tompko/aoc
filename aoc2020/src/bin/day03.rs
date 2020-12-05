use std::fs::File;
use std::io::{BufRead, BufReader};

fn slope(board: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < board.len() {
        let row = &board[y];
        if row[x] {
            trees += 1;
        }
        x += dx;
        y += dy;
        if x >= row.len() {
            x -= row.len();
        }
    }

    trees
}

fn main() {
    let file = File::open("input/day03.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let board: Vec<Vec<bool>> = file.lines().map(|x| x.unwrap().chars().map(|c| c == '#').collect()).collect();

    let part1 = slope(&board, 3, 1);
    let part2 = slope(&board, 1, 1) * slope(&board, 3, 1) * slope(&board, 5, 1) * slope(&board, 7, 1) * slope(&board, 1, 2);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}