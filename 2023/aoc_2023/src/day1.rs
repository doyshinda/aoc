use crate::util;
use std::collections::HashMap;

fn part_1() -> u64 {
    let data = util::read_input("1.input");
    let vals = data.split("\n");
    let mut sum = 0;
    for e in vals {
        if e.is_empty() {
            continue;
        }

        let c: Vec<_> = e
            .chars()
            .filter_map(|x| {
                match x.is_ascii_digit() {
                    true => Some(x.to_digit(10).unwrap()),
                    _ => None,
                }
            }).collect();
        let mut partial = c[0] * 10;
        partial += c.last().unwrap();
        log!("partial: {}", partial);
        sum += partial;
    }
    sum as u64
}

fn part_2() -> u64 {
    let data = util::read_input("1.input");
    let vals = data.split("\n");
    let mut sum = 0;
    let mapping: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    for line in vals {
        log!("start: {}", line);
        let mut n = vec![];

        let line_len = line.len();
        let all_chars = line.chars().collect::<Vec<char>>();
        for i in 0..line_len {
            let end = std::cmp::min(i+5, line_len);
            if all_chars[i].is_ascii_digit() {
                n.push(all_chars[i].to_digit(10).unwrap());
                continue
            }

            let mut chunk = String::new();
            for c in &all_chars[i..end] {
                chunk.push(*c);
            }

            for (k, v) in &mapping {
                if chunk.starts_with(k) {
                    n.push(*v);
                    break;
                }
            }
        }

        let mut part = n[0] * 10;
        part += n.last().unwrap();
        sum += part;
        log!("n: {:?} -> {}", n, part);
    }
    sum as u64
}

run!();
