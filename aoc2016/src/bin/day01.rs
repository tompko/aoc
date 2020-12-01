use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;

fn main() {
    let mut file = File::open("input/day01.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");

    let compass: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut point = 0;
    let mut x = 0;
    let mut y = 0;
    let mut part2ans: Option<(i32, i32)> = None;
    let mut places = HashSet::new();

    for d in contents.split(',') {
        let (turn, dist) = d.trim().split_at(1);

        point = match turn {
            "L" => if point == 0 { compass.len() - 1 } else { point - 1 },
            "R" => if point == compass.len() - 1 { 0 } else { point + 1 },
            _ => unreachable!(),
        };

        let dist = dist.parse::<i32>().expect("Failed to parse distance");

        for _ in 0..dist {
            x += compass[point].0;
            y += compass[point].1;

            if places.contains(&(x, y)) && part2ans.is_none() {
                part2ans = Some((x, y));
            }
            places.insert((x, y));
        }
    }

    let part2ans = part2ans.expect("No answer found for part2");

    println!("1: {}", x.abs() + y.abs());
    println!("2: {}", part2ans.0.abs() + part2ans.1.abs());
}
