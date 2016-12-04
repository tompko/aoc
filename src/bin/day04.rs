extern crate regex;

use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;

struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn new(room: &str) -> Self {
        let re = Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)]").unwrap();
        let cap = re.captures(room).unwrap();

        Room{
            name: cap.at(1).unwrap().to_owned(),
            sector_id: cap.at(2).unwrap().parse().unwrap(),
            checksum: cap.at(3).unwrap().to_owned(),
        }
    }

    fn is_valid(&self) -> bool {
        let mut counts = HashMap::new();

        for c in self.name.chars() {
            if c != '-' {
                *counts.entry(c).or_insert(0) += 1;
            }
        }

        let mut freqs: Vec<_> = counts.into_iter().map(|(a, b)| (b, a)).collect();
        freqs.sort_by(|a, b| {
            match a.0.cmp(&b.0) {
                Ordering::Less => Ordering::Greater,
                Ordering::Greater => Ordering::Less,
                Ordering::Equal => a.1.cmp(&b.1),
            }
        });

        freqs.into_iter().zip(self.checksum.chars()).all(|pair| {
            (pair.0).1 == pair.1
        })
    }

    fn decrypt(&self) -> String {
        let shift = (self.sector_id % 26) as u8;

        self.name.chars().map(|ch|
            if ch == '-' {
                ' '
            } else {
                (((ch as u8) - b'a' + shift) % 26 + b'a') as char
            }
        ).collect()
    }
}

fn main() {
    let file = File::open("input/day04.in").expect("Failed to open input");
    let reader = BufReader::new(&file);

    let mut sum_sector_ids = 0;
    let mut part2_ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let room = Room::new(&line);

        if room.is_valid() {
            sum_sector_ids += room.sector_id;

            if room.decrypt() == "northpole object storage" {
                part2_ans = room.sector_id;
            }
        }
    }

    println!("1: {}", sum_sector_ids);
    println!("2: {}", part2_ans);
}
