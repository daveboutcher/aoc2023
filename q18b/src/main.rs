use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Pos {
    row: i64,
    col: i64,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn area(vertices: &[Pos]) -> u64 {
    (0..vertices.len())
        .map(|i| {
            let pos1 = &vertices[i];
            let pos2 = &vertices[(i + 1) % vertices.len()];

            pos1.col * pos2.row - pos1.row * pos2.col
                + (pos1.row - pos2.row).abs()
                + (pos1.col - pos2.col).abs()
        })
        .sum::<i64>()
        .unsigned_abs()
        / 2
        + 1
}

fn solve() -> u64 {
    let re = Regex::new(r"([RDLU]) ([0-9]+) \(#([0-9a-f]{6})\)").unwrap();

    let mut pos = Pos { row: 0, col: 0 };

    let mut vertices: Vec<Pos> = Vec::new();
    

    vertices.push(pos.clone());

    for (_dir, _distance, color) in read_input().iter().map(|line| {
        if let Some(captures) = re.captures(line) {
            (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                captures.get(3).unwrap().as_str(),
            )
        } else {
            panic!("Problem with line: {}", line);
        }
    }) {
        let distance = i64::from_str_radix(&color[0..5], 16).unwrap();
        let dir = &color[5..6];

        match dir {
            "0" => pos.col += distance,
            "1" => pos.row += distance,
            "2" => pos.col -= distance,
            "3" => pos.row -= distance,
            _ => panic!("Unexpected character"),
        }

        vertices.push(pos.clone());
    }

    area(&vertices)
}

fn main() {
    println!("Solution: {}", solve())
}
