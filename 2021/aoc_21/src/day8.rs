use crate::util;
use std::collections::{HashMap, HashSet};

fn part_1() -> usize {
    util::read_input("8_a")
        .split('\n')
        .map(|x| {
            x.split(" | ")
                .skip(1)
                .map(|y| {
                    y.split(' ')
                        .filter(|v| {
                            let vlen = v.len();
                            vlen == 2 || vlen == 3 || vlen == 4 || vlen == 7
                        })
                        .count()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

/*
Rules

Assuming the following signal mapping:
     0000
    1    2
    1    2
     3333
    4    5
    4    5
     6666

1. We can determine which character goes in position 0 by doing set subtraction on the 2 and 3
   length input signals

2. We can determine which character goes in positions 1, 4, 5 by counting all occurrences of
   characters:
    position 1 appears in exactly 6 signals
    position 4 appears in exactly 4 signals
    position 5 appears in exactly 9 signals

3. We can determine which character goes in position 2 by taking the signal for 7 and replacing
   0 & 5 to isolate 2

4. We can now filter out known characters from the number 4 (the only one with 4 signals) to find
   position 3

5. This leaves only position 6, the only unknown char in the number 8
*/
fn part_2() -> usize {
    let mut total = 0;
    for line in util::read_input("8_a").split('\n') {
        let mut signal_map = HashMap::new();
        let mut char_map = HashMap::new();
        let mut char_counts = HashMap::new();

        let parts: Vec<&str> = line.split(" | ").collect();
        for p in parts[0].split(' ') {
            let entry = signal_map.entry(p.len()).or_insert(Vec::new());
            entry.push(p);
            for c in p.chars() {
                let entry = char_counts.entry(c).or_insert(0);
                *entry += 1;
            }
        }

        let two = signal_map.get(&2).unwrap();
        let three = signal_map.get(&3).unwrap();
        let four = signal_map.get(&4).unwrap();
        let seven = signal_map.get(&7).unwrap();

        // Rule 1
        let a: HashSet<_> = three[0].chars().collect();
        let b: HashSet<_> = two[0].chars().collect();
        for x in a.difference(&b) {
            char_map.insert(x.clone(), 0);
        }

        // Rule 2
        for (c, v) in &char_counts {
            if *v == 9 {
                char_map.insert(*c, 5);
            }
            if *v == 4 {
                char_map.insert(*c, 4);
            }
            if *v == 6 {
                char_map.insert(*c, 1);
            }
        }

        // Rule 3
        let pos = three[0]
            .chars()
            .find(|x| char_map.get(x).is_none())
            .unwrap();
        char_map.insert(pos, 2);

        // Rule 4
        let pos = four[0].chars().find(|x| char_map.get(x).is_none()).unwrap();
        char_map.insert(pos, 3);

        // Rule 5
        let pos = seven[0]
            .chars()
            .find(|x| char_map.get(x).is_none())
            .unwrap();
        char_map.insert(pos, 6);

        let mut sig_tot = 0;
        let mut mp = 1000;
        for number in parts[1].split(' ') {
            let mut chs: Vec<i32> = number.chars().map(|c| *char_map.get(&c).unwrap()).collect();
            chs.sort();
            sig_tot += num_from_vector(chs) * mp;
            mp /= 10;
        }

        total += sig_tot;
    }

    total as usize
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}

fn num_from_vector(v: Vec<i32>) -> i32 {
    match v.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        5 => {
            let t = (v[0], v[1], v[2], v[3], v[4]);
            return match t {
                (0, 2, 3, 5, 6) => 3,
                (0, 1, 3, 5, 6) => 5,
                (0, 2, 3, 4, 6) => 2,
                _ => unreachable!(),
            };
        }
        6 => {
            let t = (v[0], v[1], v[2], v[3], v[4], v[5]);
            return match t {
                (0, 1, 2, 4, 5, 6) => 0,
                (0, 1, 3, 4, 5, 6) => 6,
                (0, 1, 2, 3, 5, 6) => 9,
                _ => unreachable!(),
            };
        }
        _ => 0,
    }
}

#[test]
fn test_num_from_vector() {
    let zero = vec![0, 1, 2, 4, 5, 6];
    assert_eq!(num_from_vector(zero), 0);

    let one = vec![2, 5];
    assert_eq!(num_from_vector(one), 1);

    let two = vec![0, 2, 3, 4, 6];
    assert_eq!(num_from_vector(two), 2);

    let five = vec![0, 2, 3, 5, 6];
    assert_eq!(num_from_vector(five), 3);

    let three = vec![0, 1, 3, 5, 6];
    assert_eq!(num_from_vector(three), 5);
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(), 264);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(), 1063760);
}
