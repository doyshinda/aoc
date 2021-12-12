use crate::util;
use std::collections::{HashMap, HashSet};

const START: &'static str = "start";
const END: &'static str = "end";

trait Set: Clone {
    fn valid(&mut self, c: &String) -> bool;
    fn insert(&mut self, c: String);
}

#[derive(Debug, Clone)]
struct CC {
    counts: HashMap<String, u32>,
    valid: u32,
}

impl Set for CC {
    fn insert(&mut self, c: String) {
        let entry = self.counts.entry(c).or_insert(0);
        *entry += 1;
        if *entry > 1 {
            self.valid = 1;
        }
    }

    fn valid(&mut self, c: &String) -> bool {
        let entry = self.counts.get(c);
        if entry.is_none() {
            return true;
        }

        *entry.unwrap() < self.valid
    }
}

impl Set for HashSet<String> {
    fn insert(&mut self, c: String) {
        self.insert(c);
    }

    fn valid(&mut self, c: &String) -> bool {
        !self.contains(c)
    }
}

fn find_unique_paths(
    start: &String,
    map: &HashMap<String, Vec<String>>,
    mut seen: impl Set,
) -> u32 {
    if start == END {
        return 1;
    }

    if !seen.valid(start) {
        return 0;
    }

    if start.as_bytes()[0] > b'Z' {
        seen.insert(start.clone());
    }

    match map.get(start) {
        None => return 0,
        Some(paths) => paths
            .iter()
            .fold(0, |acc, p| acc + find_unique_paths(p, &map, seen.clone())),
    }
}

fn part_1(map: &HashMap<String, Vec<String>>) -> u32 {
    let mut unique_paths = 0;
    for p in map.get(START).unwrap() {
        let seen = HashSet::new();
        unique_paths += find_unique_paths(p, &map, seen);
    }

    unique_paths
}

fn part_2(map: &HashMap<String, Vec<String>>) -> u32 {
    let mut unique_paths = 0;
    for p in map.get(START).unwrap() {
        let seen = CC {
            counts: HashMap::new(),
            valid: 2,
        };
        unique_paths += find_unique_paths(p, &map, seen);
    }

    unique_paths
}

pub fn run() {
    let map = parse_input("12_a");
    let start = std::time::Instant::now();
    let answer = part_1(&map);
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2(&map);
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}

fn parse_input(name: &str) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    util::read_input(name).split('\n').for_each(|line| {
        let (a, b) = line.split_once('-').unwrap();
        if b != START && a != END {
            map.entry(a.to_string()).or_default().push(b.to_string());
        }

        if a != START && b != END {
            map.entry(b.to_string()).or_default().push(a.to_string());
        }
    });

    map
}
