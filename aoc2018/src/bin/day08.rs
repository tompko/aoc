use std::fs::File;
use std::io::Read;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn new<I: Iterator<Item=usize>>(ns: &mut I) -> Self {
        let num_children = ns.next().unwrap();
        let num_metadata = ns.next().unwrap();

        let mut children = Vec::new();
        let mut metadata = Vec::new();

        for _ in 0..num_children {
            children.push(Node::new(ns));
        }
        for _ in 0..num_metadata {
            metadata.push(ns.next().unwrap());
        }

        Node { children, metadata }
    }

    fn sum_metadata(&self) -> usize {
        self.metadata.iter().sum::<usize>() +
            self.children.iter().map(|c| c.sum_metadata()).sum::<usize>()
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            let child_values: Vec<_> = self.children.iter().map(|c| c.value()).collect();
            self.metadata.iter()
                .filter(|&&m| m > 0 && m <= child_values.len())
                .map(|&m| child_values[m - 1])
                .sum()
        }
    }
}

fn main() {
    let mut file = File::open("input/day08.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");

    let contents: Vec<_> = contents.trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let node = Node::new(&mut contents.into_iter());

    println!("part 1: {}", node.sum_metadata());
    println!("part 2: {}", node.value());
}
