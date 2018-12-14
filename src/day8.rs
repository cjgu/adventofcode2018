use std::env;
use std::process;

use common::load_file;

#[derive(Debug)]
struct Header {
    num_children: usize,
    num_metadata: usize,
}

#[derive(Debug)]
struct Node {
    header: Header,
    children: Vec<Box<Node>>,
    metadata: Vec<i32>,
}

fn parse(data: &Vec<i32>, pos: usize) -> (Node, usize) {
    let header = Header {
        num_children: data[pos] as usize,
        num_metadata: data[pos + 1] as usize,
    };
    let mut p = pos + 2;
    let mut children = vec![];
    for _ in 0..header.num_children {
        let (child, next_pos) = parse(data, p);

        children.push(Box::new(child));
        p = next_pos;
    }

    let mut meta = vec![];
    for _ in 0..header.num_metadata {
        meta.push(data[p]);
        p += 1;
    }

    let node = Node {
        header: header,
        children: children,
        metadata: meta,
    };

    (node, p)
}

fn sum_metadata(node: &Node) -> i32 {
    let mut sum = 0;

    for c in node.children.iter() {
        sum += sum_metadata(&c);
    }

    sum += node.metadata.iter().sum::<i32>();

    sum
}

fn node_value(node: &Node) -> i32 {
    if node.children.len() == 0 {
        node.metadata.iter().sum::<i32>()
    } else {
        let mut value = 0;
        for m in node.metadata.iter() {
            let child_id = (*m as usize) - 1;
            if child_id < node.children.len() {
                value += node_value(&node.children[child_id]);
            }
        }
        value
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("day8 <file>");
        process::exit(1);
    }

    let rows = load_file(&args[1]);
    let data: Vec<i32> = rows[0]
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (head, _) = parse(&data, 0);

    let sum = sum_metadata(&head);

    println!("Part 1: Metadata sum {}", sum);

    let value = node_value(&head);

    println!("Part 2: Node value: {}", value);
}
