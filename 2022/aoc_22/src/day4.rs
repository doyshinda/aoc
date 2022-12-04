use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("4.input");
    let mut sum = 0;
    for line in data.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let (c, d) = a.split_once('-').unwrap();
        let (e, f) = b.split_once('-').unwrap();

        let a_min = unum!(c);
        let a_max = unum!(d);
        let b_min = unum!(e);
        let b_max = unum!(f);

        if a_min >= b_min && a_max <= b_max {
            sum += 1;
            continue;
        }

        if b_min >= a_min && b_max <= a_max {
            sum += 1;
            continue;
        }
    }
    sum
}

fn part_2() -> u64 {
    let data = util::read_input("4.input");
    let mut sum = 0;
    for line in data.lines() {
        let (a, b) = line.split_once(',').unwrap();
        let (c, d) = a.split_once('-').unwrap();
        let (e, f) = b.split_once('-').unwrap();

        let a_min = unum!(c);
        let a_max = unum!(d);
        let b_min = unum!(e);
        let b_max = unum!(f);

        for i in b_min..b_max+1 {
            if i >= a_min && i <= a_max{
                sum += 1;
                break;
            }
        }
    }
    sum
}

run!();
