use std::borrow::Borrow;
use std::collections::LinkedList;
use std::fs::read_to_string;
use std::rc::Rc;
use std::mem;

#[derive(Debug, PartialEq)]
struct Pos {
    x: u16,
    y: u16,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug, PartialEq)]
struct State {
    pos: Pos,
    prev: Option<Rc<State>>,
}

fn contains(mut state: &State, pos: &Pos) -> bool {
    if state.pos == *pos {
        return true;
    }

    while let Some(rstate) = &state.prev {
        state = rstate.borrow();

        if state.pos == *pos {
            return true;
        }
    }

    false
}

fn add_state(todo: &mut LinkedList<Rc<State>>, x: u16, y: u16, state: &Rc<State>) {
    todo.push_back(Rc::new(State {
        pos: Pos { x: x, y: y },
        prev: Some(state.clone()),
    }))
}

fn solve() -> usize {
    let map = read_input()
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut todo: LinkedList<Rc<State>> = LinkedList::new();

    let first = Rc::new(State {
        pos: Pos { x: 1, y: 0 },
        prev: None,
    });

    todo.push_back(Rc::new(State {
        pos: Pos { x: 1, y: 1 },
        prev: Some(first.clone()),
    }));

    while let Some(state) = todo.pop_front() {
        if contains(&state, &state.pos) {
            continue;
        }

        match map[state.pos.y as usize][state.pos.x as usize] {
            '#' => continue,
            '>' => add_state(&mut todo, state.pos.x+1, state.pos.y, &state),
            _ => panic!("Unexpected map character"),
        }
    }

    0
}

fn main() {
    println!("Size of Pos is {}, size of state is {}", mem::size_of::<Pos>(), mem::size_of::<State>());

    println!("Solution: {}", solve())
}
