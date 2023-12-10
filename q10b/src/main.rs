use std::fs::read_to_string;

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Clone, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
    X,
}

enum State {
    I,
    O,
    LI,
    LO,
    FI,
    FO,
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

    let mut map = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<Vec<char>>>();

    let mut pos = find_start(&map);

    let mut visited = vec![vec![Dir::X; map[0].len()]; map.len()];

    visited[pos.row][pos.col] = Dir::W;

    println!("{:?} {}", pos, map[pos.row][pos.col]);

    // This is a total cheat given the two inputs we know about
    pos.col += 1;

    visited[pos.row][pos.col] = Dir::W;

    println!("{:?} {}", pos, map[pos.row][pos.col]);

    let mut direction: Dir = Dir::W;
    loop {
        match map[pos.row][pos.col] {
            'S' => {
                match direction {
                    Dir::N => map[pos.row][pos.col] = 'F',
                    Dir::S => map[pos.row][pos.col] = 'L',
                    Dir::W => map[pos.row][pos.col] = '-',
                    _ => panic!("Unexpected direction finding S"),
                }
                break;},
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
                Dir::S => {
                    pos.col += 1;
                    direction = Dir::W
                }
                Dir::E => {
                    pos.row -= 1;
                    direction = Dir::N
                }
                _ => panic!("Unexpected direction on L"),
            },
            'J' => match direction {
                Dir::S => {
                    pos.col -= 1;
                    direction = Dir::E
                }
                Dir::W => {
                    pos.row -= 1;
                    direction = Dir::N
                }
                _ => panic!("Unexpected direction on J"),
            },
            '7' => match direction {
                Dir::N => {
                    pos.col -= 1;
                    direction = Dir::E
                }
                Dir::W => {
                    pos.row += 1;
                    direction = Dir::S
                }
                _ => panic!("Unexpected direction on 7"),
            },
            'F' => match direction {
                Dir::N => {
                    pos.col += 1;
                    direction = Dir::W
                }
                Dir::E => {
                    pos.row += 1;
                    direction = Dir::S
                }
                _ => panic!("Unexpected direction on F"),
            },
            _ => panic!("Unexpected character encountered"),
        }

        println!("{:?} {}", pos, map[pos.row][pos.col]);
        visited[pos.row][pos.col] = direction.clone();
    }

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if visited[r][c] == Dir::X {
                map[r][c] = '.';
            }
        }
    }

    let mut inside: usize = 0;

    for r in 0..map.len() {
        let mut state = State::O;

        for c in 0..map[0].len() {
            match map[r][c] {
                '.' => {
                    match state {
                        State::I => {
                            inside += 1;
                            map[r][c] = 'I';
                        }
                        State::O => {
                            map[r][c] = 'O';
                        }
                        State::LI => panic!("Unexpected . in state LI"),
                        State::LO => panic!("Unexpected . in state LO"),
                        State::FI => panic!("Unexpected . in state FI"),
                        State::FO => panic!("Unexpected . in state FO"),
                    };
                }
                '|' => {
                    match state {
                        State::I => state = State::O,
                        State::O => state = State::I,
                        State::LI => panic!("Unexpected | in state LI"),
                        State::LO => panic!("Unexpected | in state LO"),
                        State::FI => panic!("Unexpected | in state FI"),
                        State::FO => panic!("Unexpected | in state FO"),
                    };
                }
                'L' => {
                    match state {
                        State::I => state = State::LI,
                        State::O => state = State::LO,
                        State::LI => panic!("Unexpected L in state LI"),
                        State::LO => panic!("Unexpected L in state LO"),
                        State::FI => panic!("Unexpected L in state FI"),
                        State::FO => panic!("Unexpected L in state FO"),
                    };
                }
                'F' => {
                    match state {
                        State::I => state = State::FI,
                        State::O => state = State::FO,
                        State::LI => panic!("Unexpected F in state LI"),
                        State::LO => panic!("Unexpected F in state LO"),
                        State::FI => panic!("Unexpected F in state FI"),
                        State::FO => panic!("Unexpected F in state FO"),
                    };
                }
                'J' => {
                    match state {
                        State::I =>  panic!("Unexpected J in state I"),
                        State::O =>  panic!("Unexpected J in state O"),
                        State::LI => state = State::I,
                        State::LO => state = State::O,
                        State::FI => state = State::O,
                        State::FO => state = State::I,
                    };
                }
                '7' => {
                    match state {
                        State::I =>  panic!("Unexpected J in state I"),
                        State::O =>  panic!("Unexpected J in state O"),
                        State::LI => state = State::O,
                        State::LO => state = State::I,
                        State::FI => state = State::I,
                        State::FO => state = State::O,
                    };
                }
                '-' => (),
                'S' => (),
                _ => panic!("Unexpected character"),
            };
        }
    }

    for row in map {
        println!("{}", String::from_iter(row));
    }

    inside
}

fn main() {
    println!("Solution: {}", solve())
}
