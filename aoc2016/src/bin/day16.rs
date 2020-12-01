const INPUT: &'static str = "11100010111110100";

fn dragon(state: &str) -> String {
    let end: String = state.chars().map(|x| if x == '0' {'1'} else {'0'}).rev().collect();
    state.to_owned() + "0" + &end
}

fn checksum(state: &str) -> String {
    let mut s = state.to_owned();

    while s.len() % 2 == 0 {
        s = s.chars().collect::<Vec<_>>().chunks(2).map(|x| if x[0] == x[1] { '1' } else { '0' }).collect();
    }

    s
}

fn fill_disk(start: &str, size: usize) -> String {
    let mut start = start.to_owned();

    while start.len() < size {
        start = dragon(&start);
    }

    start.truncate(size);

    checksum(&start)
}

fn main() {
    println!("1: {}", fill_disk(INPUT, 272));
    println!("2: {}", fill_disk(INPUT, 35651584));
}
