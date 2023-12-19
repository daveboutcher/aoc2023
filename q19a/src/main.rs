use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::FromStr;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

type Rules = HashMap<String, Vec<Rule>>;

enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(input: &str) -> Result<Category, Self::Err> {
        match input {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err(()),
        }
    }
}

enum Cond {
    Less,
    Greater,
    Always,
}

impl FromStr for Cond {
    type Err = ();

    fn from_str(input: &str) -> Result<Cond, Self::Err> {
        match input {
            "<" => Ok(Cond::Less),
            ">" => Ok(Cond::Greater),
            _ => Err(()),
        }
    }
}

struct Rule {
    category: Category,
    cond: Cond,
    value: i32,
    nextrule: String,
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn get_category(&self, category: &Category) -> i32 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn total(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

fn add_rule(rules: &mut Rules, line: &String) {
    let re = Regex::new(r"^(?<name>[a-z]+)\{(?<rules>.*),(?<final>[a-z]+|[AR])\}$").unwrap();
    let re2 =
        Regex::new("(?<category>[xmas])(?<condition>[<>])(?<value>[0-9]+):(?<nextrule>[a-z]+|A|R)")
            .unwrap();

    let (_, [name, conds, finalrule]) = re.captures(line).unwrap().extract();

    let mut r: Vec<Rule> = Vec::new();

    for c in re2.captures_iter(conds) {
        let (_, [category, cond, value, nextrule]) = c.extract();

        r.push(Rule {
            category: Category::from_str(category).unwrap(),
            cond: Cond::from_str(cond).unwrap(),
            value: value.parse::<i32>().unwrap(),
            nextrule: nextrule.to_string(),
        });
    }
    r.push(Rule {
        category: Category::X,
        cond: Cond::Always,
        value: 0,
        nextrule: finalrule.to_string(),
    });
    rules.insert(name.to_string(), r);
}

fn parse_part(line: &str) -> Part {
    let re =
        Regex::new(r"^\{x=(?<x>[0-9]+),m=(?<m>[0-9]+),a=(?<a>[0-9]+),s=(?<s>[0-9]+)\}").unwrap();

    let (_, [x, m, a, s]) = re.captures(line).unwrap().extract();

    Part {
        x: x.parse::<i32>().unwrap(),
        m: m.parse::<i32>().unwrap(),
        a: a.parse::<i32>().unwrap(),
        s: s.parse::<i32>().unwrap(),
    }
}

fn process_part(rules: &Rules, rule: &str, part: Part) -> i32 {
    let rulevec = rules.get(rule).unwrap();

    if let Some(r) = rulevec.iter().find(|r| match r.cond {
        Cond::Always => true,
        Cond::Less => part.get_category(&r.category) < r.value,
        Cond::Greater => part.get_category(&r.category) > r.value,
    }) {
        match r.nextrule.as_str() {
            "A" => part.total(),
            "R" => 0,
            nextrule => process_part(rules, nextrule, part),
        }
    } else {
        panic!("Didn't find any matching rule, which should never happen");
    }
}

fn solve() -> i32 {
    read_input()
        .iter()
        .scan(Rules::new(), |rules, line| {
            if line.is_empty() {
                Some(0)
            } else if line[..1] == *"{" {
                Some(process_part(rules, "in", parse_part(line)))
            } else {
                add_rule(rules, line);
                Some(0)
            }
        })
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
