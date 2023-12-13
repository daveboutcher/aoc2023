use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn process(lines: &Vec<&String>) -> usize {
    let cols = lines
        .iter()
        .map(|line| {
            (1..line.len())
                .filter(|col| {
                    line[0..*col]
                        .chars()
                        .rev()
                        .zip(line[*col..line.len()].chars())
                        .all(|(a, b)| a == b)
                })
                .collect::<HashSet<usize>>()
        })
        .reduce(|acc, set| acc.intersection(&set).copied().collect())
        .unwrap();

    let rows = (1..lines.len())
        .filter(|row| {
            lines[0..*row]
                .iter()
                .rev()
                .zip(lines[*row..lines.len()].iter())
                .all(|(a, b)| a == b)
        })
        .collect::<HashSet<usize>>();

    match (cols.len(), rows.len()) {
        (0, 0) => panic!("Didn't find ANY reflections"),
        (1, 0) => *cols.iter().next().unwrap(),
        (0, 1) => *rows.iter().next().unwrap() * 100,
        (1, 1) => panic!("Found BOTH column and row reflection!"),
        (c, r) => panic!("Just wow: {} {}", c, r),
    }
}

fn solve() -> usize {
    read_input()
        .iter()
        .fold(vec![Vec::<&String>::new()], |mut vec, l| {
            if l.is_empty() {
                vec.push(Vec::<&String>::new());
            } else {
                vec.last_mut().unwrap().push(l);
            }
            vec
        })
        .iter()
        .map(process)
        .sum()
}

fn main() {
    println!("Solution: {}", solve());
}
