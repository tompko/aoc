use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Immune,
    Infect,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Attack {
    Bludgeon,
    Slash,
    Radiation,
    Fire,
    Cold,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Immune(usize, usize),
    Infect(usize, usize),
    Draw,
}

struct Battle {
    armies: Vec<Army>,
    group_ids: Vec<usize>,
    rounds: usize,
    outcome: Option<Outcome>,
}

struct Army {
    side: Side,
    units: usize,
    hp: usize,
    weak: Vec<Attack>,
    immune: Vec<Attack>,
    damage: usize,
    attack: Attack,
    initiative: usize,
}

impl fmt::Debug for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Immune => write!(f, "Immune System"),
            Side::Infect => write!(f, "Infection"),
        }
    }
}

impl Battle {
    fn new() -> Self {
        Battle { armies: Vec::new(), group_ids: Vec::new(), rounds: 0, outcome: None }
    }

    fn add_army(&mut self, army: Army) {
        let mut i = 1;
        for a in self.armies.iter() {
            if a.side == army.side {
                i += 1;
            }
        }
        self.group_ids.push(i);
        self.armies.push(army);
    }

    fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    fn play_round(&mut self) {
        let num_armies = self.armies.len();
        let mut order: Vec<_> = (0..num_armies).collect();
        order.sort_by_key(|&x| (self.armies[x].effective_power(), self.armies[x].initiative));
        order.reverse();
        let mut targets = HashMap::new();
        let mut taken = HashSet::new();
        let mut did_damage = false;

        for o in order.into_iter() {
            let mut best_target = o;
            let mut best_damage = 0;

            if self.armies[o].units == 0 {
                continue
            }

            for e in 0..num_armies {
                if (self.armies[o].side == self.armies[e].side) || taken.contains(&e) || self.armies[e].units == 0 {
                    continue
                }

                let d = Battle::damage(&self.armies[o], &self.armies[e]);
                if d == 0 {
                    continue;
                }

                if (d > best_damage) || 
                    (d == best_damage && self.armies[e].effective_power() > self.armies[best_target].effective_power()) || 
                        (d == best_damage && self.armies[e].effective_power() == self.armies[best_target].effective_power() && self.armies[e].initiative > self.armies[best_target].initiative) {
                    best_damage = d;
                    best_target = e;
                }
            }

            if best_target != o {
                targets.insert(o, best_target);
                taken.insert(best_target);
            }
        }

        let mut order: Vec<_> = (0..num_armies).collect();
        order.sort_by_key(|&x| self.armies[x].initiative);
        order.reverse();

        for o in order.into_iter() {
            if let Some(&t) = targets.get(&o) {
                let d = Battle::damage(&self.armies[o], &self.armies[t]);
                let u = min(d / self.armies[t].hp, self.armies[t].units);
                if u > 0 {
                    did_damage = true;
                }
                self.armies[t].units = self.armies[t].units.saturating_sub(u);
            }
        }

        self.rounds += 1;
        if !did_damage {
            self.outcome = Some(Outcome::Draw);
        }

        let mut immune = 0;
        let mut infect = 0;
        for a in self.armies.iter() {
            match a.side {
                Side::Immune => immune += a.units,
                Side::Infect => infect += a.units,
            }
        }

        if immune == 0 {
            self.outcome = Some(Outcome::Infect(self.rounds, infect));
        }
        if infect == 0 {
            self.outcome = Some(Outcome::Immune(self.rounds, immune));
        }
    }

    fn damage(attacker: &Army, defender: &Army) -> usize {
        for &i in defender.immune.iter() {
            if i == attacker.attack {
                return 0;
            }
        }

        let d = attacker.effective_power();

        for &w in defender.weak.iter() {
            if w == attacker.attack {
                return d * 2;
            }
        }

        d
    }
}

impl Army {
    fn effective_power(&self) -> usize {
        self.units * self.damage
    }
}

fn main() {
    let part1 = battle(0);

    match part1 {
        Outcome::Infect(_, units) => println!("part 1: {}", units),
        _ => unreachable!(),
    }

    for i in 1..1000 {
        match battle(i) {
            Outcome::Immune(_, units) => { println!("part 2: {}", units); break; },
            _ => (),
        }
    }
}

