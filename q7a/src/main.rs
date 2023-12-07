use std::collections::BinaryHeap;
use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    read_input()
        .iter()
        // Update the face card names to sort correctly
        .map(|line| {
            line.replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "W")
                .replace("T", "V")
        })
        // Break the line into the hand and the bid
        .map(|line| (line[0..5].to_string(), line[6..].parse::<usize>().unwrap()))
        // Count the most frequently occurring card
        .map(|(hand, bid)| {
            (
                hand.chars()
                    .map(|c| (hand.chars().filter(|c1| c == *c1).count(), c))
                    .max()
                    .unwrap(),
                hand,
                bid,
            )
        })
        // Count the second most frequently occurring card and produce a score
        // so 5 of a kind will be 50, 3 of a kind will be 30, full house will be
        // 32, two pair will be 22
        .map(|(max1, hand, bid)| {
            (
                (max1.0 * 10
                    + hand
                        .chars()
                        .map(|c| {
                            hand.chars()
                                .filter(|c1| ((c == *c1) && (c != max1.1)))
                                .count()
                        })
                        .max()
                        .unwrap()),
                hand,
                bid,
            )
        })
        // Put it in a binary heap so it sorts by score then hand
        .collect::<BinaryHeap<_>>()
        // Note that BinaryYeap.iter() does NOT do what you would expect...
        .into_sorted_vec()
        .iter()
        .enumerate()
        .map(|(i, (_score, _hand, bid))| (i + 1) * bid)
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
