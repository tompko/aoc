extern crate pcre;

use pcre::Pcre;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn nice1(word: &str) -> bool {
    let mut one = Pcre::compile(r"(?:[aeiou].*?){3}").unwrap();
    let mut two = Pcre::compile(r"(.)\1").unwrap();
    let mut three = Pcre::compile(r"ab|cd|pq|xy").unwrap();

    one.exec(word).and(two.exec(word)).is_some() &&
        !three.exec(word).is_some()
}

fn nice2(word: &str) -> bool {
    let mut one = Pcre::compile(r"(..).*\1").unwrap();
    let mut two = Pcre::compile(r"(.).\1").unwrap();

    one.exec(word).and(two.exec(word)).is_some()
}

fn main() {
    let f = File::open("inputs/day05.in").unwrap();
    let file = BufReader::new(&f);

    let words: Vec<_> = file.lines().map(|a| a.unwrap()).collect();

    println!("{}", words.iter().filter(|a| nice1(&a)).count());
    println!("{}", words.iter().filter(|a| nice2(&a)).count());
}
