use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn number_substitute(mut s: String) -> String {
    let subs = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
            ];

    while let Some(rep) = 
    subs
        .iter()
        .map(|m| (s.find(m.0), m.0, m.1))
        .filter(|s| s.0.is_some())
        .min()
         {
            s = s.replace(rep.1, rep.2);
        }

    s
}

fn solve() -> u32 {
    let lines = read_input();
    lines
    .iter()
    .map(|l| number_substitute(l.clone())
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
    )
    .map(|v:Vec<u32>| v[0]*10 + v.last().unwrap())
    .sum()
}


fn main() {
    println!("Solution: {}", solve())
}
