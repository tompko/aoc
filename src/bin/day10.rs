extern crate itertools;

use itertools::Itertools;

fn look_and_say(seq: &str) -> String {
    seq.chars()
        .into_iter()
        .group_by(|a| a.clone())
        .map(|i| i.1.len().to_string() + &i.0.to_string())
        .fold("".to_string(), |a, b| a + &b)
}

fn main() {
    let mut state = "1113222113".to_string();

    for _ in 0..40 {
        state = look_and_say(&state);
    }
    println!("{}", state.len());

    for _ in 0..10 {
        state = look_and_say(&state);
    }
    println!("{}", state.len());
}
