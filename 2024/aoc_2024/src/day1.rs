use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("1.input");
    let vals = data.split("\n");

    let mut sum = 0;
    let mut a = vec![];
    let mut b = vec![];

    for l in vals {
        let (av, bv) = l.split_once("   ").unwrap();
        a.push(unum!(av));
        b.push(unum!(bv));
    }

    a.sort();
    b.sort();

    for (a, b) in std::iter::zip(a.iter(), b.iter()) {
        let diff = a.abs_diff(*b);
        sum += diff;
    }
    
    sum
}

fn part_2() -> u64 {
    let data = util::read_input("1.input");
    let vals = data.split("\n");

    let mut sum = 0;
    let mut a = vec![];
    hm!(b_map, u64, u64);

    for l in vals {
        let (av, bv) = l.split_once("   ").unwrap();
        a.push(unum!(av));
        let bv = unum!(bv);
        hm_inc!(b_map, bv, 1);
    }

    for v in &a {
        match b_map.get(v) {
            Some(count) => sum += *v * count,
            _ => (),
        }
    }

    sum
}

run!();
