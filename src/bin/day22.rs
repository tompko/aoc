use std::cmp::{min, max};

#[derive(Debug)]
enum Spell {
    MagicMissile = 0,
    Drain = 1,
    Shield = 2,
    Poison = 3,
    Recharge = 4,
}

static SPELLS: [Spell; 5] = [Spell::MagicMissile, Spell::Drain, Spell::Shield, Spell::Poison, Spell::Recharge];
static SPELLCOST: [i32; 5] = [53, 73, 113, 173, 229];

const BOSS_HP: i32 = 55;
const BOSS_DAMAGE: i32 = 8;
const PLAYER_HP: i32 = 50;
const PLAYER_MANA: i32 = 500;

fn round(
    mut bosshp: i32,
    mut playerhp: i32,
    mut mana: i32,
    mut cost: i32,
    mut shield: i32,
    mut poison: i32,
    mut recharge: i32,
    spell: &Spell,
    best_cost: i32,
    hard_mode: bool,
) -> i32 {
    if cost >= best_cost {
        return best_cost;
    }

    let mut armor: i32 = 0;
    let c;
    match spell {
        &Spell::MagicMissile => {
            bosshp -= 4;
            c = 53;
        }
        &Spell::Drain => {
            bosshp -= 2;
            playerhp += 2;
            c = 73;
        },
        &Spell::Shield => {
            if shield == 0 {
                shield = 6;
            } else {
                return best_cost;
            }
            c = 113;
        },
        &Spell::Poison => {
            if poison == 0 {
                poison = 6;
            } else {
                return best_cost;
            }
            c = 173;
        },
        &Spell::Recharge => {
            if recharge == 0 {
                recharge = 5;
            } else {
                return best_cost;
            }
            c = 229;
        }
    };

    mana -= c;
    cost += c;

    // boss turn
    if shield > 0 {
        armor = 7;
        shield -= 1;
    }
    if poison > 0 {
        bosshp -= 3;
        poison -= 1;
    }
    if recharge > 0 {
        mana += 101;
        recharge -= 1;
    }
    if bosshp <= 0 {
        return cost;
    } else {
        playerhp -= max(1, BOSS_DAMAGE - armor);
    }

    if hard_mode {
        playerhp -= 1;
    }

    if playerhp <= 0 {
        return best_cost;
    }

    // player turn
    if shield > 0 {
        shield -= 1;
    }
    if poison > 0 {
        bosshp -= 3;
        poison -= 1;
    }
    if recharge > 0 {
        mana += 101;
        recharge -= 1;
    }
    if bosshp <= 0 {
        return cost;
    }

    let mut ret = best_cost;
    for (i, s) in SPELLS.iter().enumerate() {
        if mana < SPELLCOST[i] {
            continue
        }
        let r = round(bosshp, playerhp, mana, cost, shield, poison, recharge, &s, ret, hard_mode);
        ret = min(ret, r)
    }

    ret
}

fn main() {
    let mut best_cost = 1000000;
    for s in SPELLS.iter() {
        let score = round(BOSS_HP, PLAYER_HP, PLAYER_MANA, 0, 0, 0, 0, s, best_cost, false);
        best_cost = min(best_cost, score);
    }
    println!("{}", best_cost);

    best_cost = 1000000;
    for s in SPELLS.iter() {
        let score = round(BOSS_HP, PLAYER_HP - 1, PLAYER_MANA, 0, 0, 0, 0, s, best_cost, true);
        best_cost = min(best_cost, score);
    }
    println!("{}", best_cost);
}
