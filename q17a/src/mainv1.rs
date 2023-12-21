use itertools::{Itertools, Position};
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
    X,
}

#[derive(Debug, Clone)]
struct Element {
    rpos: Position,
    cpos: Position,
    val: usize,
    cost: usize,
    visited: bool,
    dir: Dir,
    dircount: usize,
    path: char,
}

fn dirchar(dir: Dir) -> char{
    match dir {
        Dir::N => '^',
        Dir::S => 'v',
        Dir::W => '<',
        Dir::E => '>',
        Dir::X => todo!(),
    }
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn update(map: &mut [Vec<Element>], current: &Element, rownum: usize, colnum: usize, dir: Dir) {
    let newcost = current.cost + map[rownum][colnum].val;

    if newcost < map[rownum][colnum].cost {
        if dir == current.dir {
            if current.dircount == 3 {
                return;
            }

            map[rownum][colnum].dircount = current.dircount + 1;
        } else {
            map[rownum][colnum].dircount = 1;
        }

        map[rownum][colnum].dir = dir;
        map[rownum][colnum].cost = newcost;
    }
    if rownum == 0 && colnum == 8 {
        println!("0x8 visited {}, cost {}, dir {:?} dircount {}", map[rownum][colnum].visited,  map[rownum][colnum].cost,  map[rownum][colnum].dir,  map[rownum][colnum].dircount, );
    }
}

fn solve() -> usize {
    let mut map = read_input()
        .iter()
        .map(|l| {
            l.chars()
                .with_position()
                .map(|(pos, c)| (pos, c as usize - b'0' as usize))
        })
        .with_position()
        .map(|(rpos, row)| {
            row.map(|(cpos, val)| Element {
                rpos,
                cpos,
                val,
                cost: usize::MAX,
                visited: false,
                dir: Dir::X,
                dircount: 0,
                path: ' ',
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    map[0][0].cost = 0;

    while let Some((_, currow, curcol)) = map
        .iter()
        .enumerate()
        .filter_map(|(rownum, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, col)| !col.visited)
                .map(|(colnum, col)| (col.cost, rownum, colnum))
                .min()
        })
        .min()
    {
        let current = map[currow][curcol].clone();

        if current.rpos != Position::First {
            update(&mut map, &current, currow - 1, curcol, Dir::N);
        }

        if current.rpos != Position::Last {
            update(&mut map, &current, currow + 1, curcol, Dir::S);
        }

        if current.cpos != Position::First {
            update(&mut map, &current, currow, curcol - 1, Dir::W);
        }

        if current.cpos != Position::Last {
            update(&mut map, &current, currow, curcol + 1, Dir::E);
        }

        map[currow][curcol].visited = true;

        if currow == map.len()-1 && curcol == map[0].len()-1 {
            break
        }


    }

    {
        let mut r = map.len() - 1;
        let mut c = map[0].len() - 1;

        while (r + c) > 0 {
            map[r][c].path = dirchar(map[r][c].dir.clone());

            match map[r][c].dir {
                Dir::N => r += 1,
                Dir::S => r -= 1,
                Dir::W => c += 1,
                Dir::E => c -= 1,
                Dir::X => todo!(),
            }
        }
    }


    for row in &map {
        for col in row {
            if col.cost == usize::MAX {
                print!("[{}   ? {:?} {}]", col.val, col.dir, col.dircount)

            } else {
            print!("[{} {:>3} {:?} {} {}]", col.val, col.cost, col.dir, col.dircount, col.path)
            }
        }
        println!();
    }
    let mut x = 0;
    for pos in [(0,1),
                (0,2),
                (1,2),
                (1,3),
                (1,4),
                (1,5),
                (0,5),
                (0,6),
                (0,7),
                (0,8),
                (1,8),
                (2,8),
                (2,9),
                (2,10),
                (3,10),
                (3,11),
                
                
                ] {
                    x += map[pos.0][pos.1].val;
                    println!("({}, {}) = {} vs {}", pos.0, pos.1, x, map[pos.0][pos.1].cost);
                }


    map[map.len() - 1][map[0].len() - 1].cost


}

fn main() {
    println!("Solution: {}", solve())
}
