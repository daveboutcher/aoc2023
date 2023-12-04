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


fn solve() -> usize {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let lines = read_input();

    let mut total: usize = 0;

    let base:usize = 2;

    let mut copies = vec![1; lines.len()];
    let mut cards: usize = 0;
    for (lineno, line) in lines.iter().enumerate() {
        let mut winners = HashSet::new();

        let colonpos = line.find(":").unwrap();
        let barpos = line.find("|").unwrap();

        let winning = &line[(colonpos+1)..barpos];
        let have = &line[(barpos+1)..];

        for m in re.find_iter(&winning) {
            winners.insert(m.as_str().parse::<u32>().unwrap());
        }
        
        let mut c:usize = 0;
        for m in re.find_iter(&have) {
            if winners.contains(&m.as_str().parse::<u32>().unwrap()) {
                c += 1;
            }
        }

        let points:usize;

        if c > 0 {
            points = base.pow((c-1) as u32);
        } else {
            points = 0;
        }

            for copier in lineno+1..lineno+1+c {
                copies[copier] += copies[lineno];
            }

        println!("Card {} copies {}", lineno+1, copies[lineno]);

        total += points * copies[lineno];
        cards += copies[lineno];

    }

    println!("Cards {}", cards);

    total

 }


fn main() {
    println!("Solution: {}", solve())
}
