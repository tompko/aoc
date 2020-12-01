extern crate regex;
extern crate itertools;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Response {
    Weight(u32),
    Delta(u32),
}

fn main() {
    let file = File::open("input/day07.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut nodes = HashMap::new();
    let mut weights = HashMap::new();
    let mut child_nodes = HashSet::new();
    let leaf_re = Regex::new(r"(\w+)\s\((\d+)\)$").unwrap();
    let node_re = Regex::new(r"(\w+)\s\((\d+)\)\s->\s([a-z, ]+)").unwrap();

    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if let Some(lcap) = leaf_re.captures(line) {
            let name = lcap.get(1).unwrap().as_str().to_owned();
            let weight = lcap.get(2).unwrap().as_str().parse::<u32>().unwrap();

            weights.insert(name.clone(), weight);
        } else if let Some(ncap) = node_re.captures(line) {
            let name = ncap.get(1).unwrap().as_str().to_owned();
            let weight = ncap.get(2).unwrap().as_str().parse::<u32>().unwrap();

            let children: Vec<String> = ncap.get(3).unwrap().as_str().
                split(", ").
                map(|x| x.to_owned()).
                collect();

            for c in children.iter() {
                child_nodes.insert(c.to_owned());
            }

            nodes.insert(name.clone(), children);
            weights.insert(name.clone(), weight);
        }
    }

    let root = nodes.keys().
        map(|x| x.to_owned()).
        collect::<HashSet<_>>().
        difference(&child_nodes).
        last().unwrap().clone();

    println!("part 1: {}", root);
    if let Response::Delta(part2) = rebalance(&root, &nodes, &weights) {
        println!("part 2: {:?}", part2);
    }
}

fn rebalance(root: &str, nodes: &HashMap<String, Vec<String>>, weights: &HashMap<String, u32>) -> Response {
    if !nodes.contains_key(root) {
        let w = weights[root];
        return Response::Weight(w);
    }

    let mut child_weights: HashMap<u32, u32> = HashMap::new();
    let mut child_by_weight: HashMap<u32, String> = HashMap::new();

    for child in nodes.get(root).unwrap() {
        match rebalance(child, nodes, weights) {
            Response::Delta(i) =>  return Response::Delta(i),
            Response::Weight(i) => {
                let r = child_weights.get(&i).unwrap_or(&0) + 1;
                child_weights.insert(i, r);
                child_by_weight.insert(i, child.to_owned());
            }
        }
    }

    if child_weights.len() == 1 {
        let mut node_weight = weights[root];

        for (k, v) in child_weights.iter() {
            node_weight += k*v;
        }

        return Response::Weight(node_weight);
    } else {
        let cweights: Vec<_> = child_weights.iter().
            map(|(k, v)| (v, k)).
            sorted().iter().
            map(|&(_, x)| x).
            collect();
        let minority = cweights[0];
        let majority = cweights[1];

        let child = &child_by_weight[minority];
        let cweight = weights[child];

        return Response::Delta(cweight + majority - minority);
    }
}
