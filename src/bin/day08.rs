use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

fn group(vs: Vec<u32>, d: usize) -> Vec<Vec<u32>> {
    if vs.len() % d != 0 {
        panic!("Length of vector not divisible evenly");
    }

    let mut res = Vec::new();
    let mut curr = Vec::new();

    for v in vs {
        curr.push(v);
        if curr.len() == d {
            res.push(curr);
            curr = Vec::new();
        }
    }
    res
}

fn main() {
    let mut file = File::open("input/day08.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    let pixels: Vec<_> = contents.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let layers = group(pixels, 25*6);

    let mut part1 = 0;
    let mut zero_count = 25*6;
    for l in layers.iter() {
        let mut counts: HashMap<u32, usize> = HashMap::new();
        for p in l {
            let c = counts.get(&p).unwrap_or(&0) + 1;
            counts.insert(*p, c);
        }

        if *counts.get(&0).unwrap() < zero_count {
            part1 = counts.get(&1).unwrap() * counts.get(&2).unwrap();
            zero_count = *counts.get(&0).unwrap();
        }
    }

    let mut image = vec![2; 25*6];
    for l in layers.iter() {
        for (i, p) in l.iter().enumerate() {
            if image[i] == 2 {
                image[i] = *p;
            }
        }
    }

    println!("Part 1: {}", part1);
    for line in group(image, 25) {
        for l in line.iter(){
            if *l == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}