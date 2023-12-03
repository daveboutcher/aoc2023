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
    let re2 = Regex::new(r"[^0-9\.]+").unwrap();

    let lines = read_input();

    let mut total: u32 = 0;

    for (lineno, line) in lines.iter().enumerate() {
        for num in re.captures_iter(line) {
            let nummat = num.get(1).unwrap();
            println!("{} {:?}", lineno, nummat);

            let start = nummat.start() as i32;
            let end = nummat.end() as i32;

            let ilineno = lineno as i32;

            let lmin = cmp::max(ilineno-1, 0) as usize;
            let lmax = cmp::min(ilineno+1, lines.len() as i32-1) as usize + 1;

            let mut found = false;

            for searchline in lmin..lmax {

                let rmin = cmp::max(start-1, 0) as usize;
                let rmax = cmp::min(end+1, line.len() as i32) as usize;
                let rstring = lines[searchline][rmin..rmax].to_string();

                // println!("Checking line {} ({}-{}) range {}-{} {}", searchline, lmin, lmax, rmin, rmax, rstring);
                if let Some(m) = re2.find(&rstring) {
                    found = found | true;

                   //  println!("Found line {} {:?} in {:?}", lineno, m, rstring);
                }
            }

            if found {
                total = total + nummat.as_str().parse::<u32>().unwrap();
            }


        }
    }

    total

 }


fn main() {
    println!("Solution: {}", solve())
}
