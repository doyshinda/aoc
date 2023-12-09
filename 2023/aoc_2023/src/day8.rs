use crate::util;
use std::collections::HashMap;

fn part_1() -> u64 {
    let data = util::read_input("8.input");
    let (steps, node_lines) = data.split_once("\n\n").unwrap();
    let mut steps = steps.chars().cycle();
    hm!(map, String, (String, String));

    for line in node_lines.split('\n') {
        if line.is_empty() {
            continue;
        }

        let (key, values) = line.split_once(" = ").unwrap();
        let values = values.replace("(", "");
        let values = values.replace(")", "");
        let (left, right) = values.split_once(", ").unwrap();
        map.insert(key.to_string(), (left.to_string(), right.to_string()));
    }

    let mut curr = &"AAA".to_string();
    let mut step_count = 0;
    while curr != "ZZZ" {
        let next_step = steps.next().unwrap();
        curr = match next_step {
            'R' => &map[curr].1,
            'L' => &map[curr].0,
            _ => todo!(),
        };
        step_count += 1;
    }
    step_count
}

#[derive(Debug)]
struct Node<'a> {
    curr: String,
    step_count: u64,
    last_found: u64,
    last_diff: u64,
    step_diff: u64,
    map: &'a HashMap<String, (String, String)>,
}

impl<'a> Node<'a> {
    fn advance(&mut self, next_step: char) -> bool {
        if self.step_diff != 0 {
            self.step_count -= 1;
            if self.step_count == 0 {
                self.step_count = self.step_diff;
                return true;
            }
            return false;
        }
        self.curr = match next_step {
            'R' => self.map[&self.curr].1.to_string(),
            'L' => self.map[&self.curr].0.to_string(),
            _ => todo!(),
        };

        self.step_count += 1;

        let found = self.curr.ends_with("Z");
        if found {
            if self.last_found > 0 {
                let diff = self.step_count - self.last_found;
                if self.last_diff == diff {
                    self.step_diff = diff;
                    self.step_count = diff;
                }
                self.last_diff = diff;
            }
            self.last_found = self.step_count
        }

        found
    }
}

fn part_2() -> u64 {
    let data = util::read_input("8.input");
    let (steps, node_lines) = data.split_once("\n\n").unwrap();
    let mut steps = steps.chars().cycle();
    hm!(map, String, (String, String));

    let mut start_nodes = vec![];

    for line in node_lines.split('\n') {
        if line.is_empty() {
            continue;
        }

        let (key, values) = line.split_once(" = ").unwrap();
        let values = values.replace("(", "");
        let values = values.replace(")", "");
        let (left, right) = values.split_once(", ").unwrap();
        map.insert(key.to_string(), (left.to_string(), right.to_string()));
        if key.ends_with("A") {
            start_nodes.push(key.to_string())
        }
    }

    let mut nodes = vec![];
    for n in start_nodes {
        nodes.push(Node {
            step_count: 0,
            last_found: 0,
            last_diff: 0,
            step_diff: 0,
            curr: n,
            map: &map,
        });
    }

    let mut done = vec![];
    for _ in &nodes {
        done.push(false);
    }

    let mut common = vec![];
    while !done.iter().all(|x| *x) {
        let next_step = steps.next().unwrap();
        let mut idx = 0;
        for n in &mut nodes {
            n.advance(next_step);
            if n.step_diff > 0 {
                done[idx] = true;
                common.push(n.step_diff);
            }

            idx += 1;
        }
    }

    util::lcm_vals(&common)
}

run!();

#[cfg(test)]
mod tests {
    use crate::util::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(20, 30), 10);
    }

    #[test]
    fn test_gcd_vec() {
        let vals = vec![42, 120, 285];
        assert_eq!(gcd_vals(&vals), 3);
    }

    #[test]
    fn test_lcm() {
        let a = 19199;
        let b = 20777;
        assert_eq!(lcm(a, b), 1516721);
    }

    #[test]
    fn test_lcm_vec() {
        let vals = vec![4, 6, 21];
        assert_eq!(lcm_vals(&vals), 84);
    }
}
