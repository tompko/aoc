use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Xmas {
    deque: VecDeque<i64>,
    hash: HashMap<i64, usize>,
}

impl Xmas {
    fn new() -> Self {
        Xmas {
            deque: VecDeque::with_capacity(25),
            hash: HashMap::new(),
        }
    }

    fn is_valid_next(&self, i: i64) -> bool {
        for &a in self.hash.keys() {
            let b = i-a;
            if a != b && self.hash.get(&b).is_some() {
                return true;
            }
        }

        false
    }

    fn insert(&mut self, i: i64) {
        if self.deque.len() == 25 {
            let rem = self.deque.pop_front().unwrap();
            if *self.hash.get(&rem).unwrap() == 1 {
                self.hash.remove(&rem);
            } else {
                let c = self.hash.get_mut(&rem).unwrap();
                *c -= 1;
            }
        }
        self.deque.push_back(i);
        let c = self.hash.entry(i).or_insert(0);
        *c += 1;
    }
}

fn main() {
    let file = File::open("input/day09.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let numbers: Vec<i64> = file.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let mut xmas = Xmas::new();
    let mut part1 = 0;

    for (i, &x) in numbers.iter().enumerate() {
        if i >= 25 && !xmas.is_valid_next(x) {
            part1 = x;
            break;
        }
        xmas.insert(x);
    }

    let mut subset = VecDeque::new();
    let mut sum = 0;

    for &n in numbers.iter() {
        subset.push_back(n);
        sum += n;
        while sum > part1 {
            let rem = subset.pop_front().unwrap();
            sum -= rem;
        }
        if sum == part1 && subset.len() >= 2{
            break;
        }
    }

    let part2 = subset.iter().min().unwrap() + subset.iter().max().unwrap();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}