use std::collections::{HashMap, VecDeque};

const INPUT: u32 = 1350;

struct Maze {
    input: u32,
}

impl Maze {
    fn new(input: u32) -> Self {
        Maze{
            input: input,
        }
    }

    fn steps_to(&self, x: u32, y: u32) -> HashMap<(u32, u32), u32> {
        let mut steps = HashMap::new();
        let mut boundary = VecDeque::new();
        boundary.push_front((1, 1));
        steps.insert((1, 1), 0);

        while let Some(curr) = boundary.pop_back() {
            if curr == (x, y) {
                return steps
            }

            let (cx, cy) = curr;
            let ss = steps[&curr];

            if cx > 0 && !self.wall(cx-1, cy) && !steps.contains_key(&(cx-1, cy)) {
                steps.insert((cx-1, cy),  ss + 1);
                boundary.push_front((cx-1, cy));
            }
            if !self.wall(cx+1, cy) && !steps.contains_key(&(cx+1, cy)) {
                steps.insert((cx+1, cy),  ss + 1);
                boundary.push_front((cx+1, cy));
            }
            if cy > 0 && !self.wall(cx, cy-1) && !steps.contains_key(&(cx, cy-1)) {
                steps.insert((cx, cy-1),  ss + 1);
                boundary.push_front((cx, cy-1));
            }
            if !self.wall(cx, cy+1) && !steps.contains_key(&(cx, cy+1)) {
                steps.insert((cx, cy+1),  ss + 1);
                boundary.push_front((cx, cy+1));
            }
        }
        unreachable!();
    }

    fn wall(&self, x: u32, y: u32) -> bool {
        let test = x*x + 3*x + 2*x*y + y + y*y + self.input;
        test.count_ones() % 2 != 0
    }
}

fn main() {
    let maze = Maze::new(INPUT);
    let steps = maze.steps_to(31, 39);

    println!("1: {}", steps.get(&(31, 39)).unwrap());

    let part2 = steps.iter().filter_map(|(k, v)| if *v <= 50 { Some(k) } else { None }).count();
    println!("2: {}", part2);
}
