use std::io::Read;
use std::fs::File;

fn square(a: char, b: char, c: char) -> char {
    if (a == '^' && b == '^' && c == '.') ||
       (a == '.' && b == '^' && c == '^') ||
       (a == '^' && b == '.' && c == '.') ||
       (a == '.' && b == '.' && c == '^') {
        '^'
    } else {
        '.'
    }
}

fn produce(row: &str) -> String {
    let mut ret = String::new();
    let len = row.len();
    let row: Vec<_> = row.chars().collect();

    ret.push(square('.', row[0], row[1]));

    for i in 1..(len - 1) {
        ret.push(square(row[i-1], row[i], row[i + 1]));
    }

    ret.push(square(row[len - 2], row[len-1], '.'));

    ret
}

fn main() {
    let mut file = File::open("input/day18.in").expect("Failed to open input");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read from input");
    let mut contents = contents.trim().to_owned();

    let mut part2 = 0;
    let mut part1 = 0;
    for i in 0..400000 {
        part2 += contents.chars().filter(|&x| x == '.').count();

        if i == 39 {
            part1 = part2;
        }
        contents = produce(&contents);
    }

    println!("1: {}", part1);
    println!("2: {}", part2);
}
