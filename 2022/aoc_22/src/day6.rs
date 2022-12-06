use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("6.input");
    let chars: Vec<_> = data.chars().collect();
    let mut idx = 4;
    for chunk in chars.windows(4) {
        hs!(seen, char, chunk);
        if seen.len() == 4 {
            return idx;
        }
        idx += 1;
    }
    0
}

fn part_2() -> u64 {
    let data = util::read_input("6.input");
    let chars: Vec<_> = data.chars().collect();
    let mut idx = 14;
    for chunk in chars.windows(14) {
        hs!(seen, char, chunk);
        if seen.len() == 14 {
            return idx;
        }
        idx += 1;
    }
    0
}

run!();
