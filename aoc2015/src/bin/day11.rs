const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const INITIAL_PASSWORD: &str = "cqjxjnds";
const FORBIDDEN: &str = "iol";

struct Validator {
    triples: Vec<String>,
    pairs: Vec<String>,
    forbidden: Vec<String>,
}

impl Validator {
    fn new() -> Self {
        Validator{
            triples: ALPHABET.chars().zip(ALPHABET.chars().skip(1)).zip(ALPHABET.chars().skip(2)).map(|((x,y),z)| x.to_string()+&y.to_string()+&z.to_string()).collect(),
            pairs: ALPHABET.chars().zip(ALPHABET.chars()).map(|(x,y)| x.to_string()+&y.to_string()).collect(),
            forbidden: FORBIDDEN.chars().map(|x| x.to_string()).collect(),
        }
    }

    fn validate(&self, p: &str) -> bool {
        if self.forbidden.iter().map(|f| p.contains(f)).any(|x| x) {
            return false;
        }

        if !self.triples.iter().map(|t| p.contains(t)).any(|x| x) {
            return false;
        }

        let count: u32 = self.pairs.iter().map(|x| if p.contains(x) { 1 } else { 0 }).sum();

        if count < 2 {
            return false;
        }

        true
    }
}

fn increment_pass(p: &str) -> String {
    let mut ps: Vec<_> = p.chars().map(|x| x as u32 - 'a' as u32).collect();

    for i in (0..ps.len()).rev() {
        ps[i] += 1;
        if ps[i] <= 25 {
            break;
        }
        ps[i] -= 26;
    }

    ps.into_iter().map(|p| std::char::from_u32(p + 'a' as u32).unwrap()).collect()
}

fn main() {
    let v = Validator::new();

    let mut p = INITIAL_PASSWORD.to_owned();
    while !v.validate(&p) {
        p = increment_pass(&p);
    }
    println!("Part 1: {}", p);
    p = increment_pass(&p);
    while !v.validate(&p) {
        p = increment_pass(&p);
    }
    println!("Part 2: {}", p);
}
