use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("7.input");
    let mut ans = 0;

    for line in data.split("\n") {
        let (num, vals) = line.split_once(": ").unwrap();
        let num = unum!(num);
        let vals = auto_split!(vals);

        for c in &['+', '*'] {
            let n = recurse(num, vals[0], &vals[1..], *c);
            if n == num {
                ans += num;
                break;
            }
        }
    }

    ans
}

fn recurse(want: u64, have: u64, vals: &[u64], op: char) -> u64 {
    if vals.len() == 0 {
        return have;
    }

    let temp = match op {
        '*' => {
            have * vals[0]
        }
        '+' => {
            have + vals[0]
        }
        _ => unreachable!(),
    };

    for c in &['+', '*'] {
        let curr = recurse(want, temp, &vals[1..], *c);
        if curr == want {
            return want;
        }
    }

    return 0;
}

fn part_2() -> u64 {
    let data = util::read_input("7.input");
    let mut ans = 0;

    for line in data.split("\n") {
        let (num, vals) = line.split_once(": ").unwrap();
        let num = unum!(num);
        let vals = auto_split!(vals);
        let mut p = vec![];

        for c in &['+', '*', '|'] {
            let n = recurse2(num, vals[0], &vals[1..], *c, &mut p);
            if n == num {
                ans += num;
                break;
            }
            p.clear();
        }
    }

    ans
}

fn recurse2(want: u64, have: u64, vals: &[u64], op: char, r: &mut Vec<char>) -> u64 {
    r.push(op);
    if vals.len() == 0 {
        return have;
    }

    let temp = match op {
        '*' => {
            have * vals[0]
        }
        '+' => {
            have + vals[0]
        }
        '|' => {
            let t = have.to_string() + &vals[0].to_string();
            unum!(t)
        }
        _ => unreachable!(),
    };

    if temp > want {
        return 0;
    }

    for c in &['+', '*', '|'] {
        let curr = recurse2(want, temp, &vals[1..], *c, r);
        if curr == want {
            return want;
        }
        r.pop();
    }

    return 0;
}

run!();

fn print_match(n: &[u64], p: &[char]) {
    for (i, v) in p.iter().enumerate() {
        print!("{} {} ", n[i], v);
    }
    println!("{}", n.iter().last().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let want = 292;
        let vals = vec![11, 6, 16, 20];
        let mut a = recurse(want, 0, &vals, '*');
        a += recurse(want, 0, &vals, '+');
        assert_eq!(want, a);
    }

    #[test]
    fn test1() {
        let want = 190;
        let vals = vec![10, 19];
        let mut a = recurse(want, 0, &vals, '+');
        a += recurse(want, 0, &vals, '*');
        assert_eq!(want, a);
    }

    #[test]
    fn test2() {
        let want = 156;
        let vals = vec![15, 6];
        let mut found = false;
        for c in &['+', '*', '|'] {
            let mut path = vec![];
            let n = recurse2(want, vals[0], &vals[1..], *c, &mut path);
            if n == want {
                // dbg!(&path);
                assert_eq!(path, vec!['|']);
                print_match(&vals, &path);
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn test3() {
        let want = 230639;
        let vals = vec![88, 6, 3, 722, 3, 9, 1];
        let mut found = false;
        for c in &['+', '*', '|'] {
            let mut path = vec![];
            let n = recurse2(want, vals[0], &vals[1..], *c, &mut path);
            if n == want {
                // dbg!(&path);
                // assert_eq!(path, vec!['*', '|', '*']);
                print_match(&vals, &path);
                found = true;
                break;
            }
        }
        assert!(!found);
    }
    // 7290: 6 8 6 15
    // 230639: 88 6 3 722 3 9 1
}

