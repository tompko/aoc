#[derive(Debug, Clone, Copy)]
struct Generator {
    last: u64,
    factor: u64,
}

impl Iterator for Generator {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        let n = (self.last * self.factor) % 2147483647;
        self.last = n;

        Some(n as u16)
    }
}

fn generator(n: u64, f: u64) -> Generator {
    Generator{
        last: n,
        factor: f,
    }
}

fn main() {
    let a = generator(634, 16807);
    let b = generator(301, 48271);

    let part1 = a.zip(b).
        take(40000000).
        filter(|&(x, y)| x == y).
        count();

    let part2 = a.filter(|x| (x % 4) == 0).zip(b.filter(|x| (x % 8) == 0)).
        take(5000000).
        filter(|&(x, y)| x == y).
        count();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
