use std::collections::HashMap;
use std::fs::read_to_string;

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

    let mut count: usize = 0;
    let mut node = "AAA";
    'found: loop {
        for c in route.chars() {
            count += 1;

            node = match c {
                'L' => &map[node].0,
                'R' => &map[node].1,
                x => {
                    println! {"Unexpected route char {}", x};
                    "ZZZ"
                }
            };

            if node == "ZZZ" {
                break 'found;
            }
        }
    }

    count
}

fn main() {
    println!("Solution: {}", solve())
}
