use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn search(containers: &Vec<i32>) -> HashMap<i32, i32> {
    let mut ret = HashMap::new();

    for i in 0..(1 << containers.len()) {
        let mut s = 0;
        let mut t = 0;
        for (j, c) in containers.iter().enumerate() {
            if i & (1 << j) == (1 << j) {
                s = s + c;
                t += 1;
            }
        }

        if s == 150 {
            let score = ret.entry(t).or_insert(0);
            *score += 1;
        }
    }

    ret
}

fn main() {
    let f = File::open("day17.in")
        .ok()
        .expect("Errer opening input");
    let file = BufReader::new(&f);

    let mut containers = Vec::new();

    for line in file.lines() {
        containers.push(line.unwrap().parse::<_>().unwrap());
    }

    let mut total = 0;
    let mut min_number = 2000;
    let mut min_total = 0;

    for (k, v) in search(&containers).into_iter() {
        total += v;
        if k < min_number {
            min_number = k;
            min_total = v;
        }
    }
    println!("{}", total);
    println!("{}", min_total);
}
