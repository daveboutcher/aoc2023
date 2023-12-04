use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> u32 {
    let lines = read_input();
    lines
        .iter()
        .map(|l| {
            l.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
                .chars()
                .filter_map(|c| c.to_digit(10))
                .collect()
        })
        .map(|v: Vec<u32>| v[0] * 10 + v.last().unwrap())
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
