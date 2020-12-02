extern crate fancy_regex;

use fancy_regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn nice1(word: &str) -> bool {
    let one = Regex::new(r"(?:[aeiou].*?){3}").unwrap();
    let two = Regex::new(r"(.)\1").unwrap();
    let three = Regex::new(r"ab|cd|pq|xy").unwrap();

    one.is_match(word).unwrap() && two.is_match(word).unwrap() && !three.is_match(word).unwrap()
}

fn nice2(word: &str) -> bool {
    let one = Regex::new(r"(..).*\1").unwrap();
    let two = Regex::new(r"(.).\1").unwrap();

    one.is_match(word).unwrap() && two.is_match(word).unwrap()
}

fn main() {
    let f = File::open("inputs/day05.in").unwrap();
    let file = BufReader::new(&f);

    let words: Vec<_> = file.lines().map(|a| a.unwrap()).collect();

    println!("{}", words.iter().filter(|a| nice1(&a)).count());
    println!("{}", words.iter().filter(|a| nice2(&a)).count());
}
