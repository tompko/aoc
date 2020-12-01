extern crate pcre;

use std::io::{BufReader, BufRead};
use std::fs::File;
use pcre::Pcre;

fn split(s: &str) -> (String, String) {
    let mut supernet = Vec::new();
    let mut hypernet = Vec::new();

    for (i, s) in s.split(|c| c=='[' || c == ']').enumerate() {
        if i % 2 == 0 {
            supernet.push(s);
        } else {
            hypernet.push(s);
        }
    }

    (supernet.join(" "), hypernet.join(" "))
}

fn main() {
    let file = File::open("input/day07.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut part1_ans = 0;
    let mut part2_ans = 0;

    let mut abba = Pcre::compile(r"(.)(.)\2\1").unwrap();
    let mut aba = Pcre::compile(r"(.)(?=(.)\1)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        let (supernet, hypernet) = split(line.trim());

        if !abba.exec(&hypernet).is_some() {
            if let Some(caps) = abba.exec(&supernet) {
                if caps.group(1) != caps.group(2) {
                    part1_ans += 1;
                }
            }
        }

        for ma in aba.matches(&supernet) {
            if ma.group(1) == ma.group(2) {
                continue
            }
            let bab = ma.group(2).to_owned() + ma.group(1) + ma.group(2);
            if hypernet.contains(&bab) {
                part2_ans += 1;
                break
            }
        }
    }

    println!("1: {}", part1_ans);
    println!("2: {}", part2_ans);
}
