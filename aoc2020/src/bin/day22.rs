use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq)]
enum Winner {
    Player1,
    Player2,
}

fn combat(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> VecDeque<usize> {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() { deck2 } else { deck1 }
}

fn recursive_combat(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> (VecDeque<usize>, Winner) {
    let mut seen: HashSet<String> = HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        let k = key(&deck1, &deck2);
        if seen.contains(&k) {
            return (deck1, Winner::Player1);
        }
        seen.insert(k);
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let winner = if card1 <= deck1.len() && card2 <= deck2.len() {
            let d1 = deck1.iter().take(card1).cloned().collect();
            let d2 = deck2.iter().take(card2).cloned().collect();
            recursive_combat(d1, d2).1
        } else {
            if card1 > card2 { Winner::Player1 } else { Winner::Player2 }
        };

        if winner == Winner::Player1 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() { (deck2, Winner::Player2 ) } else { (deck1, Winner::Player1) }
}

fn score(deck: &VecDeque<usize>) -> usize {
    let mut score = 0;
    for (i, d) in (0..deck.len()).rev().zip(deck.iter()) {
        score += (i+1) * d;
    }
    score
}

fn key(deck1: &VecDeque<usize>, deck2: &VecDeque<usize>) -> String {
    let d1 = deck1.iter().map(|d| format!("{}", d)).collect::<Vec<_>>().join(",");
    let d2 = deck2.iter().map(|d| format!("{}", d)).collect::<Vec<_>>().join(",");

    d1 + "/" + &d2
}

fn main() {
    let file = File::open("input/day22.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut decks: HashMap<usize, VecDeque<usize>> = HashMap::new();
    let mut player = 0;

    for line in file.lines() {
        let line = line.unwrap();

        if line.contains("Player") {
            player = line.trim_start_matches("Player ").trim_end_matches(":").parse::<usize>().unwrap();
        } else if line != "" {
            decks.entry(player).or_insert(VecDeque::new()).push_back(line.parse::<usize>().unwrap());
        }
    }

    let deck1 = decks.get(&1).unwrap();
    let deck2 = decks.get(&2).unwrap();

    let part1_deck = combat(deck1.clone(), deck2.clone());
    let (part2_deck, _) = recursive_combat(deck1.clone(), deck2.clone());

    let part1 = score(&part1_deck);
    let part2 = score(&part2_deck);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}