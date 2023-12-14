use std::collections::HashMap;
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

    let mut maps = HashMap::<Vec<Vec<char>>, usize>::new();
    let mut weights = Vec::<usize>::new();

    let maxcycle = 1000000000usize;

    for cycle in 0..maxcycle {
        // North
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

        // West
        for line in 0..lines.len() {
            for col in 1..grid[0].len() {
                if grid[line][col] == 'O' {
                    let mut ncol = col - 1;
                    while grid[line][ncol] == '.' {
                        grid[line][ncol + 1] = '.';
                        grid[line][ncol] = 'O';

                        if ncol == 0 {
                            break;
                        } else {
                            ncol -= 1;
                        }
                    }
                }
            }
        }

        // South
        for col in 0..grid[0].len() {
            for line in (0..lines.len() - 1).rev() {
                if grid[line][col] == 'O' {
                    let mut nline = line + 1;
                    while grid[nline][col] == '.' {
                        grid[nline - 1][col] = '.';
                        grid[nline][col] = 'O';

                        if nline == lines.len() - 1 {
                            break;
                        } else {
                            nline += 1;
                        }
                    }
                }
            }
        }

        // East
        for line in 0..lines.len() {
            for col in (0..grid[0].len() - 1).rev() {
                if grid[line][col] == 'O' {
                    let mut ncol = col + 1;
                    while grid[line][ncol] == '.' {
                        grid[line][ncol - 1] = '.';
                        grid[line][ncol] = 'O';

                        if ncol == grid[0].len() - 1 {
                            break;
                        } else {
                            ncol += 1;
                        }
                    }
                }
            }
        }

        weights.push(
            grid.iter()
                .enumerate()
                .map(|(lineno, l)| l.iter().filter(|c| **c == 'O').count() * (grid.len() - lineno))
                .sum(),
        );

        // Note: I was considering doing this with a bitmap, instead of a Vec<Vec<char>>, but
        // the cycles were so small that it wasn't necessary.  The purist in me still thinks
        // it would be nice...
        if let Some(c) = maps.insert(grid.clone(), cycle) {
            return weights[c + (maxcycle - c) % (cycle - c) - 1];
        }
    }

    panic!("Didn't find a solution...")
}

fn main() {
    println!("Solution: {}", solve());
}
