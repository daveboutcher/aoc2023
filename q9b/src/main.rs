use regex::Regex;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn firstdiff(nums: &Vec<isize>) -> isize {
    let mut diffs: Vec<isize> = Vec::new();

    let mut same: bool = true;

    for idx in 0..nums.len() - 1 {
        diffs.push(nums[idx + 1] - nums[idx]);

        same = same && (*diffs.last().unwrap() == diffs[0]);
    }

    let newlast = if same {
        diffs[0]
    } else {
        diffs[0] - firstdiff(&diffs)
    };

    println!("{} -> {:?}", newlast, nums);

    newlast
}

fn solve() -> isize {
    let re = Regex::new(r"(-?[0-9]+)").unwrap();

    read_input()
        .iter()
        .map(|l| {
            println!("");
            re.captures_iter(l)
                .map(|c| c.get(1).unwrap().as_str().parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .map(|l| l[0] - firstdiff(&l))
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
