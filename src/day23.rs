use common::load_file;
use regex::Regex;

use std::env;
use std::process;

#[derive(Debug)]
struct Bot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Bot {
    fn distance(&self, other: &Bot) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day7 <file>");
        process::exit(1);
    }

    let rows = load_file(&args[1]);

    let re = Regex::new(r"pos=<(-?[0-9]+),(-?[0-9]+),(-?[0-9]+)>, r=([0-9]+)").unwrap();

    let mut bots = vec![];

    for row in rows.iter() {
        let cap = re.captures(row).unwrap();

        let x = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let y = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let z = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let r = cap.get(4).unwrap().as_str().parse::<i64>().unwrap();

        bots.push(Bot { x, y, z, r });
    }

    let max = bots.iter().max_by_key(|&row| row.r).unwrap();

    let in_range: Vec<&Bot> = bots
        .iter()
        .filter(|&bot| bot.distance(&max) <= max.r)
        .collect();

    println!("Part 1: In range {:?}", in_range.len());
}
