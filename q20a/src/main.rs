use regex::Regex;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[derive(Debug, PartialEq)]
enum Type {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug)]
struct Component {
    ctype: Type,
    outputs: Vec<String>,
}

struct Pending {
    source: String,
    target: String,
    input: bool,
}

type Components = HashMap<String, Component>;
type Pendings = LinkedList<Pending>;

fn pulse(
    components: &mut Components,
    pendings: &mut Pendings,
    source: &str,
    target: &str,
    input: bool,
) {
    /* 
    println!(
        "{} -{}-> {}",
        source,
        if input { "high" } else { "low" },
        target
    );
    */
    if let Some(c) = components.get_mut(target) {
        let outputs = c.outputs.clone();

        match &mut c.ctype {
            Type::Broadcast => {
                for o in &outputs {
                    pendings.push_back(Pending {
                        source: target.to_string(),
                        target: o.to_string(),
                        input,
                    });
                }
            }
            Type::FlipFlop(state) => {
                if !input {
                    let input: bool = !*state;
                    *state = input;
                    for o in &outputs {
                        pendings.push_back(Pending {
                            source: target.to_string(),
                            target: o.to_string(),
                            input,
                        });
                    }
                }
            }
            Type::Conjunction(states) => {
                let sourcestate = states.get_mut(source).unwrap();
                *sourcestate = input;

                let output: bool = states.iter().all(|(_, state)| *state);

                for o in &outputs {
                    pendings.push_back(Pending {
                        source: target.to_string(),
                        target: o.to_string(),
                        input: !output,
                    });
                }
            }
        }
    }
}

fn solve() -> i32 {
    let re = Regex::new(r"([a-z]+)(?:,  )?").unwrap();

    let mut components = HashMap::<String, Component>::new();
    let mut conjunction_sources = HashMap::<String, HashMap<String, bool>>::new();

    for line in read_input() {
        let parts = line.split(" -> ").collect::<Vec<_>>();

        let t = match &parts[0][..1] {
            "b" => Type::Broadcast,
            "%" => Type::FlipFlop(false),
            "&" => Type::Conjunction(HashMap::new()),
            _ => panic!("Unexpected type"),
        };

        let name = if Type::Broadcast == t {
            parts[0].to_string()
        } else {
            parts[0][1..].to_string()
        };

        if let Type::Conjunction(_) = t {
            conjunction_sources.insert(name.clone(), HashMap::new());
        }

        components.insert(
            name,
            Component {
                ctype: t,
                outputs: re
                    .captures_iter(parts[1])
                    .map(|c| c.get(1).unwrap().as_str().to_string())
                    .collect::<Vec<_>>(),
            },
        );
    }

    for (cname, component) in components.iter() {
        for o in component.outputs.iter() {
            if let Some(ocomp) = components.get(o) {
                if let Type::Conjunction(_) = &ocomp.ctype {
                    conjunction_sources
                        .get_mut(o)
                        .unwrap()
                        .insert(cname.to_string(), false);
                }
            }
        }
    }

    for (c, sources) in conjunction_sources {
        components.get_mut(&c).unwrap().ctype = Type::Conjunction(sources);
    }

    let mut pendings = Pendings::new();

    let mut low: i32 = 0;
    let mut high: i32 = 0;

    for _ in 0..1000 {

        low += 1;

        pulse(
            &mut components,
            &mut pendings,
            "button",
            "broadcaster",
            false,
        );

        while let Some(p) = pendings.pop_front() {
            if p.input {
                high += 1
            } else {
                low += 1;
            }

            pulse(
                &mut components,
                &mut pendings,
                &p.source,
                &p.target,
                p.input,
            );
        }
    }

    low * high
}

fn main() {
    println!("Solution: {}", solve())
}
