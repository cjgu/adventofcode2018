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
        println!("day3 <file>");
        process::exit(1);
    }

    let rows = load_file(&args[1]);

    // 1000 x 1000 
    let mut state = [[0i32; 1000]; 1000];

    let mut collisions = HashSet::new();

    for row in rows.iter() {
        // #1 @ 1,3: 4x4
        let parts: Vec<&str> = row.split(' ').collect();

        let id = parts[0].replace("#","").parse::<i32>().unwrap();
        let coord: Vec<&str> = parts[2].split(',').collect();
        let size: Vec<&str> = parts[3].split('x').collect();

        let x = coord[0].parse::<u32>().unwrap();
        let y = coord[1].replace(':', "").parse::<u32>().unwrap();

        let w = size[0].parse::<u32>().unwrap();
        let h = size[1].parse::<u32>().unwrap();


        for xx in x..x+w {
            for yy in y..y+h {
                if state[xx as usize][yy as usize] > 0 {
                    // already occupied
                    collisions.insert(state[xx as usize][yy as usize]);
                    collisions.insert(id);

                    state[xx as usize][yy as usize] = -1;
                }
                else if state[xx as usize][yy as usize] < 0 {  
                    // already occupied
                    collisions.insert(id);
                }
                else if state[xx as usize][yy as usize] == 0 {
                    state[xx as usize][yy as usize] = id;
                }
            }
        }
    }

    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if state[x][y] < 0 {
                count += 1;
            }
        }
    }
    println!("overlapping inches {:?}", count);

    for row in rows.iter() {
         let parts: Vec<&str> = row.split(' ').collect();

        let id = parts[0].replace("#","").parse::<i32>().unwrap();

        if !collisions.contains(&id) {
            println!("no collision: {}", id);
        }
    }

}