use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Distance {
    Best(isize, usize),
    Tied(isize),
}

fn main() {
    let file = File::open("input/day06.in").expect("Failed to open input");
    let file = BufReader::new(&file);

     let mut points = Vec::new();
     let mut max_x = 0;
     let mut max_y = 0;

     for line in file.lines() {
         let line = line.unwrap();
         let mut parts = line.split(", ");
         let x = parts.next().unwrap().parse::<isize>().unwrap();
         let y = parts.next().unwrap().parse::<isize>().unwrap();

         points.push((x, y));
         max_x = max(max_x, x);
         max_y = max(max_y, y);
     }

    let mut infinite = HashSet::new();
    let mut areas = vec![0; points.len()];
    let mut part2 = 0;

    for y in 0..max_y {
        for x in 0..max_x {
            let mut best_dist = Distance::Tied(max_x + max_y);
            let mut total_dist = 0;

            for (i, p) in points.iter().enumerate() {
                let d = distance(*p, (x, y));
                match best_dist {
                    Distance::Best(n, _) => {
                        if d == n {
                            best_dist = Distance::Tied(d);
                        } else if d < n {
                            best_dist = Distance::Best(d, i);
                        }
                    }
                    Distance::Tied(n) => {
                        if d < n {
                            best_dist = Distance::Best(d, i);
                        }
                    }
                }
                total_dist += d;
            }

            if let Distance::Best(_, i) = best_dist {
                areas[i] += 1;
                if x == 0 || x == max_x-1 || y == 0 || y == max_y-1 {
                    infinite.insert(i);
                }
            }
            if total_dist < 10000 {
                part2 += 1;
            }
        }
    }

    let (_, part1) = areas.iter()
        .enumerate()
        .filter(|(i, _)| !infinite.contains(i))
        .max_by_key(|(_, x)| *x).unwrap();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn distance(a: (isize, isize), b: (isize, isize)) -> isize {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}
