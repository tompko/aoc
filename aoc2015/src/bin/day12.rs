extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;

fn add(v: &Value) -> (i64, i64) {
    match v {
        Value::Null => unreachable!(),
        Value::Bool(_) => unreachable!(),
        Value::Number(n) => (n.as_i64().unwrap(), n.as_i64().unwrap()),
        Value::String(_) => (0, 0),
        Value::Array(vector) => {
            let mut part1 = 0;
            let mut part2 = 0;
            for v in vector.iter() {
                let (a, b) = add(v);
                part1 += a;
                part2 += b;
            }
            (part1, part2)
        },
        Value::Object(m) => {
            let mut part1 = 0;
            let mut part2 = 0;
            let mut found_red = false;
            for x in m.values() {
                let (a, b) = add(x);
                part1 += a;
                part2 += b;
                if let Value::String(s) = x {
                    found_red |= s == "red";
                }
            }
            if found_red { (part1, 0) } else { (part1, part2) }
        },
    }
}

fn main() {
    let mut f = File::open("inputs/day12.in").unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    let v: Value = serde_json::from_str(&contents).unwrap();

    let (part1, part2) = add(&v);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}