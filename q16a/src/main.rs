use itertools::Position;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
}

struct Map {
    map: Vec<Vec<char>>,
    energized: Vec<Vec<bool>>,
    visited: HashSet<(Pos, Dir)>,
    debug: bool,
}

impl Map {
    fn new(map: Vec<Vec<char>>, debug: bool) -> Map {
        Map {
            energized: vec![vec![false; map[0].len()]; map.len()],
            visited: HashSet::<(Pos, Dir)>::new(),
            map,
            debug,
        }
    }

    fn position(&self, val: usize, limit: usize) -> Position {
        if val == 0 {
            Position::First
        } else if val == limit - 1 {
            Position::Last
        } else {
            Position::Middle
        }
    }

    fn next_pos(&self, pos: &Pos, dir: &Dir) -> Option<Pos> {
        match (
            dir,
            self.position(pos.row, self.map.len()),
            self.position(pos.col, self.map[0].len()),
        ) {
            (Dir::N, Position::First, _) => None,
            (Dir::N, _, _) => Some(Pos {
                row: pos.row - 1,
                col: pos.col,
            }),
            (Dir::S, Position::Last, _) => None,
            (Dir::S, _, _) => Some(Pos {
                row: pos.row + 1,
                col: pos.col,
            }),
            (Dir::W, _, Position::First) => None,
            (Dir::W, _, _) => Some(Pos {
                row: pos.row,
                col: pos.col - 1,
            }),
            (Dir::E, _, Position::Last) => None,
            (Dir::E, _, _) => Some(Pos {
                row: pos.row,
                col: pos.col + 1,
            }),
        }
    }

    fn advance(&mut self, pos: &Pos, dir: Dir) {
        if let Some(pos) = self.next_pos(pos, &dir) {
            self.do_beam(&pos, dir);
        }
    }

    fn do_beam(&mut self, pos: &Pos, dir: Dir) {
        if self.visited.contains(&(pos.clone(), dir.clone())) {
            return;
        } else {
            self.visited.insert((pos.clone(), dir.clone()));
        }

        self.energized[pos.row][pos.col] = true;

        if self.debug {
            self.print(pos, &dir);
        }

        match (self.map[pos.row][pos.col], &dir) {
            ('.', _) => self.advance(pos, dir),
            ('|', Dir::N | Dir::S) => self.advance(pos, dir),
            ('|', Dir::E | Dir::W) => {
                self.advance(pos, Dir::N);
                self.advance(pos, Dir::S);
            }
            ('-', Dir::E | Dir::W) => self.advance(pos, dir),
            ('-', Dir::N | Dir::S) => {
                self.advance(pos, Dir::E);
                self.advance(pos, Dir::W);
            }
            ('\\', Dir::N) => self.advance(pos, Dir::W),
            ('\\', Dir::S) => self.advance(pos, Dir::E),
            ('\\', Dir::W) => self.advance(pos, Dir::N),
            ('\\', Dir::E) => self.advance(pos, Dir::S),
            ('/', Dir::N) => self.advance(pos, Dir::E),
            ('/', Dir::S) => self.advance(pos, Dir::W),
            ('/', Dir::W) => self.advance(pos, Dir::S),
            ('/', Dir::E) => self.advance(pos, Dir::N),
            _ => (),
        }
    }

    fn num_energized(&self) -> usize {
        self.energized
            .iter()
            .map(|r| r.iter().filter(|v| **v).count())
            .sum()
    }

    fn print(&self, pos: &Pos, dir: &Dir) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for (rownum, row) in self.map.iter().enumerate() {
            for (colnum, col) in row.iter().enumerate() {
                let sep = if rownum == pos.row && colnum == pos.col {
                    match dir {
                        Dir::N => '^',
                        Dir::S => 'v',
                        Dir::W => '<',
                        Dir::E => '>',
                    }
                } else {
                    ' '
                };

                print!("{}{}{}", sep, col, sep);
            }

            print!("  ");

            for c in &self.energized[rownum] {
                print!("{}", if *c { '#' } else { '.' });
            }

            println!();
        }
        println!();
    }
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    let lines = read_input();

    let mut map = Map::new(
        lines
            .iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<char>>>(),
        false, // Debug
    );

    map.do_beam(&Pos { row: 0, col: 0 }, Dir::E);

    map.num_energized()
}

fn main() {
    println!("Solution: {}", solve())
}
