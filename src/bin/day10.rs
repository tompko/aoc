#[macro_use] extern crate lazy_static;
extern crate regex;

use std::cmp::{min, max};
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;
use regex::Regex;

#[derive(Clone)]
enum Dest {
    Bot(usize),
    Output(usize),
}

#[derive(Clone)]
struct Bot {
    low: u32,
    high: u32,
    low_dest: Dest,
    high_dest: Dest,
}

impl Bot {
    fn push(&mut self, a: u32) {
        if self.low == 0 {
            self.low = a;
        } else if self.high == 0 {
            self.high = a;

            let low = min(self.low, self.high);
            let high = max(self.low, self.high);

            self.low = low;
            self.high = high;
        } else {
            unreachable!();
        }
    }

    fn get(&self) -> (u32, u32) {
        (self.low, self.high)
    }
}

struct Factory {
    inputs: Vec<(usize, u32)>,
    outputs: usize,
    bots: Vec<Bot>,
}

impl Factory {
    fn new(file: File) -> Self {
        let reader = BufReader::new(file);

        let mut inputs = Vec::new();
        let mut bot_map = HashMap::new();
        let mut max_output = 0;

        let inre = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        let botre = Regex::new(r"bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();

            if let Some(caps) = inre.captures(&line) {
                inputs.push((caps.at(2).unwrap().parse().unwrap(), caps.at(1).unwrap().parse().unwrap()));
            } else if let Some(caps) = botre.captures(&line) {
                let index = caps.at(1).unwrap().parse::<usize>().unwrap();
                let low = caps.at(3).unwrap().parse().unwrap();
                let low_dest = match caps.at(2).unwrap() {
                    "output" => { max_output = max(max_output, low); Dest::Output(low)},
                    "bot" => Dest::Bot(low),
                    _ => unreachable!(),
                };
                let high = caps.at(5).unwrap().parse().unwrap();
                let high_dest = match caps.at(4).unwrap() {
                    "output" => { max_output = max(max_output, high); Dest::Output(high)},
                    "bot" => Dest::Bot(high),
                    _ => unreachable!(),
                };

                bot_map.insert(index, Bot{
                    low: 0, high: 0,
                    low_dest: low_dest,
                    high_dest: high_dest,
                });
            } else {
                unreachable!();
            }
        }

        let mut bots = Vec::new();
        for i in 0..bot_map.len() {
            bots.push(bot_map.remove(&i).unwrap());
        }

        Factory{
            inputs: inputs,
            outputs: max_output + 1,
            bots: bots,
        }
    }

    fn simulate(&mut self) -> Vec<u32> {
        let mut nexts = HashSet::new();
        let mut processed = HashSet::new();
        let mut outputs = vec![0; self.outputs];

        for i in &self.inputs {
            self.bots[i.0].push(i.1);
            nexts.insert(i.0);
        }

        while !nexts.is_empty() {
            let mut nnexts = HashSet::new();

            for n in &nexts {
                if processed.contains(n) {
                    continue
                }

                let bot = self.bots[*n].clone();

                if bot.low != 0 && bot.high != 0 {
                    let (low, high) = bot.get();

                    match bot.low_dest {
                        Dest::Bot(i) => {
                            self.bots[i].push(low);
                            nnexts.insert(i);
                        }
                        Dest::Output(i) => outputs[i] = low,
                    }
                    match bot.high_dest {
                        Dest::Bot(i) => {
                            self.bots[i].push(high);
                            nnexts.insert(i);
                        }
                        Dest::Output(i) => outputs[i] = high,
                    }
                    processed.insert(*n);
                } else {
                    nnexts.insert(*n);
                }
            }

            nexts = nnexts;
        }
        outputs
    }

}

fn main() {
    let file = File::open("input/day10.in").expect("Failed to open input");

    let mut factory = Factory::new(file);

    let outputs = factory.simulate();

    for (i, b) in factory.bots.iter().enumerate() {
        if b.low == 17 && b.high == 61 {
            println!("1: {}", i);
        }
    }

    println!("2: {}", outputs[0] * outputs[1] * outputs[2]);

}
