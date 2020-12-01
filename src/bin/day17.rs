extern crate regex;

use std::cmp::{min, max};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let file = File::open("input/day17.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let line_re = Regex::new(r"(\w)=(\d+), \w=(\d+)\.\.(\d+)").unwrap();

    let mut grid = vec![vec!['.';2000];2000];
    let mut minx = usize::max_value();
    let mut maxx = 0;
    let mut miny = usize::max_value();
    let mut maxy = 0;

    for line in file.lines() {
        let line = line.unwrap();
        let caps = line_re.captures(&line).unwrap();

        let dir = caps.get(1).unwrap().as_str();
        let n1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let n2 = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let n3 = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();

        if dir == "x" {
            minx = min(minx, n1);
            maxx = max(maxx, n1);

            for y in n2..=n3 {
                miny = min(miny, y);
                maxy = max(maxy, y);
                grid[y][n1] = '#';
            }
        } else if dir == "y" {
            miny = min(miny, n1);
            maxy = max(maxy, n1);

            for x in n2..=n3 {
                minx = min(minx, x);
                maxx = max(maxx, x);
                grid[n1][x] = '#';
            }
        } else {
            panic!("Unrecognised direction {}", dir);
        }
    }

    let mut needs_reflow = true;
    while needs_reflow {
        needs_reflow = flow(0, 500, &mut grid);
    }

    let part1 = grid.iter()
        .skip(miny)
        .take(maxy-miny+1)
        .map(|row| row.iter().filter(|&c| *c == '~' || *c == '|').count())
        .sum::<usize>();
    println!("part 1: {}", part1);

    let part2 = grid.iter()
        .skip(miny)
        .take(maxy-miny+1)
        .map(|row| row.iter().filter(|&c| *c == '~').count())
        .sum::<usize>();
    println!("part 2: {}", part2);
}

fn flow(sy: usize, sx: usize, grid: &mut Vec<Vec<char>>) -> bool {
    for y in sy..grid.len() {
        if grid[y][sx] == '|' || grid[y][sx] == '.' {
            grid[y][sx] = '|';
            continue;
        }

        let flow_start = y - 1;
        let mut needs_reflow = false;
        let mut hit_left = false;
        let mut hit_right = false;

        for x in (0..sx).rev() {
            if grid[flow_start][x] == '#' {
                hit_left = true;
                break;
            }
            grid[flow_start][x] = '|';
            if grid[y][x] != '#' && grid[y][x] != '~' {
                needs_reflow |= flow(flow_start, x, grid);
                break;
            }
        }
        for x in sx..2000 {
            if grid[flow_start][x] == '#' {
                hit_right = true;
                break;
            }
            grid[flow_start][x] = '|';
            if grid[y][x] != '#' && grid[y][x] != '~' {
                needs_reflow |= flow(flow_start, x, grid);
                break;
            }
        }

        if hit_left && hit_right {
            needs_reflow = true;

            for x in (0..sx).rev() {
                if grid[flow_start][x] != '|' { break; }
                grid[flow_start][x] = '~';
            }
            for x in sx..2000 {
                if grid[flow_start][x] != '|' { break; }
                grid[flow_start][x] = '~';
            }
        }

        return needs_reflow;
    }

    false
}
