use crate::util;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("3.input");
    let indexing = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    for line in data.lines() {
        let vals = line.chars().collect::<Vec<char>>();
        let mid = vals.len() / 2;
        hs!(lower, char, vals[..mid]);
        hs!(upper, char, vals[mid..]);

        for x in lower.intersection(&upper) {
            let p = indexing.find(*x).unwrap() + 1;
            sum += p as u64;
        }
    }
    sum
}

fn part_2() -> u64 {
    let data = util::read_input("3.input");
    let indexing = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let lines = data.lines().collect::<Vec<&str>>();
    lines
        .chunks(3)
        .map(|chunk| {
            let e1_hash: HashSet<char> = chunk[0].chars().collect();
            let e2_hash: HashSet<char> = chunk[1].chars().collect();
            let e3_hash: HashSet<char> = chunk[2].chars().collect();

            let intersection: HashSet<_> = &(&e1_hash & &e2_hash) & &e3_hash;
            intersection
                .iter()
                .map(|i| (indexing.find(*i).unwrap() + 1) as u64)
                .sum::<u64>()
        })
        .sum()
}

run!();
