use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
    vx: isize,
    vy: isize,
    vz: isize,
}

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    let re = Regex::new(r"(?<x>[0-9]+), (?<y>[0-9]+), (?<z>[0-9]+) @ +(?<vx>-?[0-9]+), +(?<vy>-?[0-9]+), +(?<vz>-?[0-9]+)").unwrap();

    read_input()
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Pos {
                x: caps.name("x").unwrap().as_str().parse::<isize>().unwrap(),
                y: caps.name("y").unwrap().as_str().parse::<isize>().unwrap(),
                z: caps.name("z").unwrap().as_str().parse::<isize>().unwrap(),
                vx: caps.name("vx").unwrap().as_str().parse::<isize>().unwrap(),
                vy: caps.name("vy").unwrap().as_str().parse::<isize>().unwrap(),
                vz: caps.name("vz").unwrap().as_str().parse::<isize>().unwrap(),
            }
        })
        .combinations(2)
        .filter(|v| {
            if v[0].vx > v[1].vx {
                if v[0].vx / v[1].vx == v[0].vy / v[1].vy &&
                   v[0].vx / v[1].vx == v[0].vz / v[1].vz &&
                   v[0].vx % v[1].vx == 0 &&
                   v[0].vy % v[1].vy == 0 &&
                   v[0].vz % v[1].vz == 0 

                   { println!("Parallel! {:?}", v); }
            } else {
                if v[1].vx / v[0].vx == v[1].vy / v[0].vy &&
                   v[1].vx / v[0].vx == v[1].vz / v[0].vz &&
                   v[1].vx % v[0].vx == 0 &&
                   v[1].vy % v[0].vy == 0 &&
                   v[1].vz % v[0].vz == 0 
                   { println!("Parallel! {:?}", v); }

            }

            true
        })
        .count()
}

fn main() {
    println!("Solution: {}", solve())
}
