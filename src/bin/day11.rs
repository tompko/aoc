use std::fs::File;
use std::io::Read;
use std::cmp::max;

#[derive(Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut file = File::open("input/day11.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let mut pos = Coord{ x: 0, y: 0, z: 0};
    let mut max_dist = 0;

    for m in contents.split(",") {
        match m {
            "n" => pos = Coord{ x: pos.x, y: pos.y + 1, z: pos.z - 1 },
            "ne" => pos = Coord{ x: pos.x + 1, y: pos.y, z: pos.z - 1 },
            "se" => pos = Coord{ x: pos.x + 1, y: pos.y - 1, z: pos.z },
            "s" => pos = Coord{ x: pos.x, y: pos.y - 1, z: pos.z + 1 },
            "sw" => pos = Coord{ x: pos.x - 1, y: pos.y, z: pos.z + 1 },
            "nw" => pos = Coord{ x: pos.x - 1, y: pos.y + 1, z: pos.z },
            _ => unreachable!(),
        }
        let dist = (pos.x.abs() + pos.y.abs() + pos.z.abs()) / 2;
        max_dist = max(max_dist, dist);
    }

    let dist = (pos.x.abs() + pos.y.abs() + pos.z.abs()) / 2;

    println!("part 1: {}", dist);
    println!("part 2: {}", max_dist);
}
