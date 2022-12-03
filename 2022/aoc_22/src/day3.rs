use crate::util;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("3.input");
    let indexing = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut sum = 0;
    for line in data.lines() {
        let vals = line.chars().collect::<Vec<char>>();
        let mid = vals.len() / 2;
        let lower: HashSet<char> = HashSet::from_iter(vals[..mid].iter().cloned());
        let upper: HashSet<char> = HashSet::from_iter(vals[mid..].iter().cloned());

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
    let mut start = 0;
    let mut sum = 0;
    loop {
        if start >= lines.len() {
            break;
        }
        let e1_hash: HashSet<char> = lines[start].chars().collect();
        let e2_hash: HashSet<char> = lines[start+1].chars().collect();
        let e3_hash: HashSet<char> = lines[start+2].chars().collect();

        let intersection: HashSet<_> = &(&e1_hash & &e2_hash) & &e3_hash;
        for i in intersection {
            let p = indexing.find(i).unwrap() + 1;
            sum += p as u64;
        }
        start += 3;
    }

    sum
}

run!();
