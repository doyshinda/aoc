use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("1.input");
    let vals = data.split("\n\n");
    let mut max = 0;

    for e in vals {
        let this = e.split("\n").map(|x| {
            if x.is_empty() {
                0u64
            } else {
                x.parse::<u64>().unwrap()
            }
        }).sum();
        if this > max {
            max = this;
        }
    }
    max
}

fn part_2() -> u64 {
    let data = util::read_input("1.input");
    let vals = data.split("\n\n");
    let mut sums = vec![];

    for e in vals {
        let this: u64 = e.split("\n").map(|x| {
            if x.is_empty() {
                0u64
            } else {
                x.parse::<u64>().unwrap()
            }
        }).sum();
        sums.push(this);
    }

    sums.sort();
    let l = sums.len();
    return sums[l-1] + sums[l-2] + sums[l-3];
}

run!();
