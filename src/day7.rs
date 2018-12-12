use common::load_file;
use regex::Regex;
use std::collections::{HashMap, HashSet};

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day7 <file>");
        process::exit(1);
    }

    let rows = load_file(&args[1]);

    let re = Regex::new(r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.").unwrap();

    let mut steps = HashSet::new();

    let mut deps = HashMap::new();

    for x in rows.iter() {
        let cap = re.captures(x).unwrap();

        let step = cap.get(2).unwrap().as_str();
        let dep = cap.get(1).unwrap().as_str();

        steps.insert(step);
        steps.insert(dep);

        let dependencies = deps.entry(step).or_insert(HashSet::new());

        dependencies.insert(dep);
    }

    for (k, v) in deps.iter() {
        println!("{} => {:?}", k, v);
    }

    let mut completed = HashSet::new();

    let mut ordered_steps: Vec<_> = steps.iter().map(|x| *x).collect();

    ordered_steps.sort();

    println!("Ordered steps {:?}", ordered_steps);

    let mut sequence: String = "".to_string();

    loop {
        if completed.len() == ordered_steps.len() {
            break;
        }

        let mut ready_step = "";
        for step in ordered_steps.iter() {
            if completed.contains(step) {
                continue;
            }
            match deps.get(step) {
                Some(d) => {
                    if d.len() == 0 {
                        ready_step = step;
                        break;
                    }
                }
                None => {
                    ready_step = step;
                    break;
                }
            }
        }

        if ready_step == "" {
            panic!("No ready step!");
        }
        println!("Step {} ready", ready_step);
        completed.insert(ready_step);
        sequence.push_str(ready_step);

        // remove it from all that deps on it
        for s in steps.iter() {
            deps.entry(s).and_modify(|e| {
                e.remove(ready_step);
            });
        }
    }

    println!("Part 1: sequence:  {}", sequence);
}
