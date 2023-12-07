use std::fs::read_to_string;

fn read_input() -> Vec<String> {
    read_to_string("input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn solve() -> usize {
    let mut hands = read_input()
        .iter()
        .map(|line| {
            line.replace("A", "Z")
                .replace("K", "Y")
                .replace("Q", "X")
                .replace("J", "0")
                .replace("T", "V")
        })
        .map(|line| (line[0..5].to_string(), line[6..].parse::<usize>().unwrap()))
        .map(|(hand, bid)| {
            (
                hand.clone(),
                bid,
                hand.chars()
                    .map(|c| (hand.chars().filter(|c1| c == *c1).count(), c))
                    .max()
                    .unwrap(),
            )
        })
        .map(|(hand, bid, max1)| {
            (
                hand.clone(),
                bid,
                max1,
                (hand
                    .chars()
                    .map(|c| {
                        (
                            hand.chars()
                                .filter(|c1| ((c == *c1) && (c != max1.1)))
                                .count(),
                            c,
                        )
                    })
                    .max()
                    .unwrap()),
            )
        })
        .map(|(hand, bid, max1, max2)| {
            (
                hand.clone(),
                match max1.1 {
                    '0' => hand.replace("0", &max2.1.to_string()),
                    c => hand.replace("0", &c.to_string()),
                },
                bid,
            )
        })
        .map(|(hand, hand2, bid)| {
            (
                hand.clone(),
                hand2.clone(),
                bid,
                hand2
                    .chars()
                    .map(|c| (hand2.chars().filter(|c1| c == *c1).count(), c))
                    .max()
                    .unwrap(),
            )
        })
        .map(|(hand, hand2, bid, max1)| {
            (
                max1.0 *10 +
                hand2
                    .chars()
                    .map(|c| {
                            hand2
                                .chars()
                                .filter(|c1| ((c == *c1) && (c != max1.1)))
                                .count()
                        
                    })
                    .max()
                    .unwrap(),
                    hand.clone(),
                    bid
            )
        })
        .collect::<Vec<_>>();

    // Note: we SHOULD be able to collet this into a BinaryHeap her and skip the sort part, but I can't get that to work...

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, (_score, _hand, bid))| (i+1)*bid)
        .sum()
}

fn main() {
    println!("Solution: {}", solve())
}
