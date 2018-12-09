use std::fs::File;
use std::io::prelude::*;

use std::env;
use std::process;
use std::str;

use std::collections::HashMap;

fn load_file(file_path: &str) -> String {
    let mut contents = String::new();
    let mut f = File::open(file_path).expect("Unable to open file");

    f.read_to_string(&mut contents).expect("cant read file");

    contents
}

fn react(bytes: &mut Vec<u8>) -> usize {
    let mut left_pos = 0;
    let mut right_pos = 1;
    loop {
        if right_pos >= bytes.len() {
            break;
        }

        if (bytes[left_pos] as i32 - bytes[right_pos] as i32).abs() == 32i32 {
            bytes[left_pos] = 42;
            bytes[right_pos] = 42;

            if left_pos > 0 {
                left_pos -= 1;
                while bytes[left_pos] == 42 && left_pos > 0 {
                    left_pos -= 1;
                }
            }

            if right_pos < bytes.len() - 1 {
                right_pos += 1;
            }
        } else {
            left_pos += 1;
            while bytes[left_pos] == 42 && left_pos < bytes.len() - 1 {
                left_pos += 1;
            }
            right_pos += 1;
        }
    }

    let reacted: Vec<u8> = bytes.iter().filter(|x| *x != &42u8).map(|x| *x).collect();

    reacted.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day5 <file>");
        process::exit(1);
    }

    let row = load_file(&args[1]);

    let bytes = row.as_bytes().to_vec();

    let mut input = bytes.clone();
    let reacted_len = react(&mut input);
    println!("Part A answer: reacted len is {}", reacted_len);

    let mut map = HashMap::new();
    for s in 65..91 {
        let mut input: Vec<u8> = bytes
            .iter()
            .filter(|x| *x != &s && *x != &(s + 32))
            .map(|x| *x)
            .collect();
        let reacted_len = react(&mut input);
        map.insert(s as char, reacted_len);
    }

    let min = map.iter().min_by_key(|&(_, item)| item).unwrap();
    println!("Part B answer: shortest fixed polymer is {}", min.1);
}
