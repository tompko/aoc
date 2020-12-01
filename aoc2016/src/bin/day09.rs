use std::io::Read;
use std::fs::File;

enum Recurse {
    Yes,
    No,
}

fn count(sequence: &str, recurse: Recurse) -> u64 {
    let mut ret = 0;
    let mut seq = sequence.chars();

    loop {
        match seq.next() {
            Some('(') => {
                let mut length: usize = 0;
                let mut repeats: usize = 0;

                loop {
                    match seq.next() {
                        Some('x') => { length = repeats; repeats = 0 },
                        Some(')') => break,
                        Some(a) => { repeats = repeats * 10 + (a.to_digit(10).unwrap() as usize) },
                        None => unreachable!(),
                    }
                }

                let mut repeated = String::new();
                for _ in 0..length {
                    repeated.push(seq.next().unwrap());
                }

                match recurse {
                    Recurse::Yes => {
                        let l = count(&repeated, Recurse::Yes);
                        ret += l * (repeats as u64);
                    },
                    Recurse::No => {
                        ret += (length * repeats) as u64;
                    },
                }
            }
            Some(_) => ret += 1,
            None => break
        }

    }

    ret
}

fn main() {
    let mut file = File::open("input/day09.in").expect("Failed to open input");
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("1: {}", count(&contents.trim(), Recurse::No));
    println!("2: {}", count(&contents.trim(), Recurse::Yes));
}
