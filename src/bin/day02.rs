use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let f = File::open("inputs/day02.in").unwrap();
    let file = BufReader::new(&f);

    let (paper, ribbon) = file
        .lines()
        .map(|l| -> Vec<u32> {
            let mut x: Vec<_> = l.
                unwrap().
                split('x').
                map(|x| -> u32 { x.parse().unwrap() }).
                collect();
            x.sort();
            x
        })
        .map(|x| (
                2*x[0]*x[1] + 2*x[1]*x[2] + 2*x[2]*x[0] + x[0]*x[1],
                2*x[0] + 2*x[1] + x[0]*x[1]*x[2]
            ))
        .fold((0,0), |a, b| (a.0+b.0, a.1+b.1));

    println!("1: {}", paper);
    println!("2: {}", ribbon);
}
