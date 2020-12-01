const STEP: usize = 356;

fn main() {
    let mut buffer = Vec::new();
    let mut position = 0;
    buffer.push(0);

    for i in 1..2018 {
        position = (position + STEP) % buffer.len();
        buffer.insert(position + 1, i);
        position += 1;
    }

    let pos = buffer.iter().position(|&x| x == 2017).unwrap();
    println!("part 1: {}", buffer[pos + 1]);

    position = 0;
    let mut part2 = 0;
    for len in 1..50000001 {
        position = ((position+STEP) % len) + 1;
        if position == 1 {
            part2 = len;
        }
    }

    println!("part 2: {}", part2);
}
