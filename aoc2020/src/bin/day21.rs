use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day21.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut allergen_sets: HashMap<String, HashSet<String>> = HashMap::new();
    let mut allergens: HashMap<String, String> = HashMap::new();
    let mut all_ingredients: Vec<String> = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();

        let splits: Vec<_> = line.split(" (contains ").collect();
        let ingredients: HashSet<_> = splits[0].split(" ").map(|s| s.to_string()).collect();
        let allergy = splits[1].trim_end_matches(')').split(", ");

        for i in ingredients.iter() {
            all_ingredients.push(i.to_string());
        }

        for a in allergy {
            if allergen_sets.contains_key(a) {
                let new_set: HashSet<_> = allergen_sets.get(a).unwrap().intersection(&ingredients).cloned().collect();
                allergen_sets.insert(a.to_string(), new_set);
            } else {
                allergen_sets.insert(a.to_string(), ingredients.clone());
            }
        }
    }

    while !allergen_sets.is_empty() {
        for (k, v) in allergen_sets.iter() {
            if v.len() == 1 {
                let ingredient = v.iter().next().unwrap();
                allergens.insert(ingredient.to_string(), k.to_string());
            }
        }

        for (ingredient, _) in allergens.iter() {
            let mut to_remove = Vec::new();
            for (k, v) in allergen_sets.iter_mut() {
                v.remove(ingredient);
                if v.len() == 0 {
                    to_remove.push(k.to_string());
                }
            }
            for k in to_remove {
                allergen_sets.remove(&k);
            }
        }
    }

    let part1 = all_ingredients.iter().filter(|&i| !allergens.contains_key(i)).count();
    let mut allergen_list: Vec<_> = allergens.iter().map(|(i, a)| (a, i)).collect();
    allergen_list.sort();
    let part2 = allergen_list.iter().map(|(_, i)| i.to_string()).collect::<Vec<String>>().join(",");

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}