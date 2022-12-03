use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("1.input");
    let vals: Vec<u64> = data.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let v2 = vals.clone();
    for (idx, v) in vals.iter().enumerate() {
        for (jdx, x) in v2.iter().enumerate() {
            if idx == jdx {
                continue;
            }

            if v + x == 2020 {
                return v * x;
            }
        }
    }
    0
}

fn part_2() -> u64 {
    let data = util::read_input("1.input");
    let vals: Vec<u64> = data.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    for i in 0..vals.len() {
        for j in 0..vals.len() {
            if i == j {
                continue;
            }

            for k in 0..vals.len() {
                if i == k || i == j || j == k {
                    continue
                }

                if vals[i] + vals[j] + vals[k] == 2020 {
                    return vals[i] * vals[j] * vals[k];
                }
            }
        }
    }

    0
}

run!();
