use regex::Regex;
use std::cmp;
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
    value: usize,
    nextrule: String,
}

#[derive(Debug, Clone)]
struct MinMax {
    min: usize,
    max: usize,
}

impl MinMax {
    fn new() -> MinMax {
        MinMax { min: 1, max: 4000 }
    }

    fn len(&self) -> usize {
        self.max + 1 - self.min
    }
}

#[derive(Debug, Clone)]
struct PartRange {
    x: MinMax,
    m: MinMax,
    a: MinMax,
    s: MinMax,
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            x: MinMax::new(),
            m: MinMax::new(),
            a: MinMax::new(),
            s: MinMax::new(),
        }
    }

    fn get_category(&mut self, category: &Category) -> &mut MinMax {
        match category {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }

    fn total(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

fn add_rule(rules: &mut Rules, line: &str) {
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
            value: value.parse::<usize>().unwrap(),
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

fn num_rules(rules: &Rules, rule: &str, mut range: PartRange) -> usize {
    let rulevec = rules.get(rule).unwrap();

    rulevec
        .iter()
        .map(|r| match r.cond {
            Cond::Always => match r.nextrule.as_str() {
                "R" => 0,
                "A" => range.total(),
                nextrule => num_rules(rules, nextrule, range.clone()),
            },
            Cond::Less => {
                let mut matchrange = range.clone();
                matchrange.get_category(&r.category).max =
                    cmp::min(r.value - 1, matchrange.get_category(&r.category).max);
                range.get_category(&r.category).min =
                    cmp::max(r.value, range.get_category(&r.category).min);

                match r.nextrule.as_str() {
                    "R" => 0,
                    "A" => matchrange.total(),
                    nextrule => num_rules(rules, nextrule, matchrange),
                }
            }
            Cond::Greater => {
                let mut matchrange = range.clone();
                matchrange.get_category(&r.category).min =
                    cmp::max(r.value + 1, matchrange.get_category(&r.category).min);
                range.get_category(&r.category).max =
                    cmp::min(r.value, range.get_category(&r.category).max);

                match r.nextrule.as_str() {
                    "R" => 0,
                    "A" => matchrange.total(),
                    nextrule => num_rules(rules, nextrule, matchrange),
                }
            }
        })
        .sum()
}

fn solve() -> usize {
    read_input()
        .iter()
        .scan(Rules::new(), |rules, line| {
            if line.is_empty() {
                Some(num_rules(rules, "in", PartRange::new()))
            } else if line[..1] == *"{" {
                Some(0)
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
