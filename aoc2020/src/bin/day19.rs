use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub trait Rule {
    fn min(&self) -> usize;
    fn max(&self) -> usize;

    fn matches(&self, s: &str) -> bool;
}

pub struct Literal {
    s: String,
}

pub struct Concat {
    left: Box<dyn Rule>,
    right: Box<dyn Rule>,
}

pub struct Optional {
    left: Box<dyn Rule>,
    right: Box<dyn Rule>,
}

pub struct Recursive {
    pre: Box<dyn Rule>,
    post: Box<dyn Rule>,
}

pub struct Null { }

impl Rule for Literal {
    fn min(&self) -> usize {
        self.s.len()
    }

    fn max(&self) -> usize {
        self.s.len()
    }

    fn matches(&self, s: &str) -> bool {
        self.s == s
    }
}

impl Rule for Concat {
    fn min(&self) -> usize {
        self.left.min() + self.right.min()
    }

    fn max(&self) -> usize {
        self.left.max() + self.right.max()
    }

    fn matches(&self, s: &str) -> bool {
        for i in self.left.min()..=self.left.max() {
            if i > s.len() {
                break;
            }
            let (l, r) = s.split_at(i);
            if self.left.matches(l) && self.right.matches(r) {
                return true;
            }
        }
        false
    }
}

impl Rule for Optional {
    fn min(&self) -> usize {
        cmp::min(self.left.min(), self.right.min())
    }

    fn max(&self) -> usize {
        cmp::max(self.left.max(), self.right.max())
    }

    fn matches(&self, s: &str) -> bool {
        self.left.matches(s) || self.right.matches(s)
    }
}

impl Rule for Recursive {
    fn min(&self) -> usize {
        self.pre.min() + self.post.min()
    }

    fn max(&self) -> usize {
        usize::MAX
    }

    fn matches(&self, s: &str) -> bool {
        for i in self.pre.min()..=self.pre.max() {
            if i > s.len() {
                break;
            }
            let (l, r) = s.split_at(i);
            if !self.pre.matches(l) {
                continue;
            }
            if self.pre.matches(l) && self.post.matches(r) {
                return true;
            }
            let start = if self.post.max() > r.len() { r.len() } else { r.len() - self.post.max() };
            let end = if self.post.min() > r.len() { r.len() } else { r.len() - self.post.min() };
            for j in start..=end {
                let (sl, sr) = r.split_at(j);
                if self.matches(sl) && self.post.matches(sr) {
                    return true;
                }
            }
        }
        false
    }
}

impl Rule for Null {
    fn min(&self) -> usize {
        0
    }

    fn max(&self) -> usize {
        0
    }

    fn matches(&self, s: &str) -> bool {
        s.is_empty()
    }
}

fn parse_rule(index: usize, rule: &str, rule_strings: &HashMap<usize, String>) -> Box<dyn Rule> {
    let recursive = rule.split_ascii_whitespace().filter_map(|s| s.parse::<usize>().ok()).filter(|&s| s == index).count() != 0;

    if recursive {
        if index == 8 {
            let sub_index = 42;
            let sub_rule = &rule_strings[&sub_index];
            let r = parse_rule(sub_index, sub_rule, rule_strings);

            Box::new(Recursive{ pre: r, post: Box::new(Null{})})
        } else {
            let pre_index = 42;
            let pre_rule = &rule_strings[&pre_index];
            let pre = parse_rule(pre_index, pre_rule, rule_strings);

            let post_index = 31;
            let post_rule = &rule_strings[&post_index];
            let post = parse_rule(post_index, post_rule, rule_strings);

            Box::new(Recursive{pre: pre, post: post})
        }
    } else if rule.contains("|") {
        let splits: Vec<_> = rule.split("|").collect();
        assert_eq!(splits.len(), 2);
        let left = parse_rule(index, splits[0], rule_strings);
        let right = parse_rule(index, splits[1], rule_strings);
        Box::new(Optional{ left: left, right: right })
    } else if rule.contains("\"") {
        let t: &[_] = &['"', ' '];
        Box::new(Literal{ s: rule.trim_matches(t).to_string() })
    } else  {
        let splits: Vec<_> = rule.trim().split(' ').collect();
        let sub_index = splits[0].parse::<usize>().unwrap();
        let sub_rule = &rule_strings[&sub_index];
        let mut r = parse_rule(sub_index, sub_rule, rule_strings);
        for s in splits.iter().skip(1) {
            let sub_index = s.parse::<usize>().unwrap();
            let sub_rule = &rule_strings[&sub_index];
            let n = parse_rule(sub_index, sub_rule, rule_strings);
            r = Box::new(Concat{left: r, right: n});
        }
        r
    }
}

fn main() {
    let file = File::open("input/day19.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut rules_strings: HashMap::<usize, String> = HashMap::new();
    let mut messages = Vec::new();
    let mut lines = file.lines();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line == "" {
            break;
        }
        let splits: Vec::<_> = line.split(":").collect();
        let index = splits[0].parse::<usize>().unwrap();

        rules_strings.insert(index, splits[1].to_string());
    }
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        messages.push(line);
    }

    let mut part1 = 0;
    let rule0 = parse_rule(0, &rules_strings[&0], &rules_strings);
    for m in messages.iter() {
        if rule0.matches(m) {
            part1 += 1;
        }
    }

    rules_strings.insert(8, "42 | 42 8".to_string());
    rules_strings.insert(11, "42 31 | 42 11 31".to_string());
    let mut part2 = 0;
    let rule0 = parse_rule(0, &rules_strings[&0], &rules_strings);
    for m in messages.iter() {
        if rule0.matches(m) {
            part2 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}