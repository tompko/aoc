use std::collections::HashMap;

const INPUT: i32 = 325489;

fn main() {
    let part1 = solve1();
    let part2 = solve2();

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}

fn solve1() -> i32 {
    let mut n = 1;
    while n*n < INPUT {
        n += 2;
    }
    let mut corner = n*n;
    while INPUT < corner {
        corner -= n - 1;
    }
    let midpoint = corner + (n / 2);

    (n/2)  + (INPUT - midpoint).abs()
}

fn solve2() -> i32 {
    let directions = [(1,0), (0, 1), (-1, 0), (0, -1)];
    let mut directions = directions.iter().cycle();

    let mut grid = HashMap::new();
    let mut coord = (0, 0);
    let mut n = 1;

    grid.insert(coord, 1);

    loop {
        let dir = *directions.next().unwrap();
        for _ in 0..n {
            coord = (coord.0 + dir.0, coord.1 + dir.1);
            let r = fill(&mut grid, &coord);
            if r > INPUT {
                return r;
            }
        }
        let dir = *directions.next().unwrap();
        for _ in 0..n {
            coord = (coord.0 + dir.0, coord.1 + dir.1);
            let r = fill(&mut grid, &coord);
            if r > INPUT {
                return r;
            }
        }
        n += 1;
    }
}

fn fill(grid: &mut HashMap<(i32, i32), i32>, coord: &(i32, i32)) -> i32 {
    let neighbours = [(1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1)];

    let entry = neighbours.iter().
        map(|&(x, y)| (coord.0 + x, coord.1 + y)).
        map(|c| *grid.get(&c).unwrap_or(&0)).
        sum();

    grid.insert(*coord, entry);

    entry
}
