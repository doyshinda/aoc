use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("9.input");
    let vals: Vec<u64> = data.lines().map(|x| unum!(x)).collect();
    let mut start = 0;
    let window_size = 25;
    'outer: loop {
        if start + window_size >= vals.len() {
            break;
        }
        let next = vals[start + window_size];
        let window = &vals[start..start+window_size];
        for (idx, i) in window.iter().enumerate() {
            for (jdx, j) in window.iter().enumerate() {
                if idx == jdx {
                    continue
                }
                if i + j == next {
                    start += 1;
                    continue 'outer;
                }
            }
        }

        return next;
    }
    0
}

fn part_2() -> u64 {
    let data = util::read_input("9.input");
    let vals: Vec<u64> = data.lines().map(|x| unum!(x)).collect();
    let mut start = 0;
    // let invalid = 127;
    let invalid = 50047984;
    'outer: loop {
        let mut sum = vals[start] + vals[start + 1];
        let mut next = start + 2;
        while sum < invalid {
            sum += vals[next];
            next += 1;
        }
        if sum > invalid {
            start += 1;
            continue 'outer;
        }

        next -= 1;
        let min = vals[start..next].iter().min().unwrap();
        let max = vals[start..next].iter().max().unwrap();
        return min + max;
    }
}

run!();
