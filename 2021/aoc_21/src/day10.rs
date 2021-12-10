use crate::util;

fn part_1() -> u32 {
    let mut points = 0;
    'outer: for line in util::read_input("10_a").split('\n') {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<' => stack.push(c),
                ']' | ')' | '}' | '>' => {
                    let prev = stack.pop();
                    if prev.is_none() {
                        continue 'outer;
                    }

                    match (prev.unwrap(), c) {
                        ('[', ']') | ('(', ')') | ('{', '}') | ('<', '>') => (),
                        _ => points += calc_points(c),
                    }
                }
                _ => (),
            }
        }
    }

    points
}

fn calc_points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn part_2() -> u64 {
    let mut points = Vec::new();
    'outer: for line in util::read_input("10_a").split('\n') {
        let mut stack = Vec::new();
        for c in line.chars() {
            match c {
                '[' | '(' | '{' | '<' => stack.push(c),
                ']' | ')' | '}' | '>' => {
                    let prev = stack.pop();
                    if prev.is_none() {
                        continue 'outer;
                    }

                    match (prev.unwrap(), c) {
                        ('[', ']') | ('(', ')') | ('{', '}') | ('<', '>') => (),
                        _ => continue 'outer,
                    }
                }
                _ => (),
            }
        }

        if stack.len() > 0 {
            points.push(calc_score(&stack));
        }
    }

    points.sort();
    points[points.len() / 2]
}

fn calc_points_2(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn calc_score(stack: &Vec<char>) -> u64 {
    stack.iter().rev().fold(0, |mut acc, c| {
        acc = (acc * 5) + calc_points_2(*c);
        acc
    })
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}
