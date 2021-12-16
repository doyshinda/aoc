use crate::util;
use std::collections::HashMap;

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
            map.insert(a, b.chars().next().unwrap());
        } else {
            line.chars().for_each(|c| polymer.push(c));
        }
    }

    let mut counts = HashMap::new();
    for _ in 0..10 {
        let mut new_polymer = Vec::new();
        for i in 0..polymer.len() - 1 {
            let pair = format!("{}{}", polymer[i], polymer[i + 1]);
            if let Some(insert) = map.get(pair.as_str()) {
                new_polymer.push(polymer[i]);
                new_polymer.push(*insert);
            }
            *counts.entry(pair).or_insert(0) += 1;
        }
        new_polymer.push(polymer[polymer.len() - 1]);
        polymer = new_polymer;
    }

    let mut m: HashMap<char, u64> = HashMap::new();
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
    0
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}
