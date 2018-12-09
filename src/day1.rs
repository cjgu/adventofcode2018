use std::fs::File;
use std::io::{BufReader,BufRead};

use std::env;
use std::process;

use std::collections::HashSet;

fn load_file(file_path: &str) -> Vec<String> {
    let mut content = vec![]; 

    let f = File::open(file_path).expect("Unable to open file");

    let br = BufReader::new(f);

    for line in br.lines() {
        let l = line.unwrap();
        content.push(l);
    }

    content
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day1 <file>");
        process::exit(1);
    }

    let diffs: Vec<i32> = load_file(&args[1]).into_iter()
        .map(|x| { x.parse::<i32>().unwrap() }).collect();

    let mut freq = 0;

    let mut seen = HashSet::new();
    seen.insert(freq);

    let mut i = 0;
    loop {
        freq += diffs[i % diffs.len()];

        if seen.contains(&freq) {
            println!("{}", freq);
            break;
        }  
        else {
            seen.insert(freq);
        }

        i += 1;
    }
}
