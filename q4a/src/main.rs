use std::fs::read_to_string;
use regex::Regex;
use std::cmp;
use std::collections::HashSet;


fn read_input() -> Vec<String> {
    read_to_string("input.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn solve() -> u32 {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let lines = read_input();

    let mut total: u32 = 0;

    let base:u32 = 2;

    for (lineno, line) in lines.iter().enumerate() {
        let mut winners = HashSet::new();

        let colonpos = line.find(":").unwrap();
        let barpos = line.find("|").unwrap();

        let winning = &line[(colonpos+1)..barpos];
        let have = &line[(barpos+1)..];

        for m in re.find_iter(&winning) {
            winners.insert(m.as_str().parse::<u32>().unwrap());
        }
        
        let mut c:u32 = 0;
        for m in re.find_iter(&have) {
            if winners.contains(&m.as_str().parse::<u32>().unwrap()) {
                c += 1;
            }
        }

        if c > 0 {
            total += base.pow(c-1);
        }

    }

    total

 }


fn main() {
    println!("Solution: {}", solve())
}
