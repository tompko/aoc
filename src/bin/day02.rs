use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};

fn main() {
    let file = File::open("input/day02.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut checksum = 0;
    let mut data = 0;

    for line in file.lines() {
        let mut numbers = line.unwrap().
            split_whitespace().
            map(|x| x.parse::<u32>().unwrap()).
            collect::<Vec<_>>();
        numbers.sort();

        let (mini, maxi) = numbers.iter().
            fold((1000000, 0), |(m, n), &x| (min(m, x), max(n, x)));

        checksum += maxi - mini;

        for (i, x) in numbers.iter().enumerate() {
            for y in numbers.iter().skip(i + 1) {
                if y % x == 0 {
                    data += y / x;
                }
            }
        }
    }

    println!("part 1: {}", checksum);
    println!("part 2: {}", data);
}
