use std::fs::read_to_string;
use regex::Regex;

fn read_input() -> Vec<String> {
    read_to_string("input.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn solve() -> isize {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let lines = read_input();

    re
    .captures_iter(&lines[0].replace(" ",""))
    .map(|c| c.get(1).unwrap().as_str().parse::<isize>().unwrap())
    .zip(re
        .captures_iter(&lines[1].replace(" ",""))
        .map(|c| c.get(1).unwrap().as_str().parse::<isize>().unwrap()))
    .map(|(time, distance)| time + 1 - ((((((time*time-4*distance) as f64).sqrt() - time as f64) / -2.0f64) + 0.0000001f64).ceil() as isize) * 2)
    .product()
 }


fn main() {
    println!("Solution: {}", solve())
}
