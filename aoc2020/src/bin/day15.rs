use std::collections::HashMap;

const START: [usize; 7] = [0,6,1,7,2,19,20];

struct Game {
    memory: HashMap<usize, usize>,
    num_turns: usize,
    prev_num: usize,
    prev_age: Option<usize>,
}

impl Game {
    fn new() -> Self {
        Game {
            memory: HashMap::new(),
            num_turns: 0,
            prev_num: 0,
            prev_age: None,
        }
    }

    fn play(&mut self, turn: usize) {
        self.num_turns += 1;
        self.prev_num = turn;
        self.prev_age = if let Some(&u) = self.memory.get(&turn) { Some(u) } else { None };
        self.memory.insert(turn, self.num_turns);
    }

    fn next(&self) -> usize{
        if self.prev_age.is_none() {
            return 0;
        } else {
            return self.num_turns - self.prev_age.unwrap();
        }
    }
}


fn main() {
    let mut g = Game::new();
    for &v in START.iter() {
        g.play(v);
    }

    while g.num_turns < 2019 {
        let n = g.next();
        g.play(n);
    }

    println!("Part 1: {}", g.next());

    while g.num_turns < 29999999 {
        let n = g.next();
        g.play(n);
    }

    println!("Part 2: {}", g.next());
}