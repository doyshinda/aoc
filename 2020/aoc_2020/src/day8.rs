use crate::util;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("8.input");
    let mut seen = HashSet::new();
    let instructions: Vec<(&str, i64)> = data.lines().map(|x| parse_line(x)).collect();
    let mut next: i64 = 0;
    let mut acc = 0;

    loop {
        if seen.contains(&next) {
            break
        }
        seen.insert(next);
        let (i, val) = instructions[next as usize];
        // println!("Executing {} {}", i, val);
        match i {
            "nop" => next += 1,
            "acc" => { acc += val; next += 1 },
            "jmp" => next += val,
            _ => (),
        }
    }
    acc as u64
}

fn part_2() -> u64 {
    let data = util::read_input("8_test.input");
    let instructions: Vec<(&str, i64)> = data.lines().map(|x| parse_line(x)).collect();
    let mut changed_instructions = instructions.clone();
    for (idx, (i, _)) in instructions.iter().enumerate() {
        if *i == "nop" {
            {
                let (c, _) = &mut changed_instructions[idx];
                *c = "jmp";
            }
            let (acc, ok) = run_program(&changed_instructions);
            if ok {
                return acc as u64;
            }
            {
                let (c, _) = &mut changed_instructions[idx];
                *c = "nop";
            }
        }
        if *i == "jmp" {
            {
                let (c, _) = &mut changed_instructions[idx];
                *c = "nop";
            }
            let (acc, ok) = run_program(&changed_instructions);
            if ok {
                return acc as u64;
            }
            {
                let (c, _) = &mut changed_instructions[idx];
                *c = "jmp";
            }
        }
    }
    
    0
}

run!();

fn parse_line(line: &str) -> (&str, i64) {
    let (i, val) = line.split_once(" ").unwrap();
    let val = val.replace('+', "");
    let val = inum!(val);
    return (i, val);
}

fn run_program(instructions: &Vec<(&str, i64)>) -> (i64, bool) {
    let mut next: i64 = 0;
    let mut acc = 0;
    let mut seen = HashSet::new();

    loop {
        if next == instructions.len() as i64 {
            return (acc, true);
        }
        if seen.contains(&next) {
            break
        }
        seen.insert(next);
        let (i, val) = instructions[next as usize];
        // println!("Executing {} {}", i, val);
        match i {
            "nop" => next += 1,
            "acc" => { acc += val; next += 1 },
            "jmp" => next += val,
            _ => (),
        }
    }

    return (acc, false);
}