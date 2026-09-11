use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn add_mapping(name_map: &mut HashMap<String, u16>, name: &str, id: &mut u16) -> u16 {
    if let Some(i) = name_map.get(name) {
        *i
    } else {
        let i = *id;
        name_map.insert(name.to_string(), i);
        *id += 1;
        i
    }
}

fn solve() -> usize {
    let mut name_map: HashMap<String, u16> = HashMap::new();

    let mut id = 0;

    let cons = 
        read_input()
            .iter()
            .map(|l| l.split(':').collect::<Vec<_>>())
            .map(|v| (
                add_mapping(&mut name_map, v[0], &mut id),
                v[1][1..]
                    .split(' ')
                    .map(|s| add_mapping(&mut name_map, s, &mut id))
                    .collect::<Vec<u16>>()
            ))
            .collect::<Vec<_>>();

    let mut connections = vec![vec![usize::MAX; id as usize]; id as usize];

    for (n, v) in cons {
        for n1 in v {
            connections[n as usize][n1 as usize] = 1;
            connections[n1 as usize][n as usize] = 1;
        }
    }


    0
}

fn main() {
    println!("Solution: {}", solve())
}
