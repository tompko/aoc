const CUPS: &str = "364289715";

#[derive(Debug)]
struct Cups {
    current: usize,
    cups: Vec<usize>,
}

impl Cups {
    fn new(size: usize, start: usize) -> Cups {
        let mut cups = Vec::with_capacity(size + 1);
        let mut current = start;

        for i in 0..(size + 1) {
            cups.push(0);
        }
        cups[start] = start;

        Cups {
            current: current,
            cups: cups,
        }
    }

    fn advance(&mut self) {
        self.current = self.cups[self.current];
    }

    fn remove_after(&mut self, index: usize) -> usize {
        let ret = self.cups[index];
        let next = self.cups[ret];
        self.cups[index] = next;
        ret
    }

    fn insert_after(&mut self, index: usize, value: usize) {
        let next = self.cups[index];
        self.cups[index] = value;
        self.cups[value] = next;
    }

    fn to_vec(&self) -> Vec<usize> {
        let mut ret = Vec::new();
        let mut prev = self.current;

        for _ in 0..(self.cups.len()-1) {
            ret.push(prev);
            prev = self.cups[prev];
        }

        ret
    }

    fn after(&self, index: usize) -> usize {
        self.cups[index]
    }

    fn step(&mut self) {
        let mut pick_up: Vec<usize> = Vec::new();
        for _ in 0..3 {
            pick_up.push(self.remove_after(self.current));
        }
        let mut destination = self.current - 1;
        if destination == 0 {
            destination = self.cups.len() - 1;
        }
        while pick_up.iter().filter(|p| **p == destination).count() > 0 {
            destination -= 1;
            if destination == 0 {
                destination = self.cups.len() - 1;
            }
        }
        for &p in pick_up.iter().rev() {
            self.insert_after(destination, p);
        }
        self.advance();
    }
}

fn main() {
    let init: Vec<_> = CUPS.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut cups = Cups::new(CUPS.len(), init[0]);

    let mut prev = init[0];
    for &c in init.iter().skip(1) {
        cups.insert_after(prev, c);
        prev = c;
    }
    for _ in 0..100 {
        cups.step();
    }

    let mut part1 = String::new();
    let mut next = cups.after(1);
    while next != 1 {
        part1 += &format!("{}", next);
        next = cups.after(next);
    }

    println!("Part 1: {}", part1);

    let mut cups = Cups::new(1000000, init[0]);
    let mut prev = init[0];
    for &c in init.iter().skip(1) {
        cups.insert_after(prev, c);
        prev = c;
    }
    for i in 10..1000001 {
        cups.insert_after(prev, i);
        prev = i;
    }

    for i in 0..10000000 {
        cups.step();
    }
    let after = cups.after(1);
    let post = cups.after(after);
    let part2 = after * post;
    println!("Part 2: {}", part2);
}