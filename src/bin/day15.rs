fn solve(discs: &[(usize, usize)]) -> usize {
    let mut start = 0;

    loop {
        if discs.iter().enumerate().all(|(i, &(t, s))| (s + i + 1 + start) % t == 0) {
            return start;
        }

        start += 1;
    }
}

fn main() {
    let mut input = vec![
        (13, 1),
        (19, 10),
        (3, 2),
        (7, 1),
        (5, 3),
        (17, 5),
    ];

    let part1 = solve(&input);
    println!("1: {}", part1);

    input.push((11, 0));

    let part2 = solve(&input);
    println!("2: {}", part2);
}
