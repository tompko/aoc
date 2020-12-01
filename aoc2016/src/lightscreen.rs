use regex::Regex;

pub struct Screen {
    screen: [[bool; 6]; 50],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            screen: [[false; 6]; 50],
        }
    }

    pub fn execute(&mut self, instr: &str) {
        lazy_static!(
            static ref REC: Regex = Regex::new(r"rect (\d+)x(\d+)").unwrap();
            static ref ROW: Regex = Regex::new(r"rotate row y=(\d+) by (\d+)").unwrap();
            static ref COL: Regex = Regex::new(r"rotate column x=(\d+) by (\d+)").unwrap();
        );

        if let Some(caps) = REC.captures(instr) {
            let x = caps.at(1).unwrap().parse().unwrap();
            let y = caps.at(2).unwrap().parse().unwrap();
            self.rect(x, y);
        } else if let Some(caps) = ROW.captures(instr) {
            let y = caps.at(1).unwrap().parse().unwrap();
            let b = caps.at(2).unwrap().parse().unwrap();
            self.rot_row(y, b);
        } else if let Some(caps) = COL.captures(instr) {
            let x = caps.at(1).unwrap().parse().unwrap();
            let b = caps.at(2).unwrap().parse().unwrap();
            self.rot_col(x, b);
        }
    }

    pub fn count_on(&self) -> u32 {
        self.screen.iter().map(
            |r| r.iter().map(
                |b| if *b { 1 } else { 0 }).sum::<u32>()
            ).sum()
    }

    pub fn print(&self) {
        for y in 0..6 {
            for x in 0..50 {
                if self.screen[x][y] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }

    fn rect(&mut self, x: usize, y: usize) {
        for dy in 0..y {
            for dx in 0..x {
                self.screen[dx][dy] = true
            }
        }
    }

    fn rot_row(&mut self, y: usize, b: usize) {
        let mut row = [false; 50];

        for (x, c) in row.iter_mut().enumerate() {
            *c = self.screen[x][y];
        }

        for x in 0..50 {
            self.screen[x][y] = row[(x + 50 - b) % 50];
        }
    }

    fn rot_col(&mut self, x:usize, b: usize) {
        let mut col = [false; 6];

        for (y, c) in col.iter_mut().enumerate() {
            *c = self.screen[x][y];
        }

        for y in 0..6 {
            self.screen[x][y] = col[(y + 6 - b) % 6];
        }
    }
}

