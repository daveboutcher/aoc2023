use std::collections::HashMap;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn getcount(
    cache: &mut HashMap<(usize, usize, usize), usize>,
    cache_stats: &mut (usize, usize),
    line: &str,
    counts: &[usize],
    hash_match: usize,
) -> usize {
    cache_stats.0 += 1;
    if let Some(result) = cache.get(&(line.len(), hash_match, counts.len())) {
        cache_stats.1 += 1;
        *result
    } else {
        let chars = line.as_bytes();

        let result = if chars.is_empty() {
            if counts.is_empty() {
                1
            } else {
                0
            }
        } else if chars[0] == b'#' {
            getcount(cache, cache_stats, &line[1..], counts, hash_match + 1)
        } else if chars[0] == b'.' || counts.is_empty() {
            if !counts.is_empty() && hash_match == counts[0] {
                getcount(cache, cache_stats, &line[1..], &counts[1..], 0)
            } else if hash_match == 0 {
                getcount(cache, cache_stats, &line[1..], counts, 0)
            } else {
                0
            }
        } else {
            getcount(cache, cache_stats, &line[1..], counts, hash_match + 1)
                + if hash_match == counts[0] {
                    getcount(cache, cache_stats, &line[1..], &counts[1..], 0)
                } else if hash_match == 0 {
                    getcount(cache, cache_stats, &line[1..], counts, 0)
                } else {
                    0
                }
        };

        cache.insert((line.len(), hash_match, counts.len()), result);

        result
    }
}

fn solve() -> (usize, (usize, usize)) {
    let mut cache_stats = (0usize, 0usize);

    (read_input()
        .iter()
        .map(|l| l.split(' ').collect::<Vec<_>>())
        .map(|l| (l[0].to_string() + ".", l[1]))
        .map(|(s, r)| (s, r))
        .map(|(s, r)| {
            (
                s,
                r.split(',')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(line, counts)| getcount(&mut HashMap::new(), &mut cache_stats, &line[..], &counts[..], 0))
        .sum(),
        cache_stats)
}

fn main() {
    let solution = solve();
    println!("Solution: {} cache {}/{}", solution.0, solution.1.1, solution.1.0);
}
