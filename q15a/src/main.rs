use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    read_input()
        .first()
        .unwrap()
        .chars()
        .fold([0usize, 0usize], |[total, current], c| {
            if c == ',' {
                [total + current, 0]
            } else {
                [total, ((current + c as usize) * 17) % 256]
            }
        })
        .iter()
        .sum()
    }

fn main() {
    println!("Solution: {}", solve());
}
