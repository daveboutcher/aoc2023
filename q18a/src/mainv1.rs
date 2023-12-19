use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

enum Dir {
    U,
    D,
    X,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn fill(map: &mut [Vec<char>], pos: &Pos, c: char) {
    if map[pos.row][pos.col] == '#' {
        println!("Weird, double digging");
    }

    map[pos.row][pos.col] = c;
}

fn move_right(map: &mut [Vec<char>], pos: &mut Pos) {
    pos.col += 1;

    if pos.col == map[0].len() {
        for row in map.iter_mut() {
            row.push('.')
        }
    }

    fill(map, pos, 'h');
}

fn move_down(map: &mut Vec<Vec<char>>, pos: &mut Pos) {
    map[pos.row][pos.col] = 'v';

    pos.row += 1;

    if pos.row == map.len() {
        map.push(vec!['.'; map[0].len()])
    }

    fill(map, pos, 'v');
}

fn move_left(map: &mut [Vec<char>], pos: &mut Pos) {
    if pos.col == 0 {
        for row in map.iter_mut() {
            row.insert(0, '.');
        }
    } else {
        pos.col -= 1;
    }
    fill(map, pos, 'h');
}

fn move_up(map: &mut Vec<Vec<char>>, pos: &mut Pos) {
    map[pos.row][pos.col] = '^';

    if pos.row == 0 {
        map.insert(0, vec!['.'; map[0].len()]);
    } else {
        pos.row -= 1;
    }
    fill(map, pos, '^');
}

fn vadd(v: &mut Vec<char>, c: char) -> Vec<char> {
    v.push(c);
    v.to_vec()
}

fn solve() -> usize {
    let re = Regex::new(r"([RDLU]) ([0-9]+) \(#([0-9a-f]{6})\)").unwrap();

    let mut map: Vec<Vec<char>> = vec![vec!['#']];
    let mut pos = Pos { row: 0, col: 0 };

    for (dir, distance, _color) in read_input().iter().map(|line| {
        if let Some(captures) = re.captures(line) {
            (
                captures.get(1).unwrap().as_str(),
                captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
                captures.get(3).unwrap().as_str(),
            )
        } else {
            panic!("Problem with line: {}", line);
        }
    }) {
        for _ in 0..distance {
            match dir {
                "R" => move_right(&mut map, &mut pos),
                "L" => move_left(&mut map, &mut pos),
                "D" => move_down(&mut map, &mut pos),
                "U" => move_up(&mut map, &mut pos),
                c => panic!("Unexpected direction char {}", c),
            };
        }
    }

    for row in map
        .iter_mut()
        .map(|row| {
            row.iter()
                .fold((0, Dir::X, Vec::<char>::new()), |mut acc, c| match c {
                    'h' => (acc.0, acc.1, vadd(&mut acc.2, 'h')),

                    'v' => match acc.1 {
                        Dir::D => (acc.0, Dir::D, vadd(&mut acc.2, 'v')),
                        Dir::U => (acc.0 + 1, Dir::D, vadd(&mut acc.2, 'v')),
                        Dir::X => (acc.0 + 1, Dir::D, vadd(&mut acc.2, 'v')),
                    },

                    '^' => match acc.1 {
                        Dir::D => (acc.0 + 1, Dir::U, vadd(&mut acc.2, '^')),
                        Dir::U => (acc.0, Dir::U, vadd(&mut acc.2, '^')),
                        Dir::X => (acc.0 + 1, Dir::U, vadd(&mut acc.2, '^')),
                    },

                    '.' => (
                        acc.0,
                        acc.1,
                        if acc.0 % 2 == 0 {
                            vadd(&mut acc.2, '.')
                        } else {
                            vadd(&mut acc.2, 'o')
                        },
                    ),

                    c => panic!("Unexpected map char {}", c),
                })
        })
        .map(|(_, _, row)| row.iter().collect::<String>())
    {
        println!("{}", row);
    }

    map.iter()
        .map(|row| {
            row.iter().fold((0, Dir::X, 0), |acc, c| match c {
                'h' => (acc.0, acc.1, acc.2 + 1),

                'v' => match acc.1 {
                    Dir::D => (acc.0, Dir::D, acc.2 + 1),
                    Dir::U => (acc.0 + 1, Dir::D, acc.2 + 1),
                    Dir::X => (acc.0 + 1, Dir::D, acc.2 + 1),
                },

                '^' => match acc.1 {
                    Dir::D => (acc.0 + 1, Dir::U, acc.2 + 1),
                    Dir::U => (acc.0, Dir::U, acc.2 + 1),
                    Dir::X => (acc.0 + 1, Dir::U, acc.2 + 1),
                },

                '.' => (acc.0, acc.1, if acc.0 % 2 == 0 { acc.2 } else { acc.2 + 1 }),

                c => panic!("Unexpected map char {}", c),
            })
        })
        .map(|(_, _, total)| total)
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
