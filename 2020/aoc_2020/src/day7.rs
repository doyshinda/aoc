use crate::util;
use std::collections::{HashMap, HashSet};

fn part_1() -> u64 {
    let data = util::read_input("7.input");
    let mut bag_map = HashMap::new();
    for line in data.lines() {
        update_map(&mut bag_map, line.trim_end());
    }

    let mut counts = HashSet::new();
    let mut to_check = vec!["shiny gold".to_string()];
    while !to_check.is_empty() {
        let curr = to_check.pop().unwrap();
        for (n, contains) in &bag_map {
            if *n == curr {
                continue
            }

            if contains.contains(&curr) {
                counts.insert(n.clone());
                to_check.push(n.clone());
                continue
            }
        }
    }
    counts.len() as u64
}

fn part_2() -> u64 {
    let data = util::read_input("7.input");
    let mut bag_map = HashMap::new();
    for line in data.lines() {
        update_map2(&mut bag_map, line.trim_end());
    }

    recurse(&bag_map, "shiny gold")
}

run!();

fn recurse(bag_map: &HashMap<String, HashMap<String, u64>>, v: &str) -> u64 {
    let mut count = 0;
    match bag_map.get(v) {
        Some(sub_bags) => {
            for (n, i) in sub_bags {
                count += i;
                count += i * recurse(bag_map, n);
            }
        }
        _ => (),
    }
    count
}

fn update_map(map: &mut HashMap<String, HashSet<String>>, rule: &str) {
    let (first, rest) = rule.split_once(" contain ").unwrap();
    if rest == "no other bags." {
        return;
    }
    let curr_bag = first.replace(" bags", "");
    let other_bags = parse_rest_bags(rest);
    map.insert(curr_bag, other_bags);
}

fn parse_rest_bags(rest: &str) -> HashSet<String> {
    let mut result = HashSet::new();
    let vals = rest.split(",").collect::<Vec<&str>>();

    for v in vals {
        let trimmed = v.trim_end_matches('.')
            .trim_end_matches(',')
            .trim_end_matches(" bags")
            .trim_end_matches(" bag")
            .trim_start();
        let vals = trimmed.split_ascii_whitespace().collect::<Vec<&str>>();
        result.insert(format!{"{} {}", vals[1], vals[2]});
    }

    result
}

fn update_map2(map: &mut HashMap<String, HashMap<String, u64>>, rule: &str) {
    let (first, rest) = rule.split_once(" contain ").unwrap();
    if rest == "no other bags." {
        return;
    }
    let curr_bag = first.replace(" bags", "");
    let other_bags = parse_rest_bags2(rest);
    map.insert(curr_bag, other_bags);
}

fn parse_rest_bags2(rest: &str) -> HashMap<String, u64> {
    let mut result = HashMap::new();
    let vals = rest.split(",").collect::<Vec<&str>>();

    for v in vals {
        let trimmed = v.trim_end_matches('.')
            .trim_end_matches(',')
            .trim_end_matches(" bags")
            .trim_end_matches(" bag")
            .trim_start();
        let vals = trimmed.split_ascii_whitespace().collect::<Vec<&str>>();
        let n = vals[0];
        result.insert(format!{"{} {}", vals[1], vals[2]}, unum!(n));
    }

    result
}
