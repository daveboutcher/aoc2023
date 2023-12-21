use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn add_next(map: &[Vec<char>], nextmap: &mut [Vec<char>], row: isize, col: isize) {
    let rows = map.len() as isize;
    let cols = map[0].len() as isize;

    if row < 0 || col < 0 || row >= rows || col >= cols {
        return;
    }

    let row = row as usize;
    let col = col as usize;

    if map[row][col] == '#' {
        return;
    }

    nextmap[row][col] = 'O';
}

fn solve() -> usize {
    let map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (startrow, startcol) = map
        .iter()
        .enumerate()
        .find_map(|(rownum, row)| {
            row.iter()
                .enumerate()
                .find_map(|(colnum, c)| if *c == 'S' { Some(colnum) } else { None })
                .map(|colnum| (rownum, colnum))
        })
        .unwrap();

    let rows = map.len();
    let cols = map[0].len();

    let mut startmap = vec![vec!['.'; cols]; rows];
    startmap[startrow][startcol] = 'O';

    for _step in 0..64 {
        let mut nextmap = vec![vec!['.'; cols]; rows];
        for (rownum, row) in startmap.iter().enumerate() {
            for (colnum, _) in row.iter().enumerate().filter(|(_, c)| **c == 'O') {
                add_next(&map, &mut nextmap, rownum as isize - 1, colnum as isize);
                add_next(&map, &mut nextmap, rownum as isize + 1, colnum as isize);
                add_next(&map, &mut nextmap, rownum as isize, colnum as isize - 1);
                add_next(&map, &mut nextmap, rownum as isize, colnum as isize + 1);
            }
        }

        /* 
        println!("Step {} ", step);
        for (rownum, row) in nextmap.iter().enumerate() {
            for (colnum, col) in row.iter().enumerate() {
                if *col == '.' {
                    print!("{}", map[rownum][colnum]);
                } else {
                    print!("{}", col);
                }
            }
            println!();
        }
        */

        startmap = nextmap;
    }

    startmap
        .iter()
        .map(|row| row.iter().filter(|c| **c == 'O').count())
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
