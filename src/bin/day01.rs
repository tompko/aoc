use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("inputs/day01.in").unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();

    let mut moves = contents.
        chars().
        map(|ch| match ch { ')' => -1,'(' => 1,_ => 0 }).
        scan(0, |state, f| { *state += f; Some(*state) });

    let basement = moves.position(|f| f == -1).unwrap() + 1;
    let floor = moves.last().unwrap();

    println!("1: {}", floor);
    println!("2: {}", basement);
}
