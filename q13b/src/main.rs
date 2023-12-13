use std::collections::HashSet;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn calc(lines: &Vec<String>) -> (HashSet<usize>, HashSet<usize>) {
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

    (cols, rows)
}

fn process_one(mut lines: Vec<String>, row: usize, col: usize) -> Option<usize> {
    let (ocols, orows) = calc(&lines);

    let rep = match lines[row].chars().nth(col).unwrap() {
        '.' => "#",
        '#' => ".",
        _ => panic!("Unexpected char"),
    };

    lines[row].replace_range(col..col + 1, rep);

    let (ncols, nrows) = calc(&lines);

    let cols = ncols.difference(&ocols).collect::<Vec<&usize>>();
    let rows = nrows.difference(&orows).collect::<Vec<&usize>>();

    match (cols.len(), rows.len()) {
        (0, 0) => None,
        (1, 0) => Some(**cols.first().unwrap()),
        (0, 1) => Some(**rows.first().unwrap() * 100),
        (1, 1) => panic!("Found BOTH column and row reflection!"),
        (c, r) => panic!("Just wow: {} {}", c, r),
    }
}

fn process(lines: &Vec<String>) -> usize {
    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            if let Some(val) = process_one(lines.clone(), row, col) {
                return val;
            }
        }
    }
    panic!("No reflections found")
}

fn solve() -> usize {
    read_input()
        .iter()
        .fold(vec![Vec::<String>::new()], |mut vec, l| {
            if l.is_empty() {
                vec.push(Vec::<String>::new());
            } else {
                vec.last_mut().unwrap().push(l.clone());
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
