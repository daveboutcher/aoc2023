use regex::Regex;
use std::cmp;
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
        .map(|line| {
            re.captures_iter(line)
                .fold([0u32, 0u32, 0u32], |mut acc, colormatch| {
                    let count = colormatch.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let elem = match colormatch.get(2).unwrap().as_str() {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        x => {
                            println!("Unexpected color {}", x);
                            0
                        }
                    };

                    acc[elem] = cmp::max(acc[elem], count);

                    acc
                })
        })
        .fold(0u32, |acc, t| acc + t.iter().product::<u32>())
}

fn main() {
    println!("Solution: {}", solve())
}
