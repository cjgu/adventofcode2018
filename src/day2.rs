use std::fs::File;
use std::io::{BufReader,BufRead};

use std::env;
use std::process;

use std::collections::HashMap;

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

fn distance(a: &str, b: &str) -> i32 {
    a.chars().into_iter().zip(b.chars()).map(|(x,y)| { 
        if x == y {
            return 0;
        }
        1
    }).fold(0, |acc, x| acc + x)
}

fn common(a: &str, b: &str) -> String {
    a.chars().into_iter().zip(b.chars()).filter(|(x,y)| x == y).map(|(x,y)| x).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day2 <file>");
        process::exit(1);
    }

    let ids = load_file(&args[1]);
    let mut three_count = 0;
    let mut two_count = 0;

    for id in &ids {
        let mut counts = HashMap::new();
        for c in id.chars() {
            let counter = counts.entry(c).or_insert(0);
            *counter += 1;
        }

        // Find if there's a count 3
        let mut has_three = false;
        let mut has_two = false;
        for (_, val) in &counts {
            if val == &3 {
                has_three = true;
            }
            if val == &2 {
                has_two = true;
            }
        }

        if has_three {
            three_count +=1 ;
        }
        if has_two {
            two_count += 1;
        }

        
    }
    println!("{:?}", three_count * two_count);

    // part 2
    for i in 0..ids.len() {
        for j in i+1..ids.len() {
            let dist = distance(&ids[i], &ids[j]);

            if dist == 1 {
                println!("a: {} b: {} dist {} common {}", &ids[i], &ids[j], dist, common(&ids[i], &ids[j]));
            }
        }
    }

}
