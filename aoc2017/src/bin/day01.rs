use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input/day01.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    println!("part 1: {}", solve(contents, 1));
    println!("part 2: {}", solve(contents, contents.len() / 2));
}

fn solve(input: &str, skip: usize) -> u32 {
    input.chars().
    zip(input.chars().cycle().skip(skip)).
    filter_map(|(x, y)| if x == y { x.to_digit(10) } else { None }).
    sum()
}
