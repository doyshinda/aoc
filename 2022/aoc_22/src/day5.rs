use crate::util;

#[derive(Debug)]
struct Move{
    count: usize,
    src: usize,
    dst: usize,
}

fn part_1() -> u64 {
    let data = util::read_input("5.input");
    let (stack_lines, moves) = data.split_once("\n\n").unwrap();
    let moves = moves.lines().map(|m| parse_move(m)).collect::<Vec<Move>>();
    let stacks: Vec<Vec<char>> = stack_lines.lines().map(|line| {
        let chars: Vec<_> = line.chars().collect();
        chars.chunks(4).map(|c| {
            if c[0] == '[' {
                c[1]
            } else {
                'E'
            }
        }).collect()
    }).collect();

    let mut new_stacks = Vec::new();
    for row in stacks.iter().rev().skip(1) {
        for (idx, v) in row.iter().enumerate() {
            if idx >= new_stacks.len() {
                new_stacks.push(vec![]);
            }
            if *v == 'E' {
                continue;
            }
            new_stacks[idx].push(v);
        }
    };

    for m in moves {
        for _ in 0..m.count {
            let old = new_stacks[m.src].pop().unwrap();
            new_stacks[m.dst].push(old);
        }
    }

    for mut n in new_stacks {
        print!("{}", n.pop().unwrap());
    }
    println!();

    0
}

fn part_2() -> u64 {
    let data = util::read_input("5.input");
    let (stack_lines, moves) = data.split_once("\n\n").unwrap();
    let moves = moves.lines().map(|m| parse_move(m)).collect::<Vec<Move>>();
    let stacks: Vec<Vec<char>> = stack_lines.lines().map(|line| {
        let chars: Vec<_> = line.chars().collect();
        chars.chunks(4).map(|c| {
            if c[0] == '[' {
                c[1]
            } else {
                'E'
            }
        }).collect()
    }).collect();

    let mut new_stacks = Vec::new();
    for row in stacks.iter().rev().skip(1) {
        for (idx, v) in row.iter().enumerate() {
            if idx >= new_stacks.len() {
                new_stacks.push(vec![]);
            }
            if *v == 'E' {
                continue;
            }
            new_stacks[idx].push(v);
        }
    };

    for m in moves {
        let mut old = Vec::new();
        for _ in 0..m.count {
            let ov = new_stacks[m.src].pop().unwrap();
            old.push(ov);
        }

        for v in old.into_iter().rev() {
            new_stacks[m.dst].push(v);
        }
    }

    for mut n in new_stacks {
        print!("{}", n.pop().unwrap());
    }
    println!();

    0
}

run!();

fn parse_move(line: &str) -> Move {
    let vals: Vec<_> = line.split_ascii_whitespace().collect();
    Move{
        count: usnum!(vals[1]),
        src: usnum!(vals[3]) - 1,
        dst: usnum!(vals[5]) - 1,
    }
}
