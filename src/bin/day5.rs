extern crate pcre;

use pcre::Pcre;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn nice1(word: &String) -> bool {
    let mut one = Pcre::compile(r"(?:[aeiou].*?){3}").unwrap();
    let mut two = Pcre::compile(r"(.)\1").unwrap();
    let mut three = Pcre::compile(r"ab|cd|pq|xy").unwrap();

    if !one.exec(word).is_some() {
        return false;
    }
    if !two.exec(word).is_some() {
        return false;
    }
    if three.exec(word).is_some() {
        return false;
    }
    true
}

fn nice2(word: &String) -> bool {
    let mut one = Pcre::compile(r"(..).*?\1").unwrap();
    let mut two = Pcre::compile(r"(.).\1").unwrap();

    if !one.exec(word).is_some() {
        return false;
    }
    if !two.exec(word).is_some() {
        return false;
    }

    true
}

fn main() {
    let f = File::open("day5.in")
        .ok()
        .expect("Error opening input");
    let file = BufReader::new(&f);

    let words: Vec<String> = file.lines().map(|a| a.unwrap()).collect();

    println!("{}", words.clone().into_iter().filter(nice1).count());
    println!("{}", words.clone().into_iter().filter(nice2).count());
}
