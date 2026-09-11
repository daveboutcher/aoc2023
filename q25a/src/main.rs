use std::collections::{HashMap, HashSet, VecDeque};
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Edge {
    name: String,
    start: String,
    end: String,
}

impl Edge {
    fn reverse(&self) -> Self {
        Self {
            start: self.end.clone(),
            end: self.start.clone(),
            name: self.name.clone(),
        }
    }
}

fn get_input() -> HashMap<String, Vec<Edge>> {
    let mut network: HashMap<String, Vec<Edge>> = HashMap::new();
    let input = fs::read_to_string(Path::new("input.txt"))
        .expect("Something went wrong with the input");

    for line in input.trim().lines() {
        let splits: Vec<_> = line.split(": ").collect();
        let name = splits[0].to_string();
        let childs: Vec<String> = splits[1]
            .split_whitespace()
            .map(|name| name.to_string())
            .collect();

        for child in childs.iter() {
            let edge_name = format!("{}-{}", name, child);
            let edge = Edge {
                start: name.clone(),
                end: child.clone(),
                name: edge_name.clone(),
            };
            let node = network.entry(child.clone()).or_default();
            node.push(edge.reverse());
            let node = network.entry(name.clone()).or_default();
            node.push(edge);
        }
    }

    network
}

type Network = HashMap<String, Vec<Edge>>;

fn get_edge_to_remove(network: &Network) -> Edge {
    let mut betweeness: HashMap<&String, (usize, Edge)> = HashMap::new();

    for node in network.keys() {
        let mut visited: HashSet<&String> = HashSet::new();
        let mut nodes: VecDeque<(&String, Vec<&String>)> = VecDeque::from(vec![(node, vec![])]);
        while let Some((node, path)) = nodes.pop_front() {
            visited.insert(node);
            if let Some(edges) = network.get(node) {
                for edge in edges.iter().filter(|edge| !visited.contains(&edge.end)) {
                    let mut next_path = path.clone();
                    next_path.push(&edge.name);
                    for sub_path in next_path.iter() {
                        betweeness
                            .entry(sub_path)
                            .and_modify(|(x, _)| *x += 1)
                            .or_insert((1, edge.clone()));
                    }
                    nodes.push_back((&edge.end, next_path));
                }
            }
        }
    }

    betweeness
        .values()
        .max_by(|(val_a, _), (val_b, _)| val_a.cmp(val_b))
        .unwrap()
        .1
        .clone()
}

fn get_subsets_size(network: &HashMap<String, Vec<Edge>>) -> (usize, usize) {
    let mut visited: HashSet<&String> = HashSet::new();
    let start = network.keys().next().unwrap();
    let mut to_check = vec![start];

    while let Some(node) = to_check.pop() {
        visited.insert(node);

        if let Some(children) = network.get(node) {
            to_check.extend(
                children
                    .iter()
                    .filter_map(|elem| {
                        if visited.contains(&elem.end) {
                            None
                        } else {
                            Some(&elem.end)
                        }
                    })
                    .collect::<Vec<&String>>(),
            );
        }
    }
    let remaining_size = network.len() - visited.len();
    (visited.len(), remaining_size)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut network = get_input();

    for _ in 0..3 {
        // How did I find it: google find which edge to remove to disconnect graph
        // lead to https://stackoverflow.com/questions/1566967/checking-if-removing-an-edge-in-a-graph-will-result-in-the-graph-splitting
        // lead to https://en.wikipedia.org/wiki/Girvan%E2%80%93Newman_algorithm
        let edge_to_remove = get_edge_to_remove(&network);
        if let Some(edges) = network.get_mut(&edge_to_remove.start) {
            edges.retain(|edge| edge.name != edge_to_remove.name);
        }
        if let Some(edges) = network.get_mut(&edge_to_remove.end) {
            edges.retain(|edge| edge.name != edge_to_remove.name);
        }
    }

    let (size_a, size_b) = get_subsets_size(&network);

    println!(
        "The multiplication of the two subsets size is {}",
        size_a * size_b
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    println!("Only ready when all other stars are here");

    Ok(())
}

fn main() {
    first_star();
}
