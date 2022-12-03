use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("2.input");
    let mut sum: u64 = 0;
    for line in data.lines() {
        let mut vals = line.split_ascii_whitespace();
        let min_max = vals.next().unwrap();
        let the_char = vals.next().unwrap().trim().chars().next().expect("string is empty");
        let passwd = vals.next().unwrap();

        let (min, max) = min_max.split_once('-').unwrap();
        let min_val = unum!(min);
        let max_val = unum!(max);
        let c = passwd.chars().filter(|x| *x == the_char).count() as u64;
        if min_val <= c && c <= max_val {
            sum += 1;
        }
    }
    sum
}

fn part_2() -> u64 {
    let data = util::read_input("2.input");
    let mut sum: u64 = 0;
    for line in data.lines() {
        let mut vals = line.split_ascii_whitespace();
        let min_max = vals.next().unwrap();
        let the_char = vals.next().unwrap().trim().chars().next().expect("string is empty");
        let passwd: Vec<_> = vals.next().unwrap().chars().collect();

        let (min, max) = min_max.split_once('-').unwrap();
        let min_val = usnum!(min) - 1;
        let max_val = usnum!(max) - 1;

        let m = passwd[min_val];
        let a = passwd[max_val];
        if m == a {
            continue
        }

        if m == the_char || a == the_char {
            sum += 1;
        }
    }
    sum
}

run!();
