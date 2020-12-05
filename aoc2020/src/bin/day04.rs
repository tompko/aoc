extern crate fancy_regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use fancy_regex::Regex;

#[derive(Default, Debug, Clone)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn has_required(&self) -> bool {
        self.byr.is_some() && 
        self.iyr.is_some() &&
        self.eyr.is_some() &&
        self.hgt.is_some() &&
        self.hcl.is_some() &&
        self.ecl.is_some() &&
        self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if !Passport::is_bounded_num(&self.byr.as_ref().unwrap(), 4, 1920, 2002) {
            return false;
        }
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if !Passport::is_bounded_num(&self.iyr.as_ref().unwrap(), 4, 2010, 2020) {
            return false;
        }
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if !Passport::is_bounded_num(&self.eyr.as_ref().unwrap(), 4, 2020, 2030) {
            return false;
        }
        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.
        if !Passport::is_height(&self.hgt.as_ref().unwrap()) {
            return false;
        }
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if !Passport::is_hair_color(&self.hcl.as_ref().unwrap()) {
            return false;
        }
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !Passport::is_eye_color(&self.ecl.as_ref().unwrap()) {
            return false;
        }
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if !Passport::is_bounded_num(&self.pid.as_ref().unwrap(), 9, 1, 999999999) {
            return false;
        }

        true
    }

    fn is_bounded_num(num: &str, digits: usize, min: u64, max: u64) -> bool {
        if num.len() != digits {
            return false;
        }
        if let Ok(n) = num.parse::<u64>() {
            if min > n || max < n {
                return false;
            }
        } else {
            return false;
        }
        true
    }

    fn is_height(hgt: &str) -> bool {
        if hgt.ends_with("cm") {
            let hgt = hgt.strip_suffix("cm").unwrap();
            Passport::is_bounded_num(hgt, 3, 150, 193)
        } else if hgt.ends_with("in") {
            let hgt = hgt.strip_suffix("in").unwrap();
            Passport::is_bounded_num(hgt, 2, 59, 76)
        } else {
            false
        }
    }

    fn is_hair_color(hcl: &str) -> bool {
        let re = Regex::new("#[0-9a-f]{6}").unwrap();
        re.is_match(hcl).unwrap()
    }

    fn is_eye_color(ecl: &str) -> bool {
        for &m in &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] {
            if ecl == m {
                return true;
            }
        }
        false
    }
}

fn main() {
    let file = File::open("input/day04.txt").expect("Failed to open input");
    let file = BufReader::new(&file);

    let mut passports: Vec<Passport> = Vec::new();
    let mut p = Passport::default();
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line == "" {
            passports.push(p.clone());
            p = Passport::default();
        } else {
            for pair in line.split(" ") {
                let mut pair = pair.split(":");
                let k = pair.next().unwrap();
                let v = pair.next().unwrap();
                match k {
                    "byr" => p.byr = Some(v.to_string()),
                    "iyr" => p.iyr = Some(v.to_string()),
                    "eyr" => p.eyr = Some(v.to_string()),
                    "hgt" => p.hgt = Some(v.to_string()),
                    "hcl" => p.hcl = Some(v.to_string()),
                    "ecl" => p.ecl = Some(v.to_string()),
                    "pid" => p.pid = Some(v.to_string()),
                    "cid" => p.cid = Some(v.to_string()),
                    _ => unreachable!(),
                }
            }
        }
    }
    passports.push(p);

    let part1 = passports.iter().filter(|p| p.has_required()).count();
    let part2 = passports.iter().filter(|p| p.has_required() && p.is_valid()).count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}