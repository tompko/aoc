use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
enum State {
    Group,
    Garbage,
}

fn main() {
    let mut file = File::open("input/day09.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let (part1, part2) = score(contents);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn score(input: &str) -> (u32, u32) {
    let mut stream = input.chars();
    let mut state = State::Group;
    let mut depth = 0;
    let mut score = 0;
    let mut count = 0;

    while let Some(c) = stream.next() {
        match (state, c) {
            (State::Group, '{') => depth += 1,
            (State::Group, '}') => { score += depth; depth -= 1; },
            (State::Group, '<') => state = State::Garbage,
            (State::Group, _) => {},
            (State::Garbage, '!') => { stream.next(); },
            (State::Garbage, '>') => state = State::Group,
            (State::Garbage, _) => count += 1,
        }
    }

    return (score, count);
}
