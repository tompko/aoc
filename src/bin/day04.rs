extern crate chrono;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use chrono::prelude::*;
use chrono::DateTime;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventType {
    NewShift(u32),
    Wake,
    Sleep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    time: DateTime<Utc>,
    typ: EventType,
}

fn main() {
    let file = File::open("input/day04.in").expect("Failed to open input");
    let file = BufReader::new(&file);

    let shift_re = Regex::new(r"\[([0-9: -]+)] Guard #(\d+) begins shift").unwrap();
    let wake_re = Regex::new(r"\[([0-9: -]+)] wakes up").unwrap();
    let sleep_re = Regex::new(r"\[([0-9: -]+)] falls asleep").unwrap();

    let mut events = Vec::new();

    for line in file.lines() {
        let line = line.unwrap();

        if let Some(scap) = shift_re.captures(&line) {
            let time_str = scap.get(1).unwrap().as_str();
            let time = Utc.datetime_from_str(time_str, "%Y-%m-%d %H:%M").unwrap();
            let guard = scap.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let typ = EventType::NewShift(guard);
            events.push(Event{ time: time, typ: typ });
        } else if let Some(scap) = wake_re.captures(&line) {
            let time_str = scap.get(1).unwrap().as_str();
            let time = Utc.datetime_from_str(time_str, "%Y-%m-%d %H:%M").unwrap();
            events.push(Event{ time: time, typ: EventType::Wake });
        } else if let Some(scap) = sleep_re.captures(&line) {
            let time_str = scap.get(1).unwrap().as_str();
            let time = Utc.datetime_from_str(time_str, "%Y-%m-%d %H:%M").unwrap();
            events.push(Event{ time: time, typ: EventType::Sleep });
        }
    }

    events.sort();

    let mut guard_times: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    let mut current_guard = 0;
    let mut asleep_minute = 0;

    for e in events.iter() {
        match e.typ {
            EventType::NewShift(guard) => {
                current_guard = guard;
                guard_times.entry(current_guard).or_insert(HashMap::new());
            }
            EventType::Wake => {
                let wake_minute = e.time.minute();
                let mut guard_map = guard_times.get_mut(&current_guard).unwrap();
                for i in asleep_minute..wake_minute {
                    guard_map.entry(i)
                        .and_modify(|s| *s += 1)
                        .or_insert(1);
                }
            }
            EventType::Sleep => {
                asleep_minute = e.time.minute();
            }
        }
    }

    let mut best_guard = 0;
    let mut best_asleep = 0;
    let mut best_minute = 0;

    let mut worst_minute = 0;
    let mut worst_asleep = 0;
    let mut worst_guard = 0;

    for (guard, times) in guard_times.iter() {
        let mut bm = 0;
        let mut bms = 0;
        let mut total = 0;

        for (minute, ts) in times.iter() {
            total += ts;
            if *ts > bms {
                bm = *minute;
                bms = *ts;
            }

            if *ts > worst_asleep {
                worst_minute = *minute;
                worst_asleep = *ts;
                worst_guard = *guard;
            }
        }

        if total > best_asleep {
            best_guard = *guard;
            best_asleep = total;
            best_minute = bm;
        }
    }

    println!("part 1: {}", best_guard * best_minute);
    println!("part 1: {}", worst_guard * worst_minute);
}
