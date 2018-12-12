use std::env;
use std::process;

use common::load_file;

use multiarray::Array2D;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn dist(&self, other: Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day6 <file>");
        process::exit(1);
    }

    let rows = load_file(&args[1]);

    let coords: Vec<Coord> = rows
        .iter()
        .map(|r| r.split(", ").collect::<Vec<&str>>())
        .map(|c| Coord {
            x: c[0].parse::<i32>().unwrap(),
            y: c[1].parse::<i32>().unwrap(),
        })
        .collect();

    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();

    println!("Max: x={} y={}", max_x, max_y);

    let mut board = Array2D::new([max_x as usize, max_y as usize], 0i32);

    for x in 0..max_x {
        for y in 0..max_y {
            let mut distances: Vec<(usize, i32)> = coords
                .iter()
                .enumerate()
                .map(|(i, c)| (i, c.dist(Coord { x: x, y: y })))
                .collect();

            distances.sort_by_key(|&(_, d)| d);

            if distances[0].1 == distances[1].1 {
                board[[x as usize, y as usize]] = -1;
            } else {
                board[[x as usize, y as usize]] = distances[0].0 as i32;
            }
        }
    }

    let mut sizes = HashMap::new();
    for x in 0..max_x {
        for y in 0..max_y {
            let id = board[[x as usize, y as usize]];
            if id == -1 {
                continue;
            }

            let size_counter = sizes.entry(id).or_insert(0);
            *size_counter += 1u32;
        }
    }

    let mut infinite_ids = HashSet::new();
    for x in 0..max_x {
        infinite_ids.insert(board[[x as usize, 0]]);
        infinite_ids.insert(board[[x as usize, (max_y - 1) as usize]]);
    }
    for y in 0..max_y {
        infinite_ids.insert(board[[0, y as usize]]);
        infinite_ids.insert(board[[(max_x - 1) as usize, y as usize]]);
    }
    // find all ids that face the edge of the board
    // exclude them and do a max

    let finite_sizes = sizes.iter().filter(|(id, _)| !infinite_ids.contains(id));

    let max_area = finite_sizes.max_by_key(|&(_, size)| size).unwrap();

    println!("Part 1: Max area by id {} area {}", max_area.0, max_area.1);

    let mut area_size = 0;

    for x in 0..max_x {
        for y in 0..max_y {
            let sum_dist: i32 = coords
                .iter()
                .enumerate()
                .map(|(_i, c)| c.dist(Coord { x: x, y: y }))
                .sum();
            if sum_dist < 10000 {
                area_size += 1;
            }
        }
    }

    println!("Part 2: Area is  {}", area_size);
}
