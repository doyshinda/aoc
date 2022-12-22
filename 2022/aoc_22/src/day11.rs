use crate::util;

#[derive(Debug, PartialEq, Eq)]
enum OpType {
    Old,
    Val(u128),
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    worry_level: u128,
}

struct Monkey {
    op: fn(old: u128, v: u128) -> u128,
    op_type: OpType,
    items: Vec<Item>,
    test: u128,
    true_monkey: u128,
    false_monkey: u128,
}

fn part_1() -> u128 {
    let data = util::read_input("11.input");
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in data.split("\n\n") {
        monkeys.push(parse_monkey(monkey));
    }

    hm!(counts, usize, usize);
    for _ in 0..20 {
        for mdx in 0..monkeys.len() {
            let m = &mut monkeys[mdx];
            let mut to_add = Vec::new();
            hm_inc!(counts, mdx, m.items.len());
            for mut i in m.items.drain(..) {
                match m.op_type {
                    OpType::Old => i.worry_level = (m.op)(i.worry_level, i.worry_level),
                    OpType::Val(v) => i.worry_level = (m.op)(i.worry_level, v),
                }
                i.worry_level /= 3;

                if i.worry_level % m.test == 0 {
                    to_add.push((m.true_monkey as usize, i));
                } else {
                    to_add.push((m.false_monkey as usize, i));
                }
            }

            for (idx, i) in to_add {
                monkeys[idx].items.push(i)
            }
        }
    }

    let mut s: Vec<usize> = counts.into_values().collect();
    s.sort();
    (s[s.len() - 1] * s[s.len() - 2]) as u128
}

fn part_2() -> u128 {
    let data = util::read_input("11.input");

    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey in data.split("\n\n") {
        monkeys.push(parse_monkey(monkey));
    }

    let magic_number = get_magic_number(&monkeys);
    hm!(counts, usize, usize);
    for _ in 0..10000 {
        for mdx in 0..monkeys.len() {
            let m = &mut monkeys[mdx];
            let mut to_add = Vec::new();
            hm_inc!(counts, mdx, m.items.len());
            for mut i in m.items.drain(..) {
                match m.op_type {
                    OpType::Old => i.worry_level = (m.op)(i.worry_level, i.worry_level),
                    OpType::Val(v) => i.worry_level = (m.op)(i.worry_level, v),
                }

                if i.worry_level % m.test == 0 {
                    i.worry_level = i.worry_level % magic_number;
                    to_add.push((m.true_monkey as usize, i));
                } else {
                    i.worry_level = i.worry_level % magic_number;
                    to_add.push((m.false_monkey as usize, i));
                }
            }

            for (idx, i) in to_add {
                monkeys[idx].items.push(i)
            }
        }
    }

    let mut s: Vec<usize> = counts.into_values().collect();
    s.sort();
    (s[s.len() - 1] * s[s.len() - 2]) as u128
}

// Looked this up in Reddit, don't understand how it works.
// TODO: Learn how it works.
fn get_magic_number(monkeys: &Vec<Monkey>) -> u128 {
    let mut n = 1;
    for m in monkeys {
        n *= m.test;
    }
    n
}

run!();

fn parse_monkey(monkey: &str) -> Monkey {
    let lines: Vec<&str> = monkey.lines().collect();
    let (_, str_items) = lines[1].split_once(": ").unwrap();
    let items: Vec<Item> = str_items.split(", ").map(|item| Item{worry_level: unum_128!(item)}).collect();

    let op = parse_operation(lines[2]);
    let op_type = parse_operation_value(lines[2]);
    let test = parse_test(lines[3]);
    Monkey {
        items,
        op,
        op_type,
        test,
        true_monkey: parse_next_monkey(lines[4]),
        false_monkey: parse_next_monkey(lines[5]),
    }
}

fn add(old: u128, v: u128) -> u128 {
    old + v
}

fn multiply(old: u128, v: u128) -> u128 {
    old * v
}

fn parse_operation(op: &str) -> fn(old: u128, v: u128) -> u128 {
    let vals: Vec<&str> = op.split_ascii_whitespace().collect();
    match vals[4] {
        "+" => add,
        "*" => multiply,
        _ => unreachable!(),
    }
}

fn parse_operation_value(op: &str) -> OpType {
    let vals: Vec<&str> = op.split_ascii_whitespace().collect();
    match vals[5] {
        "old" => OpType::Old,
        m => OpType::Val(unum_128!(m)),
    }
}

fn parse_test(line: &str) -> u128 {
    let vals: Vec<&str> = line.split_ascii_whitespace().collect();
    unum_128!(vals[3])
}

fn parse_next_monkey(line: &str) -> u128 {
    let vals: Vec<&str> = line.split_ascii_whitespace().collect();
    unum_128!(vals[5])
}
