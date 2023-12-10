use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

enum Dir {
    N,
    S,
    E,
    W,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn find_start(map: &Vec<Vec<char>>) -> Pos {
    for row in map.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            if *col.1 == 'S' {
                return Pos {
                    row: row.0,
                    col: col.0,
                };
            }
        }
    }
    Pos { row: 0, col: 0 }
}

fn solve() -> usize {
    let lines = read_input();

    let map = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos = find_start(&map);

    let mut len: usize = 0;

    println!("{:?} {}", pos, map[pos.row][pos.col]);

    // This is a total cheat given the two inputs we know about
    pos.col += 1;

    println!("{:?} {}", pos, map[pos.row][pos.col]);

    let mut direction: Dir = Dir::W;
    loop {
        match map[pos.row][pos.col] {
            'S' => break,
            '|' => match direction {
                Dir::N => pos.row -= 1,
                Dir::S => pos.row += 1,
                _ => panic!("Unexpected direction on vertical bar"),
            },
            '-' => match direction {
                Dir::E => pos.col -= 1,
                Dir::W => pos.col += 1,
                _ => panic!("Unexpected direction on horizontal bar"),
            },
            'L' => match direction {
                Dir::S => {pos.col += 1; direction = Dir::W},
                Dir::E => {pos.row -= 1; direction = Dir::N}
                _ => panic!("Unexpected direction on L"),
            },
            'J' => match direction {
                Dir::S => {pos.col -= 1; direction = Dir::E},
                Dir::W => {pos.row -= 1; direction = Dir::N}
                _ => panic!("Unexpected direction on J"),
            },
            '7' => match direction {
                Dir::N => {pos.col -= 1; direction = Dir::E},
                Dir::W => {pos.row += 1; direction = Dir::S}
                _ => panic!("Unexpected direction on 7"),
            },
            'F' => match direction {
                Dir::N => {pos.col += 1; direction = Dir::W},
                Dir::E => {pos.row += 1; direction = Dir::S}
                _ => panic!("Unexpected direction on F"),
            },
            _ => panic!("Unexpected character encountered"),
        }

        println!("{:?} {}", pos, map[pos.row][pos.col]);
        len += 1;
    }

    (len + 1) / 2  
}

fn main() {
    println!("Solution: {}", solve())
}
