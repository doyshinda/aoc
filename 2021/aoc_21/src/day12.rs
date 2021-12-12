use crate::util;
use std::collections::{HashMap, HashSet};

const START: &'static str = "start";
const END: &'static str = "end";

fn find_unique_paths(
    start: &String,
    map: &HashMap<String, Vec<String>>,
    mut seen: HashSet<String>,
) -> u32 {
    if start == END {
        return 1;
    }

    if seen.contains(start) {
        return 0;
    }

    if *start == start.clone().to_ascii_lowercase() {
        seen.insert(start.clone());
    }

    let paths = map.get(start);
    if paths.is_none() {
        return 0;
    }
    let paths = paths.unwrap();
    if paths.len() == 0 {
        return 0;
    }

    let mut unique_paths = 0;
    for p in paths {
        unique_paths += find_unique_paths(p, &map, seen.clone());
    }

    return unique_paths;
}

fn part_1() -> u32 {
    let map = parse_input("12_a");
    let mut unique_paths = 0;
    for p in map.get(&"start".to_string()).unwrap() {
        let seen = HashSet::new();
        unique_paths += find_unique_paths(p, &map, seen);
    }

    unique_paths
}

fn part_2() -> u32 {
    let map = parse_input("12_a");
    let mut unique_paths = 0;
    for p in map.get(&"start".to_string()).unwrap() {
        let seen = CC {
            counts: HashMap::new(),
            valid: 2,
        };
        unique_paths += find_unique_paths2(p, &map, seen);
    }

    unique_paths
}

#[derive(Debug, Clone)]
struct CC {
    counts: HashMap<String, u32>,
    valid: u32,
}

impl CC {
    fn valid(&mut self, c: &String) -> bool {
        let entry = self.counts.get(c);
        if entry.is_none() {
            return true;
        }

        *entry.unwrap() < self.valid
    }

    fn insert(&mut self, c: String) {
        let entry = self.counts.entry(c).or_insert(0);
        *entry += 1;
        if *entry > 1 {
            self.valid = 1;
        }
    }
}

fn find_unique_paths2(start: &String, map: &HashMap<String, Vec<String>>, mut seen: CC) -> u32 {
    if start == END {
        return 1;
    }

    if !seen.valid(start) {
        return 0;
    }

    if *start == start.clone().to_ascii_lowercase() {
        seen.insert(start.clone());
    }

    let paths = map.get(start);
    if paths.is_none() {
        return 0;
    }
    let paths = paths.unwrap();
    if paths.len() == 0 {
        return 0;
    }

    let mut unique_paths = 0;
    for p in paths {
        unique_paths += find_unique_paths2(p, &map, seen.clone());
    }

    return unique_paths;
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}

fn parse_input(name: &str) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();
    util::read_input(name).split('\n').for_each(|line| {
        let mut parts = line.split('-');
        let a = parts.next().unwrap().to_string();
        let b = parts.next().unwrap().to_string();
        if b != START && a != END {
            let entry = map.entry(a.clone()).or_insert(Vec::new());
            entry.push(b.clone());
        }

        if a != START && b != END {
            let entry = map.entry(b).or_insert(Vec::new());
            entry.push(a);
        }
    });

    map
}
