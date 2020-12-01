use std::collections::HashMap;

const SERIAL: i32 = 2694;

fn main() {
    let mut grid = vec![vec![0;300];300];

    for y in 0..300 {
        for x in 0..300 {
            grid[y][x] = power_level(x as i32, y as i32);
        }
    }

    let mut best_power_level = 9 * -5;
    let mut best_coord = (0, 0);

    let mut powers = HashMap::new();
    let mut max_power = 300 * 300 * -5;
    let mut best_triple = (0,0,0);

    for y in 0..300 {
        for x in 0..300 {
            let p = grid[y][x];
            if p > max_power {
                max_power = p;
                best_triple = (x, y, 1);
            }
            powers.insert((x, y, 1), p);
        }
    }


    for i in 2..300 {
        for y in 0..(300-i) {
            for x in 0..(300-i) {
                let mut p = powers.get(&(x, y, i-1)).unwrap() + grid[y+i-1][x+i-1];
                for j in 0..(i-1) {
                    p += grid[y+j][x+i-1];
                    p += grid[y+i-1][x+j];
                }
                if p > max_power {
                    max_power = p;
                    best_triple = (x, y, i);
                }
                if i == 3 && p > best_power_level {
                    best_power_level = p;
                    best_coord = (x, y);
                }
                powers.insert((x, y, i), p);
            }
        }
    }

    println!("part 1: {},{}", best_coord.0, best_coord.1);
    println!("part 2: {},{},{}", best_triple.0, best_triple.1, best_triple.2);
}

fn power_level(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = rack_id * y;
    let power_level = power_level + SERIAL;
    let power_level = power_level * rack_id;
    let power_level = (power_level / 100) % 10;
    power_level - 5
}
