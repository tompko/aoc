fn digits(a: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut a = a;
    while a > 0 {
        ret.insert(0, a % 10);
        a /= 10;
    }
    ret
}

fn adjacent(a: &Vec<usize>) -> bool {
    for i in 1..a.len() {
        if a[i-1] == a[i] {
            return true;
        }
    }
    false
}

fn increasing(a: &Vec<usize>) -> bool {
    for i in 1..a.len() {
        if a[i-1] > a[i] {
            return false;
        }
    }
    true
}

fn run(a: &Vec<usize>) -> bool {
    let mut run_len = 0;
    let mut current = 0;
    for &d in a.iter() {
        if d == current {
            run_len += 1;
        } else {
            if run_len == 2 {
                return true;
            }
            run_len = 1;
            current = d;
        }
    }

    run_len == 2
}

fn main() {
    let part1 = (382345..843167).
        map(digits).
        filter(adjacent).
        filter(increasing).
        count();
    let part2 = (382345..843167).
        map(digits).
        filter(adjacent).
        filter(increasing).
        filter(run).
        count();


    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
