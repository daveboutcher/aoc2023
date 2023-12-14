use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    let lines = read_input();

    let mut grid = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();


    for col in 0..grid[0].len() {
        for line in 1..lines.len() {
            if grid[line][col] == 'O' {
                let mut nline = line - 1;
                while grid[nline][col] == '.' {
                    grid[nline + 1][col] = '.';
                    grid[nline][col] = 'O';

                    if nline == 0 {
                        break;
                    } else {
                        nline -= 1;
                    }
                }
            }
        }
    }

    grid.iter()
    .enumerate()
    .map(|(lineno, l)| l.iter().filter(|c| **c == 'O').count() * (grid.len() - lineno))
    .sum()
}

fn main() {
    println!("Solution: {}", solve());
}
