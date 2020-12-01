use std::collections::HashSet;

const INITIAL: &'static str = "##.#############........##.##.####..#.#..#.##...###.##......#.#..#####....##..#####..#.#.##.#.##";

#[derive(Debug, Clone)]
struct Flowers {
    minx: isize,
    pots: HashSet<isize>,
}

impl Flowers {
    fn new(state: &str) -> Self {
        let mut pots = HashSet::new();

        for (i, c) in state.chars().enumerate() {
            if c == '#' {
                pots.insert(i as isize);
            }
        }

        let minx = *pots.iter().min().unwrap();
        let pots = pots.iter().map(|x| x - minx).collect();

        Flowers { minx, pots }
    }

    fn step(self, rules: &HashSet<String>) -> Self {
        let mut pots = HashSet::new();

        let min: isize = *self.pots.iter().min().unwrap();
        let max: isize = *self.pots.iter().max().unwrap();

        for i in (min - 5)..(max + 5) {
            let mut frame = String::new();
            for j in -2..=2 {
                if self.pots.contains(&(i+j)) {
                    frame.push('#');
                } else {
                    frame.push('.');
                }
            }

            if rules.contains(&frame) {
                pots.insert(i);
            }
        }

        let minx = pots.iter().min().unwrap();
        let pots: HashSet<_> = pots.iter().map(|x| x - minx).collect();

        Flowers{ minx: self.minx+minx, pots }
    }

    fn score(&self) -> isize {
        (self.minx * self.pots.len() as isize) + self.pots.iter().sum::<isize>() 
    }
}

fn main() {
    let rules: HashSet<String> = [
        ("###.#", '#'),
        (".####", '#'),
        ("#.###", '.'),
        (".##..", '.'),
        ("##...", '#'),
        ("##.##", '#'),
        (".#.##", '#'),
        ("#.#..", '#'),
        ("#...#", '.'),
        ("...##", '#'),
        ("####.", '#'),
        ("#..##", '.'),
        ("#....", '.'),
        (".###.", '.'),
        ("..#.#", '.'),
        ("..###", '.'),
        ("#.#.#", '#'),
        (".....", '.'),
        ("..##.", '.'),
        ("##.#.", '#'),
        (".#...", '#'),
        ("#####", '.'),
        ("###..", '#'),
        ("..#..", '.'),
        ("##..#", '#'),
        ("#..#.", '#'),
        ("#.##.", '.'),
        ("....#", '.'),
        (".#..#", '#'),
        (".#.#.", '#'),
        (".##.#", '.'),
        ("...#.", '.'),
    ].iter()
        .filter_map(|&(k, v)| if v == '#' { Some(k.to_owned()) } else { None })
        .collect();

    let mut flowers = Flowers::new(INITIAL);

    for _ in 0..20 {
        flowers = flowers.step(&rules);
    }

    println!("part 1: {}", flowers.score());
    println!("part 2: {}", 2650000000466_u64);

}
