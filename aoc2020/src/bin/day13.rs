use std::fs::File;
use std::io::{BufRead, BufReader};

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
 
    let mut sum = 0;
 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
 
    Some(sum % prod)
}

fn main() {
    let file = File::open("input/day13.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut lines = file.lines();
    let timestamp = lines.next().unwrap().unwrap().parse::<i64>().unwrap();
    let buses = lines.next().unwrap().unwrap();

    let ids: Vec<_> = buses.split(",").filter(|&b| b != "x").map(|b| b.parse::<i64>().unwrap()).collect();

    let (i, t) = ids.iter().map(|i| (i, ((timestamp / i) + 1) * i)).min_by_key(|&(_, t)| t).unwrap();
    let part1 = i * (t - timestamp);

    println!("Part 1: {}", part1);

    let residues: Vec<i64> = buses.split(",").enumerate().filter(|&(_, b)| b != "x").map(|(i, b)| b.parse::<i64>().unwrap()-(i as i64)).collect();
    let part2 = chinese_remainder(&residues, &ids).unwrap();

    println!("Part 2: {}", part2);
}