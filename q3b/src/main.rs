use std::fs::read_to_string;
use regex::Regex;
use std::cmp;

fn read_input() -> Vec<String> {
    read_to_string("input.txt") 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}


fn solve() -> u32 {
    let re = Regex::new(r"([0-9]+)").unwrap();
    let re2 = Regex::new(r"\*").unwrap();

    let lines = read_input();

    let mut total: u32 = 0;

    let mut linesnums = Vec::new();

    for (lineno, line) in lines.iter().enumerate() {
        let mut linenums = Vec::new();

        for num in re.captures_iter(line) {
            let nummat = num.get(1).unwrap();

            linenums.push(nummat);
        }

        linesnums.push(linenums);
    }

    for (lineno, line) in lines.iter().enumerate() {
        for star in re2.find_iter(line) {
            let mut matches = Vec::new();

            let lmin = cmp::max((cmp::max(lineno, 1))-1, 0);
            let lmax = cmp::min(lineno+1, lines.len()-1) + 1;

            let mut found = false;

            for searchline in lmin..lmax {
                for num in &linesnums[searchline] {
                    let rmin = cmp::max((cmp::max(num.start(), 1))-1, 0);
                    let rmax = num.end() + 1;

                    if (rmin..rmax).contains(&star.start()) {
                        matches.push(num.as_str().parse::<u32>().unwrap())
                        //println!("Found * at line {} col {} matches line {} {:?} {}..{}", lineno, star.start(), searchline, num, rmin, rmax);
                    }
                }            
            }

            if matches.len() == 2 {
                total = total + (matches[0] * matches[1]);
            }
        }
    }

    total

 }


fn main() {
    println!("Solution: {}", solve())
}
