use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("5.input");
    data.lines().map(|x| parse_pass(x.trim_end())).max().unwrap()
}

fn part_2() -> u64 {
    let data = util::read_input("5.input");
    let mut sids: Vec<_> = data.lines().map(|x| parse_pass(x.trim_end())).collect();
    sids.sort();
    let mut prev = 0;
    for s in sids {
        if prev == 0 {
            prev = s;
            continue
        }

        if prev+1 != s {
            return prev + 1;
        }
        prev = s
    }
    0
}

run!();

fn parse_pass(value: &str) -> u64 {
    let mut min = 0;
    let mut max = 127;
    for c in value.chars() {
        let mid = ((max - min) + 1) / 2;
        match c {
            'F' => max = max - mid,
            'B' => min = min + mid,
            _ => (),
        }
    }
    let sid = min;

    min = 0;
    max = 7;
    for c in value.chars() {
        let mid = ((max - min) + 1) / 2;
        match c {
            'L' => max = max - mid,
            'R' => min = min + mid,
            _ => (),
        }
    }

    return (sid * 8) + min;
}
