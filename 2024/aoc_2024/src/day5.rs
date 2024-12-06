use crate::util;
use std::collections::HashMap;

fn valid(rule_dict: &HashMap<u64, Vec<u64>>, vals: &Vec<u64>) -> bool {
    for (i, v) in vals.iter().enumerate() {
        match rule_dict.get(v) {
            Some(deps) => {
                for d in deps {
                    match vals.iter().position(|&r| r == *d) {
                        Some(idx) => if idx < i {
                            return false;
                        }
                        _ => (),
                    }
                }
            },
            _ => continue,
        }
    }

    true
}

fn part_1() -> u64 {
    let data = util::read_input("5.input");
    let mut ans = 0;

    let (rules, ordering) = data.split_once("\n\n").expect("invalid input");
    hm!(rule_dict, u64, Vec<u64>);
    for r in rules.split("\n") {
        let (key, dependent) = r.split_once("|").expect("invalid rule");
        let key = unum!(key);
        let dependent = unum!(dependent);
        rule_dict.entry(key).and_modify(|v| v.push(dependent)).or_insert(vec![dependent]);
    }

    for order in ordering.split("\n") {
        let vals: Vec<u64> = order.split(",").map(|o| unum!(o)).collect();

        if valid(&rule_dict, &vals) {
            assert!(vals.len() % 2 == 1);
            ans += vals[vals.len() / 2];
        }
    }

    ans
}

fn part_2() -> u64 {
    let data = util::read_input("5.input");
    let mut ans = 0;

    let (rules, ordering) = data.split_once("\n\n").expect("invalid input");
    hm!(rule_dict, u64, Vec<u64>);
    for r in rules.split("\n") {
        let (key, dependent) = r.split_once("|").expect("invalid rule");
        let key = unum!(key);
        let dependent = unum!(dependent);
        rule_dict.entry(key).and_modify(|v| v.push(dependent)).or_insert(vec![dependent]);
    }

    let compare = |a: &u64, b: &u64| -> std::cmp::Ordering {
        match rule_dict.get(a) {
            Some(deps) => {
                if deps.contains(b) {
                    return std::cmp::Ordering::Less;
                }
            }
            _ => (),
        }

        return std::cmp::Ordering::Equal;
    };

    for order in ordering.split("\n") {
        let mut vals: Vec<u64> = order.split(",").map(|o| unum!(o)).collect();
        if !valid(&rule_dict, &vals) {
            vals.sort_by(compare);
            assert!(vals.len() % 2 == 1);
            ans += vals[vals.len() / 2];
        }
    }

    ans
}

run!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}

