extern crate regex;

use std::env;
use std::process;

use common::load_file;
use regex::Regex;
use std::collections::HashMap;

enum Action {
    FallAsleep,
    WakesUp,
}

struct Log {
    guard_id: u32,
    minute: u8,
    action: Action,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day3 <file>");
        process::exit(1);
    }

    let mut rows = load_file(&args[1]);

    // sort lexigraphically
    rows.sort();

    // [1518-11-18 00:02] Guard #1049 begins shift
    let re = Regex::new(r"^\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] (.+)").unwrap();
    let shift_re = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();

    let mut curr_guard_id = 0;

    let mut log = vec![];

    for row in rows.iter() {
        let caps = re.captures(row).unwrap();
        let _hh = caps.get(1).unwrap().as_str();
        let mm = caps.get(2).unwrap().as_str();
        let action = caps.get(3).unwrap().as_str();

        if action == "wakes up" {
            log.push(Log {
                guard_id: curr_guard_id,
                minute: mm.parse::<u8>().unwrap(),
                action: Action::WakesUp,
            });
        } else if action == "falls asleep" {
            log.push(Log {
                guard_id: curr_guard_id,
                minute: mm.parse::<u8>().unwrap(),
                action: Action::FallAsleep,
            });
        } else {
            let shift_caps = shift_re.captures(action).unwrap();
            let guard_id = shift_caps.get(1).unwrap().as_str().parse::<u32>().unwrap();

            curr_guard_id = guard_id;
        }
    }

    let mut guard_sleeps = HashMap::new();
    let mut curr_falls_asleep_at = 0;
    for l in log.iter() {
        match l.action {
            Action::FallAsleep => curr_falls_asleep_at = l.minute,
            Action::WakesUp => {
                let asleep_for = l.minute - curr_falls_asleep_at;

                let sleep_counter = guard_sleeps.entry(l.guard_id).or_insert(0);
                *sleep_counter += asleep_for as u32;
            }
        }
    }

    let max = guard_sleeps.iter().max_by_key(|&(_, item)| item).unwrap();
    println!("guard: {} asleep for {}", max.0, max.1);

    // let mut minute_sleep = [0u32; 60];
    let mut guard_histogram = HashMap::new();

    for l in log.iter() {
        match l.action {
            Action::FallAsleep => curr_falls_asleep_at = l.minute,
            Action::WakesUp => {
                let histogram = guard_histogram.entry(l.guard_id).or_insert([0u32; 60]);

                for m in curr_falls_asleep_at..l.minute {
                    histogram[m as usize] += 1;
                }
            }
        }
    }
    let max_minute = guard_histogram
        .get(max.0)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|&(_, item)| item)
        .unwrap();

    println!(
        "guard  asleep for {} on minute {}",
        max_minute.1, max_minute.0
    );

    println!("answer part 1: {}", *max.0 as usize * max_minute.0);

    let mut max_id = 0;
    let mut max_minute = 0;
    let mut max_minutes = 0;
    for (guard_id, histogram) in guard_histogram.iter() {
        let mm = histogram
            .iter()
            .enumerate()
            .max_by_key(|&(_, item)| item)
            .unwrap();
        if *mm.1 > max_minutes {
            max_minute = mm.0 as u32;
            max_minutes = *mm.1;

            max_id = *guard_id;

            println!(
                "New leader, guard id: {}; max_minute {} max_minutes {}",
                max_id, max_minute, max_minutes
            );
        }
    }

    println!("guard id {} slept most on minute {}", max_id, max_minute);
    println!("Answer part 2: {}", max_id * max_minute);
}
