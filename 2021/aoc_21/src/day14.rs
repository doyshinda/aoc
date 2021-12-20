use crate::util;
use std::collections::HashMap;
use indexmap::{IndexMap};

fn part_1() -> u64 {
    let mut map = HashMap::new();
    let mut polymer = Vec::new();
    let data = util::read_input("14_a");
    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }
        if line.contains(" -> ") {
            let (a, b) = line.split_once(" -> ").unwrap();
            let mut a_iter = a.chars();
            let k = (a_iter.next().unwrap(), a_iter.next().unwrap());
            map.insert(k, b.chars().next().unwrap());
        } else {
            line.chars().for_each(|c| polymer.push(c));
        }
    }

    for _step in 0..10 {
        let len = polymer.len();
        let mut new_polymer = Vec::with_capacity(len * 2);
        for i in 0..polymer.len() - 1 {
            let pair = (polymer[i], polymer[i + 1]);
            if let Some(insert) = map.get(&pair) {
                new_polymer.push(polymer[i]);
                new_polymer.push(*insert);
            }
        }
        new_polymer.push(polymer[len - 1]);
        polymer = new_polymer;
    }

    let mut m: IndexMap<char, u64> = IndexMap::new();
    for x in polymer {
        *m.entry(x).or_default() += 1;
    }

    let mut min = u64::MAX;
    let mut max = 0;
    for v in m.values() {
        if *v > max {
            max = *v;
        }

        if *v < min {
            min = *v;
        }
    }

    max - min
}

fn part_2() -> u64 {
    let mut map = HashMap::new();
    let mut polymer = Vec::new();
    let data = util::read_input("14_a");
    for line in data.lines() {
        if line.len() == 0 {
            continue;
        }
        if line.contains(" -> ") {
            let (a, b) = line.split_once(" -> ").unwrap();
            let mut a_iter = a.chars();
            let k = (a_iter.next().unwrap(), a_iter.next().unwrap());
            map.insert(k, b.chars().next().unwrap());
        } else {
            line.chars().for_each(|c| polymer.push(c));
        }
    }

    let mut counts: IndexMap<(char, char), u64> = IndexMap::new();
    let mut m: IndexMap<char, u64> = IndexMap::new();

    for i in 0..polymer.len() - 1 {
        let pair = (polymer[i], polymer[i + 1]);
        *counts.entry(pair).or_default() += 1;
    }

    for _step in 0..40 {
        let mut new_counts: IndexMap<(char, char), u64> = IndexMap::new();
        for (pair, count) in counts {
            if let Some(insert) = map.get(&pair) {
                let new_pair = (pair.0, *insert);
                *new_counts.entry(new_pair).or_default() += count;
                let new_pair = (*insert, pair.1);
                *new_counts.entry(new_pair).or_default() += count;
            }
        }
        counts = new_counts;
    }

    for (pair, count) in counts {
        *m.entry(pair.0).or_default() += count;
    }
    let last = polymer[polymer.len() - 1];
    *m.entry(last).or_default() += 1;

    let mut min = u64::MAX;
    let mut max = 0;
    for (_, v) in &m {
        if *v > max {
            max = *v;
        }

        if *v < min {
            min = *v;
        }
    }

    max - min
}

run!();
