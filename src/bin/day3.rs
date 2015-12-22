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
        match p{
            '<' => xs[active] -= 1,
            '>' => xs[active] += 1,
            '^' => ys[active] += 1,
            'v' => ys[active] -= 1,
            _ => {}
        };
        seen.insert((xs[active], ys[active]));
        active = (active + 1) % players;
    }

    seen.len()
}

fn main() {
    let mut f = File::open("day3.in")
        .ok()
        .expect("Error opening input");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .ok()
        .expect("Error reading input");
    let contents = contents.trim();
    println!("{}", play(contents, 1));
    println!("{}", play(contents, 2));
}
