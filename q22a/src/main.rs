use std::cmp;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug, Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Pos {
    z: usize,
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Stack {
    height: usize,
    block: isize,
}

fn parse_loc(loc: &str) -> Pos {
    let pieces = loc
        .split(',')
        .map(|p| p.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Pos {
        z: pieces[2],
        x: pieces[0],
        y: pieces[1],
    }
}

fn blockname(id: isize) -> char {
    if id < 0 {
        '.'
    } else {
    ('A' as u8 + id as u8) as char
    }
}

fn solve() -> usize {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;

    let mut blocks = read_input()
        .iter()
        .enumerate()
        .map(|(blockno, l)| {
            let ps = l.split('~').map(parse_loc).collect::<Vec<Pos>>();
            max_x = cmp::max(ps[1].x, max_x);
            max_y = cmp::max(ps[1].y, max_y);
            Reverse((ps[0].clone(), ps[1].clone(), blockno as isize))
        })
        .collect::<BinaryHeap<Reverse<(Pos, Pos, isize)>>>();

    let mut stacks = vec![
        vec![
            Stack {
                height: 0,
                block: -1
            };
            max_y+1
        ];
        max_x+1
    ];

    let nblocks = blocks.len();
    let mut supports = vec![Vec::<isize>::new(); nblocks];
    let mut supported_by = vec![Vec::<isize>::new(); nblocks];

    while let Some(rpos) = blocks.pop() {

        let (a, b, blockno) = rpos.0;
        let mut max_z: usize = 0;
        for stack_row in stacks.iter().take(b.x + 1).skip(a.x) {
            for stack in stack_row.iter().take(b.y + 1).skip(a.y) {
                max_z = cmp::max(stack.height, max_z);
            }
        }

        let new_max_z = max_z + b.z - a.z + 1;

        for stack_row in stacks.iter_mut().take(b.x + 1).skip(a.x) {
            for stack in stack_row.iter_mut().take(b.y + 1).skip(a.y) {
                if stack.height == max_z && stack.block != -1 {
                    if !supports[stack.block as usize].contains(&blockno) {
                        supports[stack.block as usize].push(blockno)
                    }

                    if !supported_by[blockno as usize].contains(&stack.block) {
                        supported_by[blockno as usize].push(stack.block);
                    }
                }

                stack.height = new_max_z;
                stack.block = blockno;
            }
        }
    }


    supports
    .iter()
    .filter(|v| !v.iter().any(|blockno| supported_by[*blockno as usize].len() == 1))
    .count()
}

fn main() {
    println!("Solution: {}", solve())
}
