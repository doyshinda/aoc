use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("6.input");
    let lines: Vec<&str> = data.split("\n").collect();
    let (_, right) = lines[0].split_once(":").unwrap();
    let right = right.trim();
    let time_vals = right.split_ascii_whitespace().map(|x| unum!(x));

    let (_, right) = lines[1].split_once(":").unwrap();
    let right = right.trim();
    let distance_vals: Vec<_> = right.split_ascii_whitespace().map(|x| unum!(x)).collect();

    let mut sum = 1;
    for (i, t) in time_vals.enumerate() {
        let mut wins = 0;
        let d = distance_vals[i];
        for button_hold_time in 1..t {
            let traveled = calc_distance(button_hold_time as u64, t);
            if traveled > d {
                wins += 1;
            }
        }
        sum *= wins;
    }
    sum
}

fn part_2() -> u64 {
        let data = util::read_input("6.input");
    let lines: Vec<&str> = data.split("\n").collect();
    let (_, right) = lines[0].split_once(":").unwrap();
    let right = right.replace(" ", "");
    let time = unum!(right);

    let (_, right) = lines[1].split_once(":").unwrap();
    let right = right.replace(" ", "");
    let distance = unum!(right);

    let mut wins = 0;
    for button_hold_time in 1..time {
        let traveled = calc_distance(button_hold_time as u64, time);
        if traveled > distance {
            wins += 1;
        }
    }
    wins
}

fn calc_distance(ht: u64, t: u64) -> u64 {
    let run_time = t - ht;
    let speed = ht * 1;
    run_time * speed
}

run!();
