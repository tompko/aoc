extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
struct Worker {
    step: char,
    finish_time: u32,
}

fn main() {
    let file = File::open("input/day07.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let line_re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    let mut prereqs = HashMap::new();
    let mut steps = HashSet::new();

    for line in file.lines() {
        let line = line.unwrap();

        let lcap = line_re.captures(&line).unwrap();
        let step = lcap.get(2).unwrap().as_str().chars().nth(0).unwrap();
        let pre = lcap.get(1).unwrap().as_str().chars().nth(0).unwrap();

        steps.insert(step);
        steps.insert(pre);

        prereqs.entry(step)
            .and_modify(|s: &mut Vec<_>| s.push(pre))
            .or_insert_with(|| vec![pre]);
    }

    let mut steps: Vec<char> = steps.into_iter().collect();
    steps.sort();

    let part1 = part_1(&steps, &prereqs);
    let part2 = part_2(&steps, &prereqs);


    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn part_1(steps: &[char], prereqs: &HashMap<char, Vec<char>>) -> String {
    let mut order = String::new();
    let mut done = HashSet::new();

    while done.len() < steps.len() {
        for s in steps.iter() {
            if done.contains(s) {
                continue
            }
            let default = Vec::new();
            let pre_steps = prereqs.get(s).unwrap_or(&default);
            if pre_steps.iter().all(|x| done.contains(x)) {
                order.push(*s);
                done.insert(*s);
                break;
            }
        }
    }

    order
}

fn part_2(steps: &[char], prereqs: &HashMap<char, Vec<char>>) -> u32 {
    let mut done = HashSet::new();
    let mut inflight = HashSet::new();
    let mut workers = Vec::new();
    let mut time = 0;
    let step_times: HashMap<_, _> = steps.iter().enumerate()
        .map(|(i, &c)| (c, (i as u32) + 61))
        .collect();

    while done.len() < steps.len() {
        for _ in 0..(5 - workers.len()) {
            for s in steps.iter() {
                if done.contains(s) || inflight.contains(s) {
                    continue
                }
                let default = Vec::new();
                let pre_steps = prereqs.get(s).unwrap_or(&default);
                if pre_steps.iter().all(|x| done.contains(x)) {
                    inflight.insert(*s);
                    workers.push(Worker{ step: *s, finish_time: time + step_times[s] });
                    break;
                }
            }
        }

        workers.sort_by_key(|w| w.finish_time);
        workers.reverse();

        if let Some(w) = workers.pop() {
            time = w.finish_time;
            done.insert(w.step);
            inflight.remove(&w.step);
        }
    }

    time
}
