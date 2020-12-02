extern crate fancy_regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;
use fancy_regex::Regex;

#[derive(Clone, Debug)]
enum Operation {
    Nop,
    Not,
    And,
    Or,
    LShift,
    RShift,
}

#[derive(Clone, Debug)]
struct Gate {
    inputs: Vec<String>,
    op: Operation,
    output: String,
}

fn parse(line: &str) -> Gate {
    let sig = Regex::new(r"^(\d+) -> (\w)+").unwrap();
    let nop = Regex::new(r"^(\w+) -> (\w+)").unwrap();
    let no = Regex::new(r"^NOT (\w+) -> (\w+)").unwrap();
    let op = Regex::new(r"^(\w+) (\w+) (\w+) -> (\w+)").unwrap();

    let msig = sig.captures(line).unwrap();
    let mnop = nop.captures(line).unwrap();
    let mno = no.captures(line).unwrap();
    let mop = op.captures(line).unwrap();

    if msig.is_some() {
        let msig = msig.unwrap();
        Gate{
            inputs: vec![msig.get(1).unwrap().as_str().to_owned()],
            op: Operation::Nop,
            output: msig.get(2).unwrap().as_str().to_owned(),
        }
    } else if mnop.is_some() {
        let mnop = mnop.unwrap();
        Gate {
            inputs: vec![mnop.get(1).unwrap().as_str().to_owned()],
            op: Operation::Nop,
            output: mnop.get(2).unwrap().as_str().to_owned(),
        }
    } else if mno.is_some() {
        let mno = mno.unwrap();
        Gate {
            inputs: vec![mno.get(1).unwrap().as_str().to_owned()],
            op: Operation::Not,
            output: mno.get(2).unwrap().as_str().to_owned(),
        }
    } else {
        let mop = mop.unwrap();
        Gate {
            inputs: vec![mop.get(1).unwrap().as_str().to_owned(), mop.get(3).unwrap().as_str().to_owned()],
            op: match mop.get(2).unwrap().as_str() {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "LSHIFT" => Operation::LShift,
                "RSHIFT" => Operation::RShift,
                _ => unreachable!(),
            },
            output: mop.get(4).unwrap().as_str().to_owned(),
        }
    }
}

fn simulate(gates: &[Gate], overrides: Vec<(&str, u16)>) -> HashMap<String, u16> {
    let mut signals: HashMap<String, u16> = HashMap::new();
    for i in 0..(1<<16) {
        signals.insert(i.to_string(), i as u16);
    }

    for o in &overrides {
        let &(s, a) = o;
        signals.insert(s.to_string(), a);
    }

    let mut queue: VecDeque<Gate> = gates.iter().cloned().collect();
    while !queue.is_empty() {
        let g = queue.pop_front().unwrap();

        if signals.contains_key(&g.output) {
            continue;
        }

        let mut all_inputs = true;
        for i in &g.inputs {
            if !signals.contains_key(i) {
                all_inputs = false;
                break;
            }
        }

        if !all_inputs {
            queue.push_back(g);
            continue;
        }

        match g.op {
            Operation::Nop => {
                let a = signals[&g.inputs[0]];
                signals.insert(g.output, a);
            },
            Operation::Not => {
                let a = signals[&g.inputs[0]];
                signals.insert(g.output, !a);
            },
            Operation::And => {
                let a = signals[&g.inputs[0]];
                let b = signals[&g.inputs[1]];
                signals.insert(g.output, a & b);
            },
            Operation::Or => {
                let a = signals[&g.inputs[0]];
                let b = signals[&g.inputs[1]];
                signals.insert(g.output, a | b);
            },
            Operation::LShift => {
                let a = signals[&g.inputs[0]];
                let b = g.inputs[1].parse::<usize>().unwrap();
                signals.insert(g.output, a << b);
            },
            Operation::RShift => {
                let a = signals[&g.inputs[0]];
                let b = g.inputs[1].parse::<usize>().unwrap();
                signals.insert(g.output, a >> b);
            },
        };
    }

    signals
}

fn main() {
    let f = File::open("inputs/day07.in").unwrap();
    let file = BufReader::new(&f);

    let mut gates: Vec<_> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        gates.push(parse(&line));
    }

    let signals = simulate(&gates, Vec::new());
    println!("{}", signals["a"]);

    let overrides: Vec<_> = vec![("b", signals["a"])];
    let signals = simulate(&gates, overrides);
    println!("{}", signals["a"]);
}
