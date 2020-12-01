use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn escape(string: &str) -> String {
    let mut ret = String::new();
    ret.push('"');

    for ch in string.chars() {
        match ch {
            '"' => ret += "\\\"",
            '\\' => ret += "\\\\",
            a => ret.push(a),
        }
    }
    ret.push('"');
    ret
}

fn unescape(string: &str) -> String {
    let mut seq = string.chars().skip(1);
    let mut ret = String::new();

    loop {
        match seq.next() {
            Some('\\') => {
                match seq.next() {
                    Some('\\') => ret.push('\\'),
                    Some('"') => ret.push('"'),
                    Some('x') => {
                        let mut charcode: u8 = seq.next().unwrap().to_digit(16).unwrap() as u8;
                        charcode = (charcode * 10) + seq.next().unwrap().to_digit(16).unwrap() as u8;

                        ret.push(charcode as char);
                    },
                    _ => unreachable!(),
                }
            },
            Some('"') | None => break,
            Some(a) => ret.push(a),
        }
    }

    ret
}

fn main() {
    let f = File::open("inputs/day08.in").unwrap();
    let file = BufReader::new(&f);

    let (count1, count2) = file.lines().
        map(|l| l.unwrap()).
        map(|l| (l.len(), escape(l.trim()), unescape(l.trim()))).
        map(|(l, e, u)| (l - u.chars().count(), e.len() - l)).
        fold((0, 0), |a, b| (a.0+b.0, a.1 + b.1));

    println!("1: {}", count1);
    println!("2: {}", count2);
}
