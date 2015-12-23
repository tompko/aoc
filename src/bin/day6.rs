extern crate pcre;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use pcre::Pcre;

enum Action {
    On,
    Toggle,
    Off,
}

struct Instruction {
    start: (usize, usize),
    stop: (usize, usize),
    action: Action,
}

fn parse(line: &str) -> Instruction {
    let mut re = Pcre::compile(r"(turn on|toggle|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();
    let m = re.exec(line).expect("Failed to parse line");

    let start = (m.group(2).parse().unwrap(), m.group(3).parse().unwrap());
    let stop = (m.group(4).parse().unwrap(), m.group(5).parse().unwrap());

    match m.group(1) {
        "turn on" => Instruction{start:start, stop:stop, action:Action::On},
        "toggle" => Instruction{start:start, stop:stop, action: Action::Toggle},
        "turn off" => Instruction{start:start, stop:stop, action: Action::Off},
        _ => unreachable!(),
    }
}

fn simulate(instr: &Vec<Instruction>) -> (u32, u32) {
    let mut part1: Vec<Vec<bool>> = Vec::with_capacity(1000);
    let mut part2: Vec<Vec<u32>> = Vec::with_capacity(1000);
    for _ in 0..1000 {
        part1.push(vec![false; 1000]);
        part2.push(vec![0; 1000]);
    }

    for i in instr{
        let (a, b) = i.start;
        let (c, d) = i.stop;
        for x in a..c+1 {
            for y in b..d+1 {
                match i.action {
                    Action::On => {
                        part1[y][x] = true;
                        part2[y][x] += 1;
                    },
                    Action::Toggle => {
                        part1[y][x] = !part1[y][x];
                        part2[y][x] += 2;
                    },
                    Action::Off => {
                        part1[y][x] = false;
                        if part2[y][x] > 0 {
                            part2[y][x] -= 1;
                        }
                    },
                }
            }
        }
    }

    let mut p1count = 0;
    let mut p2count = 0;

    for x in 0..1000 {
        for y in 0..1000 {
            if part1[y][x] {
                p1count += 1;
            }
            p2count += part2[y][x];
        }
    }

    (p1count, p2count)
}

fn main() {
    let f = File::open("day6.in")
        .ok()
        .expect("Failed to open input");
    let file = BufReader::new(&f);

    let mut instr: Vec<_> = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();
        instr.push(parse(&line));
    }

    let (a, b) = simulate(&instr);
    println!("{}", a);
    println!("{}", b);
}
