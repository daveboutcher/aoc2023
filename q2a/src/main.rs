use regex::Regex;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> u32 {
    let re = Regex::new(r"([0-9]+) (blue|green|red)").unwrap();

    read_input()
        .iter()
        .enumerate()
        .map(|(gameno, line)| {
            if re
                .captures_iter(line)
                .map(|colormatch| {
                    let count = colormatch.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    match colormatch.get(2).unwrap().as_str() {
                        "red" => count <= 12,
                        "green" => count <= 13,
                        "blue" => count <= 14,
                        x => {
                            println!("Unexpected color {}", x);
                            false
                        }
                    }
                })
                .all(|x| x)
            {
                gameno as u32 + 1
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
