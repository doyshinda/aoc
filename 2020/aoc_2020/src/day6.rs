use crate::util;
use std::collections::HashMap;

fn part_1() -> u64 {
    let data = util::read_input("6.input");
    data.split("\n\n").map(|group| {
        let mut letters = group
            .lines()
            .fold(vec![], |mut acc, x| {
                for c in x.trim_end().chars() {
                    acc.push(c);
                }
                acc
            });

        letters.sort();
        letters.dedup();
        letters.len() as u64
    }).sum()
}

fn part_2() -> u64 {
    let data = util::read_input("6_test.input");
    data.split("\n\n").map(|group| {
        let mut letters = HashMap::new();
        let mut member_count = 0;
        for line in group.lines() {
            member_count += 1;
            for c in line.trim_end().chars() {
               letters.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
            }
        }
        let mut valid = 0;
        for (_, c) in &letters {
            if *c == member_count {
                valid += 1;
            }
        }
        valid
    }).sum()
}

run!();
