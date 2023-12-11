use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> i32 {
    let lines = read_input().iter().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let mut offsets = Vec::new();
    let mut offset:usize = 0;
    for col in 0..lines[0].len() {
        let mut empty:bool = true;
        for row in 0..lines.len() {
            if lines[row][col] != '.' {
                empty = false;
                break;
            }
        }
        if empty {
            offset += 1;
        }
        offsets.push(offset);
    }

    let mut positions: Vec<Pos> = Vec::new();

    let mut total:i32 = 0;

    let mut rownum: usize = 0;
    for row in lines {
        if row.iter().all(|c| *c == '.') {
            rownum += 2;
            continue;
        }
        rownum += 1;
        for col in 0..row.len() {
            let colnum:usize = col + offsets[col];
            if row[col] == '#' {
                for p in &positions {
                    let distance = (p.row as i32 - rownum as i32).abs() + (p.col as i32- colnum as i32).abs();
                    total += distance;
                }

                positions.push(Pos{row:rownum, col:colnum});
            }
        }
    }

    total
}

fn main() {
    println!("Solution: {}", solve())
}
