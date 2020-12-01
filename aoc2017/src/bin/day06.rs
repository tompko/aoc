use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut file = File::open("input/day06.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");

    let mut banks = contents.split_whitespace().
        map(|x| x.parse().unwrap()).
        collect::<Vec<_>>();

    let mut cycles = 0;
    let mut seen = HashMap::new();
    let len = banks.len();

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), cycles);

        let (i, &val) = banks.iter().enumerate().
            max_by_key(|&(i, val)| (val, -(i as isize))).unwrap();

        banks[i] = 0;

        for j in 0..val {
            banks[(i + j + 1) % len] += 1;
        }

        cycles += 1
    }

    println!("part 1: {}", cycles);
    println!("part 2: {}", cycles - seen[&banks]);
}
