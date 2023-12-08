use std::collections::HashMap;
use std::fs::read_to_string;
use num::integer::lcm;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    let lines = read_input();

    let route = &lines[0];

    let mut map = HashMap::new();

    for line in lines[2..].iter() {
        map.insert(
            line[..3].to_string(),
            (line[7..10].to_string(), line[12..15].to_string()),
        );
    }

    let zzz: String = "ZZZ".to_string();

    let nodes = map
        .keys()
        .filter(|n| n.chars().nth(2).unwrap() == 'A')
        .map(|map_node| {
            let mut count: usize = 0;

            let mut node = map_node.clone();
    'found: loop {
        for c in route.chars() {
            count += 1;

            node = match c {
                'L' => map[&node].0.clone(),
                'R' => map[&node].1.clone(),
                x => {
                    println! {"Unexpected route char {}", x};
                    zzz.clone()
                }
            };

            if node.chars().nth(2).unwrap() == 'Z' {
                break 'found;
            }
        }
    };

    count})
    .reduce(|acc, n| lcm(acc, n));

    println!("{:?}", nodes);
    0
}

fn main() {
    println!("Solution: {}", solve())
}
