use regex::Regex;
use std::cmp;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn domap(maps: &[Vec<(usize, usize, usize)>], mut cur: usize, mut count: usize) -> usize {
    if maps.len() == 0 {
        cur
    } else {
        let mut loc = std::usize::MAX;
        let mut nextcur = cur;
        let mut nextcount = count;

        while count > 0 {
            cur = nextcur;
            count = nextcount;
            let mut found = false;
            let mut closest = std::usize::MAX;
            for conv in &maps[0] {
                if cur >= conv.0 && cur < (conv.0 + conv.2) {
                    found = true;
                    let chunksize = cmp::min(count, conv.2 - (cur - conv.0));
                    nextcount -= cmp::min(count, chunksize);
                    nextcur = cur + chunksize;

                    cur = conv.1 + (cur - conv.0);
                    count = chunksize;

                    break;
                } else {
                    if conv.0 > cur {
                        closest = cmp::min(closest, conv.0);
                    }
                }
            }

            if !found {
                nextcur = closest;
                nextcount -= cmp::min(count, closest - cur);
            }

            loc = cmp::min(domap(&maps[1..], cur, count), loc)
        }
        loc
    }
}

fn solve() -> usize {
    let re = Regex::new(r"([0-9]+)").unwrap();

    let lines = read_input();

    let mut loc = std::usize::MAX;

    let mut maps = Vec::new();
    let mut map = Vec::new();

    for line in &lines[2..] {
        if line.contains(":") {
            println!("Processing {}", line);
            continue;
        } else if line.len() == 0 {
            maps.push(map);
            map = Vec::new();
        } else {
            let vals: Vec<usize> = re
                .captures_iter(&line)
                .map(|c| c.get(1).unwrap().as_str().parse::<usize>().unwrap())
                .collect();

            let dstart = vals[0];
            let sstart = vals[1];
            let count = vals[2];

            map.push((sstart, dstart, count));
        }
    }

    println!("starting seeds...");

    for seedpair in re
        .captures_iter(&lines[0])
        .map(|c| c.get(1).unwrap().as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .chunks(2)
    {
        loc = cmp::min(loc, domap(&maps[..], seedpair[0], seedpair[1]));
    }

    loc
}

fn main() {
    println!("Solution: {}", solve())
}
