extern crate fancy_regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::max;
use std::str::FromStr;
use fancy_regex::Regex;

enum State {
    Flying(u32),
    Resting(u32),
}

struct Reindeer {
    speed: u32,
    flight_time: u32,
    rest_time: u32,
}

struct Racer {
    state: State,
    distance: u32,
    points: u32,
}

impl FromStr for Reindeer {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.")?;
        let capture = re.captures(s)?.unwrap();

        let speed = capture.get(1).unwrap().as_str().parse()?;
        let flight_time = capture.get(2).unwrap().as_str().parse()?;
        let rest_time = capture.get(3).unwrap().as_str().parse()?;

        let r = Reindeer{
            speed: speed,
            flight_time: flight_time,
            rest_time: rest_time,
        };

        Ok(r)
    }
}

fn simulate(reindeers: &[Reindeer], limit: usize) -> (u32, u32) {
    let mut racers: Vec<Racer> = reindeers.iter().map(|r| Racer {state: State::Flying(r.flight_time), distance: 0, points: 0}).collect();

    for _ in 0..limit {
        let mut m = 0;
        for (i, r) in racers.iter_mut().enumerate() {
            match r.state {
                State::Flying(f) => {
                    r.distance += reindeers[i].speed;
                    r.state = if f == 1 { State::Resting(reindeers[i].rest_time) } else { State::Flying(f-1) };
                }
                State::Resting(x) => {
                    r.state = if x == 1 { State::Flying(reindeers[i].flight_time) } else { State::Resting(x-1) };
                }
            }
            m = max(m, r.distance);
        }
        for r in racers.iter_mut() {
            if r.distance == m {
                r.points += 1;
            }
        }
    }

    let mut max_distance = 0;
    let mut max_points = 0;
    for r in racers.iter() {
        max_distance = max(max_distance, r.distance);
        max_points = max(max_points, r.points);
    }
    
    (max_distance, max_points)
}


fn main() {
    let f = File::open("inputs/day14.in").unwrap();
    let file = BufReader::new(&f);

    let reindeers: Vec<Reindeer> = file.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    let (part1, part2) = simulate(&reindeers, 2503);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
