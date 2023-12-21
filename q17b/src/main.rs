use itertools::{Itertools, Position};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
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
    path: Vec<(usize, usize)>,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    row: usize,
    col: usize,
    dir: Dir,
    count: usize,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other)) // Delegate to the implementation in `Ord`.
    }
}

type VisitMap = HashSet<(usize, usize, usize, usize)>;

fn update(
    map: &mut [Vec<Element>],
    heap: &mut BinaryHeap<State>,
    visited: &mut VisitMap,
    state: &State,
    row: usize,
    col: usize,
    dir: Dir,
) {
    let count = if dir == state.dir { state.count + 1 } else { 1 };

    if count > 10 {
        return;
    }

    let newcost = state.cost + map[row][col].val;
    if count > 3 && newcost < map[row][col].cost {
        map[row][col].cost = newcost;
        map[row][col].path = state.path.clone();
    }

    let dirnum: usize = match dir {
        Dir::N => 0,
        Dir::S => 1,
        Dir::E => 2,
        Dir::W => 3,
        Dir::X => 4,
    };

    if visited.contains(&(row, col, count, dirnum)) {
        return;
    }

    visited.insert((row, col, count, dirnum));

    let mut newpath = state.path.clone();
    newpath.push((row, col));

    heap.push(State {
        cost: newcost,
        row,
        col,
        count,
        dir,
        path: newpath,
    });
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
                path: Vec::new(),
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut heap = BinaryHeap::new();
    let mut visited = VisitMap::new();

    map[0][0].cost = 0;

    heap.push(State {
        cost: 0,
        row: 0,
        col: 0,

        dir: Dir::E,
        count: 0,
        path: Vec::new(),
    });

    heap.push(State {
        cost: 0,
        row: 0,
        col: 0,

        dir: Dir::S,
        count: 0,
        path: Vec::new(),
    });
    while let Some(state) = heap.pop() {
        if map[state.row][state.col].rpos != Position::First &&  // no overrun
         ((state.count < 4 && state.dir == Dir::N) || // Short case
         (state.count >= 4 && state.dir != Dir::S))
        {
            update(
                &mut map,
                &mut heap,
                &mut visited,
                &state,
                state.row - 1,
                state.col,
                Dir::N,
            );
        }

        if map[state.row][state.col].rpos != Position::Last &&  // no overrun
        ((state.count < 4 && state.dir == Dir::S) || // Short case
        (state.count >= 4 && state.dir != Dir::N))
        {
            update(
                &mut map,
                &mut heap,
                &mut visited,
                &state,
                state.row + 1,
                state.col,
                Dir::S,
            );
        }

        if map[state.row][state.col].cpos != Position::First && // no overrun
        ((state.count < 4 && state.dir == Dir::W) || // Short case
        (state.count >= 4 && state.dir != Dir::E))
        {
            update(
                &mut map,
                &mut heap,
                &mut visited,
                &state,
                state.row,
                state.col - 1,
                Dir::W,
            );
        }

        if map[state.row][state.col].cpos != Position::Last && // no overrun
        ((state.count < 4 && state.dir == Dir::E) || // Short case
        (state.count >= 4 && state.dir != Dir::W))
        {
            update(
                &mut map,
                &mut heap,
                &mut visited,
                &state,
                state.row,
                state.col + 1,
                Dir::E,
            );
        }
    }

    map[map.len() - 1][map[0].len() - 1].cost
}

fn main() {
    println!("Solution: {}", solve())
}