fn battle(boost: usize) -> Outcome {
    let mut battle = Battle::new();
    battle.add_army(Army{
        side: Side::Immune,
        units: 4647,
        hp: 7816, 
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 13 + boost,
        attack: Attack::Fire,
        initiative: 1,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 301,
        hp: 3152,
        weak: vec![Attack::Fire],
        immune: Vec::new(),
        damage: 104 + boost,
        attack: Attack::Fire,
        initiative: 3,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 1508,
        hp: 8344,
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 50 + boost,
        attack: Attack::Cold,
        initiative: 9,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 2956,
        hp: 5021,
        weak: vec![Attack::Slash],
        immune: vec![Attack::Bludgeon],
        damage: 13 + boost,
        attack: Attack::Slash,
        initiative: 15,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 898,
        hp: 11545,
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 100 + boost,
        attack: Attack::Cold,
        initiative: 2,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 207,
        hp: 6235,
        weak: vec![Attack::Cold],
        immune: Vec::new(),
        damage: 242 + boost,
        attack: Attack::Slash,
        initiative: 17,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 7550,
        hp: 8773,
        weak: vec![Attack::Fire, Attack::Slash],
        immune: vec![Attack::Radiation],
        damage: 11 + boost,
        attack: Attack::Radiation,
        initiative: 11,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 1057,
        hp: 3791,
        weak: Vec::new(),
        immune: vec![Attack::Cold],
        damage: 27 + boost,
        attack: Attack::Bludgeon,
        initiative: 5,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 5086,
        hp: 3281,
        weak: vec![Attack::Bludgeon],
        immune: Vec::new(),
        damage: 5 + boost,
        attack: Attack::Cold,
        initiative: 19,
    });
    battle.add_army(Army{
        side: Side::Immune,
        units: 330,
        hp: 4136,
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 91 + boost,
        attack: Attack::Cold,
        initiative: 6,
    });

    battle.add_army(Army{
        side: Side::Infect,
        units: 1755,
        hp: 6886,
        weak: Vec::new(),
        immune: vec![Attack::Slash, Attack::Radiation],
        damage: 6,
        attack: Attack::Fire,
        initiative: 4,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 2251,
        hp: 33109,
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 29,
        attack: Attack::Cold,
        initiative: 7,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 298,
        hp: 18689,
        weak: vec![Attack::Radiation, Attack::Slash],
        immune: Vec::new(),
        damage: 123,
        attack: Attack::Slash,
        initiative: 13,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 312,
        hp: 15735,
        weak: vec![Attack::Bludgeon, Attack::Slash],
        immune: Vec::new(),
        damage: 99,
        attack: Attack::Cold,
        initiative: 8,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 326,
        hp: 16400,
        weak: vec![Attack::Bludgeon],
        immune: Vec::new(),
        damage: 98,
        attack: Attack::Radiation,
        initiative: 20,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 4365,
        hp: 54947,
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 22,
        attack: Attack::Cold,
        initiative: 14,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 1446,
        hp: 51571,
        weak: vec![Attack::Cold],
        immune: Vec::new(),
        damage: 63,
        attack: Attack::Fire,
        initiative: 18,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 8230,
        hp: 12331,
        weak: vec![Attack::Bludgeon],
        immune: vec![Attack::Slash],
        damage: 2,
        attack: Attack::Fire,
        initiative: 12,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 4111,
        hp: 17381,
        weak: Vec::new(),
        immune: Vec::new(),
        damage: 7,
        attack: Attack::Cold,
        initiative: 10,
    });
    battle.add_army(Army{
        side: Side::Infect,
        units: 366,
        hp: 28071,
        weak: vec![Attack::Cold, Attack::Slash],
        immune: Vec::new(),
        damage: 150,
        attack: Attack::Fire,
        initiative: 16,
    });

    while !battle.is_over() {
        battle.play_round();
    }

    battle.outcome.unwrap()
}
