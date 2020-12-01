use std::cmp::{min, max};

#[derive(PartialEq, Eq)]
struct Item {
    name: &'static str,
    cost: i32,
    damage: i32,
    armor: i32,
}

const WEAPONS: [Item; 5] = [
    Item{name: "Dagger", cost: 8, damage: 4, armor: 0},
    Item{name: "Shortsword", cost: 10, damage: 5, armor: 0},
    Item{name: "Warhammer", cost: 25, damage: 6, armor: 0},
    Item{name: "Longsword", cost: 40, damage: 7, armor: 0},
    Item{name: "Greataxe", cost: 74, damage: 8, armor: 0},
];

const ARMOR: [Item; 6] = [
    Item{name: "None", cost: 0, damage: 0, armor: 0},
    Item{name: "Leather", cost: 13, damage: 0, armor: 1},
    Item{name: "Chainmail", cost: 31, damage: 0, armor: 2},
    Item{name: "Splintmail", cost: 53, damage: 0, armor: 3},
    Item{name: "Bandedmail", cost: 75, damage: 0, armor: 4},
    Item{name: "Platemail", cost: 102, damage: 0, armor: 5},
];

const RINGS: [Item; 8] = [
    Item{name: "None", cost: 0, damage: 0, armor: 0},
    Item{name: "None", cost: 0, damage: 0, armor: 0},
    Item{name: "Damage +1", cost: 25, damage: 1, armor: 0},
    Item{name: "Damage +2", cost: 50, damage: 2, armor: 0},
    Item{name: "Damage +3", cost: 100, damage: 3, armor: 0},
    Item{name: "Defense +1", cost: 20, damage: 0, armor: 1},
    Item{name: "Defense +2", cost: 40, damage: 0, armor: 2},
    Item{name: "Defense +3", cost: 80, damage: 0, armor: 3},
];

const BOSS_HIT_POINTS: i32 = 100;
const BOSS_ARMOR: i32 = 2;
const BOSS_DAMAGE: i32 = 8;

const PLAYER_HIT_POINTS: i32 = 100;

fn simulate(player_damage: i32, player_armor: i32) -> bool {
    let mut boss_health = BOSS_HIT_POINTS;
    let mut player_health = PLAYER_HIT_POINTS;

    loop {
        boss_health -= max(1, player_damage - BOSS_ARMOR);
        if boss_health <= 0 {
            return true;
        }

        player_health -= max(1, BOSS_DAMAGE - player_armor);
        if player_health <= 0 {
            return false;
        }
    }
}

fn main() {
    let mut best_win = 1000000;
    let mut worst_loss = 0;

    for w in WEAPONS.into_iter() {
        for a in ARMOR.into_iter() {
            for r in RINGS.into_iter() {
                for s in RINGS.into_iter() {
                    if r == s {
                        continue
                    }
                    let cost = w.cost + a.cost + r.cost + s.cost;
                    let pl_dam = w.damage + a.damage + r.damage + s.damage;
                    let pl_arm = w.armor + a.armor + r.armor + s.armor;

                    if simulate(pl_dam, pl_arm) {
                        best_win = min(best_win, cost);
                    } else {
                        worst_loss = max(worst_loss, cost);
                    }
                }
            }
        }
    }

    println!("{}", best_win);
    println!("{}", worst_loss);
}
