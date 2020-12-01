use std::fs::File;
use std::io::Read;
use std::cmp::min;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn main() {
    let mut file = File::open("input/day05.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");

    let ans = reduce(&contents);

    let mut min_len = contents.len();

    for a in ALPHABET.chars() {
        let poly = remove(&contents, a);
        let l = reduce(&poly).len();
        min_len = min(min_len, l);
    }

    println!("part 1: {}", ans.len());
    println!("part 2: {}", min_len);
}

fn reduce(contents: &str) -> Vec<char> {
    let mut contents: Vec<_> = contents.trim().chars().collect();
    let mut old_len = 0;

    while old_len != contents.len() {
        let mut index = 0;
        let mut new_contents = Vec::new();
        old_len = contents.len();

        while index < contents.len() {
            if index == contents.len() - 1 {
                new_contents.push(contents[index]);
                index += 1;
                continue
            }
            if contents[index] != contents[index+1] && contents[index].to_ascii_uppercase() == contents[index+1].to_ascii_uppercase() {
                index += 2;
            } else {
                new_contents.push(contents[index]);
                index += 1;
            }
        }
        contents = new_contents;
    }

    contents
}

fn remove(s: &str, c: char) -> String {
    let mut ret = String::new();

    for x in s.chars() {
        if x.to_ascii_lowercase() != c {
            ret.push(x);
        }
    }

    ret
}
