use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0usize, |acc, c| ((acc + c as usize) * 17) % 256)
}

fn solve() -> usize {
    read_input()
        .first()
        .unwrap()
        .split(',')
        .fold(
            vec![Vec::<(String, usize)>::new(); 256],
            |mut boxes, cmd| {
                if cmd.ends_with('-') {
                    let label = &cmd[0..cmd.len() - 1];

                    boxes[hash(label)].retain(|s| s.0 != label);

                    boxes
                } else {
                    let pieces = cmd.split('=').collect::<Vec<&str>>();
                    let label = pieces[0];

                    let boxnum = hash(label);

                    let val: usize = pieces[1].parse::<usize>().unwrap();

                    let mut found: bool = false;
                    for t in boxes[boxnum].iter_mut() {
                        if t.0 == label {
                            t.1 = val;
                            found = true;
                        }
                    }

                    if !found {
                        boxes[boxnum].push((label.to_string(), val));
                    }

                    boxes
                }
            },
        )
        .iter()
        .enumerate()
        .map(|(boxnum, bx)| {
            (boxnum + 1)
                * bx.iter()
                    .enumerate()
                    .map(|(slotnum, (_, val))| (slotnum + 1) * val)
                    .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("Solution: {}", solve());
}
