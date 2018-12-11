use std::env;
use std::process;

use std::collections::HashSet;

use common::load_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day1 <file>");
        process::exit(1);
    }

    let diffs: Vec<i32> = load_file(&args[1])
        .into_iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut freq = 0;

    let mut seen = HashSet::new();
    seen.insert(freq);

    let mut i = 0;
    loop {
        freq += diffs[i % diffs.len()];

        if seen.contains(&freq) {
            println!("{}", freq);
            break;
        } else {
            seen.insert(freq);
        }

        i += 1;
    }
}
