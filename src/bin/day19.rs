use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

struct Replacement {
    before: String,
    after: String,
}

fn replace(med: &str, rep: &Replacement) -> Vec<String> {
    let mut meds: Vec<String> = Vec::new();

    let parts: Vec<&str> = med.split(&rep.before).collect();
    for i in 1..parts.len() {
        let mut new_med = parts[0].to_string();
        for j in 1..parts.len() {
            if i == j {
                new_med = new_med + &rep.after;
            } else {
                new_med = new_med + &rep.before;
            }

            new_med = new_med + parts[j];
        }
        meds.push(new_med);
    }

    meds
}

fn search(med: &str, reps: &Vec<Replacement>) -> u32 {
    struct State {
        molecule: String,
        steps: u32
    }
    let mut queue: Vec<State> = Vec::new();
    queue.push(
        State{
            molecule: med.to_string(),
            steps: 0,
        }
    );

    while !queue.is_empty() {
        let s = queue.pop().unwrap();

        if s.molecule == "e" {
            return s.steps
        }

        for r in reps {
            for m in replace(&s.molecule, &r) {
                queue.push(
                    State{
                        molecule: m.clone(),
                        steps: s.steps + 1,
                    }
                );
            }
        }
        queue.sort_by(|a, b| b.molecule.len().cmp(&a.molecule.len()));
    }
    unreachable!();
}

fn main() {
    let f = File::open("day19.in")
        .ok()
        .expect("Error opening input");
    let file = BufReader::new(&f);

    let mut in_replacements = true;
    let mut replacements: Vec<Replacement> = Vec::new();
    let mut medecine: String = String::new();

    for l in file.lines() {
        let l = l.unwrap();
        if in_replacements {
            if l == "" {
                in_replacements = false;
            } else {
                let parts: Vec<&str> = l.trim().split(" => ").collect();
                replacements.push(Replacement{
                    before: parts[0].to_string(),
                    after: parts[1].to_string(),
                });
            }
        } else {
            medecine = l.trim().to_string();
        }
    }

    let mut meds = HashSet::new();
    for r in &replacements {
        for m in replace(&medecine, &r) {
            meds.insert(m);
        }
    }
    println!("{}", meds.len());

    let mut reversed: Vec<Replacement> = Vec::new();
    for r in &replacements {
        reversed.push(Replacement{
            before: r.after.clone(),
            after: r.before.clone(),
        });
    }
    let steps = search(&medecine, &reversed);
    println!("{}", steps)
}
