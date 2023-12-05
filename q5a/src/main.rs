use std::fs::read_to_string;
use regex::Regex;
use std::cmp;

fn read_input() -> Vec<String> {
    read_to_string("input.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn solve() -> usize {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let lines = read_input();

    let mut loc = std::usize::MAX;

    let mut maps = Vec::new();
    let mut map = Vec::new();

    for line in &lines[2..] {
        if line.contains(":") {
            println!("Processing {}", line);
            continue;
        } else if line.len() == 0 {
            maps.push(map);
            map =Vec::new();
        } else {
            let vals:Vec::<usize> = re.captures_iter(&line)
            .map(|c| c.get(1).unwrap().as_str().parse::<usize>().unwrap())
            .collect();

            let dstart = vals[0];
            let sstart = vals[1];
            let count = vals[2];

            map.push((sstart, dstart, count));
       }
    }

    println!("starting seeds...");

    for seed in re.captures_iter(&lines[0]) {
        let mut cur = seed.get(1).unwrap().as_str().parse::<usize>().unwrap();

        for m in &maps {
            for conv in m {
                if cur >= conv.0 && cur < (conv.0 + conv.2) {
                    cur = conv.1 + (cur - conv.0);
                    break;
                }
            }
        }

        loc = cmp::min(cur, loc);
    }

    loc

 }


fn main() {
    println!("Solution: {}", solve())
}
