use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const SEA_MONSTER: [&str; 3] = ["                  # ", "#    ##    ##    ###", " #  #  #  #  #  #   "];

#[derive(Debug, Copy, Clone, EnumIter)]
enum Direction {
    Normal,
    NormalFlipped,
    R90,
    R90Flipped,
    R180,
    R180Flipped,
    R270,
    R270Flipped,
}

struct Tile {
    tile: Vec<Vec<char>>,
}

impl Tile {
    fn new() -> Self {
        Tile { tile: Vec::new() }
    }

    fn add_row(&mut self, row: Vec<char>) {
        self.tile.push(row);
    }

    fn get(&self, x: usize, y: usize, d: Direction) -> char {
        let size = self.tile.len();

        match d {
            Direction::Normal => { self.tile[y][x] },
            Direction::NormalFlipped => { self.tile[size-y-1][x] },
            Direction::R90 => { self.tile[x][size-y-1] },
            Direction::R90Flipped => { self.tile[x][y] },
            Direction::R180 => { self.tile[size-y-1][size-x-1] },
            Direction::R180Flipped => { self.tile[y][size-x-1] },
            Direction::R270 => { self.tile[size-x-1][y] },
            Direction::R270Flipped => { self.tile[size-x-1][size-y-1] },
        }
    }

    fn size(&self) -> usize {
        self.tile.len()
    }

    fn has_pattern(&self, x: usize, y: usize, d: Direction, pattern: &Vec<Vec<char>>) -> bool {
        for dy in 0..pattern.len() {
            for dx in 0..pattern[dy].len() {
                if pattern[dy][dx] == '#' && self.get(x+dx, y+dy, d) != '#' {
                    return false;
                }
            }
        }
        true
    }
}

fn arrange(tiles: &HashMap<usize, Tile>, size: usize, tile_size: usize, arrangement: HashMap<(usize, usize), (usize, Direction)>) -> Option<HashMap<(usize, usize), (usize, Direction)>> {
    if arrangement.len() == size*size {
        return Some(arrangement);
    }

    let used: HashSet::<_> = arrangement.values().map(|(i, _)| i).collect();
    let index = arrangement.len();
    let y = index / size;
    let x = index % size;

    for (i, t) in tiles.iter() {
        if used.contains(i) {
            continue;
        }

        for d in Direction::iter() {
            if x > 0 {
                let left = (x-1, y);
                let (left_tile_index, left_tile_dir) = &arrangement[&left];
                let left_tile = &tiles[left_tile_index];
                let mut valid = true;
                for i in 0..tile_size {
                    if t.get(0, i, d) != left_tile.get(tile_size-1, i, *left_tile_dir) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
            }
            if y > 0 {
                let up = (x, y-1);
                let (up_tile_index, up_tile_dir) = &arrangement[&up];
                let up_tile = &tiles[up_tile_index];
                let mut valid = true;
                for i in 0..tile_size {
                    if t.get(i, 0, d) != up_tile.get(i, tile_size-1, *up_tile_dir) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    continue;
                }
            }
            let mut arr = arrangement.clone();
            arr.insert((x,y), (*i, d));
            if let Some(a) = arrange(tiles, size, tile_size, arr) {
                return Some(a);
            }
        }
    }
    None
}

fn main() {
    let file = File::open("input/day20.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut tiles = HashMap::new();
    let mut tile_index = 0;
    let mut tile = Tile::new();
    let mut tile_size = 0;

    for line in file.lines() {
        let line = line.unwrap();
        if line.starts_with("Tile") {
            let stripped = line.replace("Tile ", "").replace(":", "");
            tile_index = stripped.parse::<usize>().unwrap();
        } else if line == "" {
            tiles.insert(tile_index, tile);
            tile = Tile::new();
        } else {
            tile_size = line.chars().count();
            tile.add_row(line.chars().collect());
        }
    }
    tiles.insert(tile_index, tile);

    let mut size = 0;
    while size*size != tiles.len() {
        size += 1;
    }

    let arrangement = arrange(&tiles, size, tile_size, HashMap::new()).unwrap();

    let tl = arrangement.get(&(0, 0)).unwrap();
    let tr = arrangement.get(&(0, size-1)).unwrap();
    let bl = arrangement.get(&(size-1, 0)).unwrap();
    let br = arrangement.get(&(size-1, size-1)).unwrap();

    let part1 = tl.0 * tr.0 * bl.0 * br.0;

    println!("Part 1: {}", part1);

    let mut map = Tile::new();

    for tile_y in 0..size {
        for y in 1..(tile_size-1) {
            let mut row = Vec::new();
            for tile_x in 0..size {
                let (tile_index, dir) = arrangement.get(&(tile_x, tile_y)).unwrap();
                let tile = &tiles[tile_index];
                for x in 1..(tile_size-1) {
                    row.push(tile.get(x, y, *dir));
                }
            }
            map.add_row(row);
        }
    }

    let mut num_rough_waters = 0;
    for y in 0..map.size() {
        for x in 0..map.size() {
            if map.get(x, y, Direction::Normal) == '#' {
                num_rough_waters += 1;
            }
        }
    }

    let monster: Vec<Vec<char>> = SEA_MONSTER.iter().map(|s| s.chars().collect()).collect();

    let mut part2 = 0;
    for d in Direction::iter() {
        let mut sea_monster_tiles = HashSet::new();
        for y in 0..(map.size()-SEA_MONSTER.len()) {
            for x in 0..(map.size()-SEA_MONSTER[0].len()) {
                if map.has_pattern(x, y, d, &monster) {
                    for dy in 0..SEA_MONSTER.len() {
                        for dx in 0..SEA_MONSTER[dy].len() {
                            if monster[dy][dx] == '#' {
                                sea_monster_tiles.insert((x+dx, y+dy));
                            }
                        }
                    }
                }
            }
        }

        if sea_monster_tiles.len() > 0 {
            part2 = num_rough_waters - sea_monster_tiles.len();
        }
    }

    println!("Part 2: {}", part2);
}