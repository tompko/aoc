use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let f = File::open("day2.in")
        .ok()
        .expect("Failed to open input");
    let file = BufReader::new(&f);

    let mut paper = 0;
    let mut ribbon = 0;

    for line in file.lines() {
        let line = line.unwrap();
        let mut dims: Vec<_> = line.split("x")
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        dims.sort();

        paper += 2*dims[0]*dims[1] + 2*dims[1]*dims[2] + 2*dims[2]*dims[0];
        paper += dims[0]*dims[1];

        ribbon += 2*dims[0] + 2*dims[1];
        ribbon += dims[0] * dims[1] * dims[2];
    }

    println!("{}", paper);
    println!("{}", ribbon);
}
