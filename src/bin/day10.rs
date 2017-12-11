use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("input/day10.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let contents = contents.trim();

    println!("part 1: {}", part1(contents));
    println!("part 2: {}", hash(&contents));
}

fn part1(contents: &str) -> usize {
    let mut input = (0..256).collect();
    let part1_lengths = contents.split(",").
        map(|x| x.parse().unwrap()).
        collect::<Vec<_>>();

    round(&mut input, 0, 0, &part1_lengths);

    input[0] * input[1]
}

fn hash(input: &str) -> String {
    let lengths = input.chars().
        map(|x| x as usize).
        chain(vec![17, 31, 73, 47, 23].into_iter()).
        collect::<Vec<_>>();
    let mut input = (0..256).collect();
    let mut pos = 0;
    let mut skip = 0;

    for _ in 0..64 {
        let (p, s) = round(&mut input, pos, skip, &lengths);
        pos = p;
        skip = s;
    }

    let mut ret = Vec::new();

    for i in 0..16 {
        let base_index = i * 16;
        let mut r = 0;
        for j in 0..16 {
            r = r ^ input[base_index + j];
        }

        ret.push(r);
    }

    ret.iter().
        map(|x| format!("{:02x}", x)).
        collect()
}

fn round(input: &mut Vec<usize>, position: usize, skip: usize, lengths: &[usize]) -> (usize, usize) {
    let mut pos = position;
    let mut skip = skip;
    let len = input.len();

    for l in lengths {
        let mut start = pos;
        let mut stop = (pos + l - 1) % len;

        for _ in 0..(l/2) {
            input.swap(start, stop);
            start = (start + 1) % len;
            stop = (stop + len - 1) % len;
        }

        pos = (pos + l + skip) % len;
        skip += 1;
    }

    (pos, skip)
}
