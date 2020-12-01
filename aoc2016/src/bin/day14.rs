extern crate crypto;

use std::collections::HashMap;
use crypto::md5::Md5;
use crypto::digest::Digest;

struct HashCache<'a> {
    base: String,
    hashes: HashMap<String, String>,
    hasher: &'a Fn(&str) -> String,
}

impl<'a> HashCache<'a> {
    fn new(base: &str, f: &'a Fn(&str) -> String) -> Self {
        HashCache{
            base: base.to_owned(),
            hashes: HashMap::new(),
            hasher: f,
        }
    }

    fn get(&mut self, index: usize) -> String {
        let input = format!("{}{}", self.base, index);

        if self.hashes.get(&input).is_none() {
            let key = (self.hasher)(&input);
            self.hashes.insert(input.clone(), key);
        }

        self.hashes[&input].clone()
    }
}

fn has_triple(key: &str) -> Option<String> {
    let v : Vec<_> = key.chars().collect();
    let mut windows = v.windows(3);

    while let Some(x) = windows.next() {
        let (a, b, c) = (x[0], x[1], x[2]);
        if a == b && b == c {
            return Some(format!("{}{}{}{}{}", a, a, a, a, a))
        }
    }
    None
}

fn find_keys(salt: &str, num_keys: usize, hasher: &Fn(&str) -> String) -> Vec<(usize, String)> {
    let mut ret = Vec::new();
    let mut index = 0;
    let mut hc = HashCache::new(salt, hasher);

    while ret.len() < num_keys {
        let key = hc.get(index);

        if let Some(t) = has_triple(&key) {
            for i in 1..1000 {
                if hc.get(index + i).contains(&t) {
                    ret.push((index, key));
                    break
                }
            }
        }
        index += 1;
    }

    ret
}

fn h1(salt: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input(salt.as_bytes());
    hasher.result_str()
}

fn h2016(salt: &str) -> String {
    let mut hasher = Md5::new();
    let mut key = h1(salt);

    for _ in 0..2016 {
        hasher.input(key.as_bytes());
        key = hasher.result_str();
        hasher.reset();
    }
    key
}

fn main() {
    let keys = find_keys("jlmsuwbz", 64, &h1);
    println!("1: {}", keys.last().unwrap().0);

    let keys = find_keys("jlmsuwbz", 64, &h2016);
    println!("2: {}", keys.last().unwrap().0);
}
