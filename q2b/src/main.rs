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
    let re = Regex::new(r"([0-9]+) (blue|green|red)").unwrap();

    let lines = read_input();

    let mut total: u32 = 0;

    for (gameno, line) in lines.iter().enumerate() {
        let mut red:u32 = 0;
        let mut green:u32 = 0;
        let mut blue:u32 = 0;

        for color in re.captures_iter(line) {
            let count =  color.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let color = color.get(2).unwrap().as_str();

            match color {
                "red" => red = cmp::max(red, count),
                "green" => green = cmp::max(green, count), 
                "blue" => blue = cmp::max(blue, count),
                x => println!("Unexpected color {}", x),
            };
        }  

        total = total + (red * green * blue);

    }

    total

 }


fn main() {
    println!("Solution: {}", solve())
}
