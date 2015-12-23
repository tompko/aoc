extern crate pcre;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::collections::VecDeque;
use pcre::Pcre;

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
    let mut sig = Pcre::compile(r"^(\d+) -> (\w)+").unwrap();
    let mut nop = Pcre::compile(r"^(\w+) -> (\w+)").unwrap();
    let mut no = Pcre::compile(r"^NOT (\w+) -> (\w+)").unwrap();
    let mut op = Pcre::compile(r"^(\w+) (\w+) (\w+) -> (\w+)").unwrap();

    let msig = sig.exec(line);
    let mnop = nop.exec(line);
    let mno = no.exec(line);
    let mop = op.exec(line);

    if msig.is_some() {
        let msig = msig.unwrap();
        return Gate{
            inputs: vec![msig.group(1).to_string()],
            op: Operation::Nop,
            output: msig.group(2).to_string(),
        }
    }
    if mnop.is_some() {
        let mnop = mnop.unwrap();
        return Gate {
            inputs: vec![mnop.group(1).to_string()],
            op: Operation::Nop,
            output: mnop.group(2).to_string(),
        }
    }
    if mno.is_some() {
        let mno = mno.unwrap();
        return Gate {
            inputs: vec![mno.group(1).to_string()],
            op: Operation::Not,
            output: mno.group(2).to_string(),
        }
    }
    if mop.is_some() {
        let mop = mop.unwrap();
        return Gate {
            inputs: vec![mop.group(1).to_string(), mop.group(3).to_string()],
            op: match mop.group(2) {
                "AND" => Operation::And,
                "OR" => Operation::Or,
                "LSHIFT" => Operation::LShift,
                "RSHIFT" => Operation::RShift,
                _ => unreachable!(),
            },
            output: mop.group(4).to_string(),
        }
    }

    unreachable!()
}

fn simulate(gates: &Vec<Gate>, overrides: Vec<(&str, u16)>) -> HashMap<String, u16> {
    let mut signals: HashMap<String, u16> = HashMap::new();
    for i in 0..(1<<16) {
        signals.insert(i.to_string(), i as u16);
    }

    for o in overrides.iter() {
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
        for i in g.inputs.iter() {
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
    let f = File::open("day7.in")
        .ok()
        .expect("Failed to open input");
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
