use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn play(path: &str, players: usize) -> usize {
    let mut active = 0;
    let mut xs: Vec<i32> = vec![0; players];
    let mut ys: Vec<i32> = vec![0; players];
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    seen.insert((0, 0));

    for p in path.chars() {
        xs[active] += match p {
            '<' => -1,
            '>' => 1,
            _ => 0,
        };
        ys[active] += match p {
            '^' => 1,
            'v' => -1,
            _ => 0,
        };
        seen.insert((xs[active], ys[active]));
        active = (active + 1) % players;
    }

    seen.len()
}

fn main() {
    let mut f = File::open("inputs/day03.in").unwrap();
    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();
    let contents = contents.trim();
    println!("{}", play(contents, 1));
    println!("{}", play(contents, 2));
}
