use itertools::Itertools;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Pos {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
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

    let min_coord = 7.0;
    let max_coord = 27.0;

    let min_coord = 200000000000000.0;
    let max_coord = 400000000000000.0;

    read_input()
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Pos {
                x: caps.name("x").unwrap().as_str().parse::<f64>().unwrap(),
                y: caps.name("y").unwrap().as_str().parse::<f64>().unwrap(),
                vx: caps.name("vx").unwrap().as_str().parse::<f64>().unwrap(),
                vy: caps.name("vy").unwrap().as_str().parse::<f64>().unwrap(),
            }
        })
        .combinations(2)
        .filter(|v| {
            let x1 = v[0].x;
            let y1 = v[0].y;
            let x2 = v[0].x + v[0].vx * 1000.0;
            let y2 = v[0].y + v[0].vy * 1000.0;
            let x3 = v[1].x;
            let y3 = v[1].y;
            let x4 = v[1].x + v[1].vx * 1000.0;
            let y4 = v[1].y + v[1].vy * 1000.0;

            let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

            if denom == 0.0 {
                println!("{:?} are parallel", v);
                false
            } else {
                let px =
                    ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denom;
                let py =
                    ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denom;

                let future = 
                (px - x1) / v[0].vx >= 0.0 &&
                (py - y1) / v[0].vy >= 0.0 &&
                (px - x3) / v[1].vx >= 0.0 &&
                (py - y3) / v[1].vy >= 0.0;

                /*  println!("{:?} meet at {:.3} x {:.3} {} ({} {} {} {})", v, px, py, if future { "" } else { "IN THE PAST"},
                (px - x1) / v[0].vx,
                (py - y1) / v[0].vy,
                (px - x3) / v[1].vx, 
                (py - y3) / v[1].vy
            
            );
                */

                px >= min_coord
                    && px <= max_coord
                    && py >= min_coord
                    && py <= max_coord
                    && future
            }
        })
        .count()
}

fn main() {
    println!("Solution: {}", solve())
}
