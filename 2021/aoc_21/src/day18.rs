use crate::util;
use std::{
    env,
    fmt,
    ops::Add,
};

const OPEN: i32 = -2;
const CLOSE: i32 = -1;
const SEPARATOR: i32 = -3;

#[derive(Debug, Eq, Clone, PartialEq)]
struct Number {
    c: Vec<i32>
}

impl Number {
    fn new(s: &str) -> Self {
        Number {
            c: s.chars().map(|c| {
                match c {
                    '[' => OPEN,
                    ']' => CLOSE,
                    ',' => SEPARATOR,
                    v => v.to_digit(10).expect("invalid input number") as i32,
                }
            }).collect()
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if !self.split() {
                break;
            }
        }

    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;
        for i in 0..self.c.len() {
            let m = self.c[i];
            match m {
                OPEN => { depth += 1; continue },
                CLOSE => { depth -= 1; continue },
                _ => (),
            }
            if depth == 5 {
                log!("found explode candidate: {}", m);
                log!("before: {}", self);
                self.add_left(i, m);
                log!("after : {}", self);

                log!("before: {}", self);
                self.add_right(i + 2);
                log!("after : {}", self);

                for _ in 0..4 {
                    self.c.remove(i - 1);
                }
                self.c[i- 1] = 0;
                log!("done  : {}", self);
                return true;
            }
        }
        false
    }

    fn add_left(&mut self, i: usize, o: i32) {
        for j in (0..i).rev() {
            if self.c[j] > -1 {
                self.c[j] += o;
                return;
            }
        }
    }

    fn add_right(&mut self, i: usize) {
        for (j, v) in self.c[i + 1..].iter().enumerate() {
            if *v > -1 {
                self.c[i + j + 1] += self.c[i];
                break;
            }
        }
    }

    fn split(&mut self) -> bool {
        for i in 0..self.c.len() {
            if self.c[i] > 9 {
                log!("Found split candidate: {}", self.c[i]);
                log!("before: {}", self);
                self.c.insert(i + 1, CLOSE);
                self.c.insert(i + 1, ceil(self.c[i]));
                self.c.insert(i + 1, SEPARATOR);
                self.c.insert(i + 1, self.c[i] / 2);
                self.c[i] = OPEN;
                log!("after : {}", self);
                return true;
            }
        }
        return false;
    }

    fn magnitude(&self) -> i64 {
        let mut l = self.c.clone();
        while l.len() > 4 {
            log!("{}", to_str(&l));
            let mut m = Vec::new();
            let mut skip = 0;
            for s in l.windows(5) {
                log!("s -> {}", to_str(&s));
                if skip > 0 {
                    skip -= 1;
                    continue;
                }
                match s {
                    [OPEN, l, SEPARATOR, r, CLOSE] => {
                        m.push((3 * l) + (2 * r));
                        skip = 4;
                    },
                    [a, _, _, _, _] => m.push(*a),
                    _ => (),
                }
                log!("m -> {}", to_str(&m));
            }
            if m.len() > 1 {
                let e = l.len() - 4 + skip;
                for v in &l[e..] {
                    m.push(*v);
                }
            } else {
                return m.pop().unwrap() as i64;
            }
            l = m;
        }
        0
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut new = vec![OPEN];
        new.extend(self.c);
        new.push(SEPARATOR);
        new.extend(other.c);
        new.push(CLOSE);
        Number {c: new}
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = to_str(&self.c);
        write!(f, "{}", s)
    }
}

macro_rules! num {
    ( $arg:tt ) => { Number::new($arg) };
}

fn to_str(c: &[i32]) -> String {
    let mut s = String::new();
    for n in c {
        match *n {
            OPEN => s.push('['),
            SEPARATOR => s.push(','),
            CLOSE => s.push(']'),
            _ => s.push_str(&n.to_string()),
        }
    }
    s
}

fn ceil(v: i32) -> i32 {
    if v % 2 == 0 {
        v / 2
    } else {
        (v / 2) + 1
    }
}

fn part_1() -> i64 {
    let mut prev = None;
    for line in util::read_input("18_a").split('\n') {
        let n = num!(line);
        if prev.is_none() {
            prev = Some(n);
        } else {
            let mut a = prev.unwrap() + n;
            a.reduce();
            prev = Some(a);
        }
    }
    prev.unwrap().magnitude() as i64
}

fn part_2() -> i64 {
    let mut numbers = Vec::new();
    for line in util::read_input("18_a").split('\n') {
        numbers.push(num!(line));
    }

    let mut max = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }

            let mut a = numbers[i].clone() + numbers[j].clone();
            a.reduce();
            let m = a.magnitude();
            if m > max {
                max = m;
            }

            a = numbers[j].clone() + numbers[i].clone();
            a.reduce();
            let m = a.magnitude();
            if m > max {
                max = m;
            }
        }
    }
    max
}

run!();

#[test]
fn test_add() {
    let a = num!("[1,2]");
    let b = num!("[[3,4],5]");
    let r = a + b;
    assert_eq!(r, num!("[[1,2],[[3,4],5]]"));
}

#[test]
fn test_explode_no_left() {
    let mut a = num!("[[[[[9,8],1],2],3],4]");
    assert!(a.explode());
    assert_eq!(a, num!("[[[[0,9],2],3],4]"))
}

#[test]
fn test_explode_no_right() {
    let mut a = num!("[7,[6,[5,[4,[3,2]]]]]");
    assert!(a.explode());
    assert_eq!(a, num!("[7,[6,[5,[7,0]]]]"))
}

#[test]
fn test_explode_norm() {
    let mut a = num!("[[6,[5,[4,[3,2]]]],1]");
    assert!(a.explode());
    let b = num!("[[6,[5,[7,0]]],3]");
    assert_eq!(a, b, "\ne: {}, \ng: {}\n", b, a);

    let mut a = num!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
    assert!(a.explode());
    assert_eq!(a, num!("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));

    let mut a = num!("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
    assert!(a.explode());
    assert_eq!(a, num!("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"));

    let mut a = num!("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]");
    assert!(a.explode());
    let b = Number {c: vec![-2, -2, -2, -2, 0, -3, 7, -1, -3, 4, -1, -3, -2, 15, -3, -2, 0, -3, 13, -1, -1, -1, -3, -2, 1, -3, 1, -1, -1]};
    assert_eq!(a, b, "\ne: {}, \ng: {}\n", b, a);
}

#[test]
fn test_reduce_no_left() {
    let mut a = num!("[[[[[9,8],1],2],3],4]");
    a.reduce();
    assert_eq!(a, num!("[[[[0,9],2],3],4]"))
}

#[test]
fn test_reduce_no_right() {
    let mut a = num!("[7,[6,[5,[4,[3,2]]]]]");
    a.reduce();
    assert_eq!(a, num!("[7,[6,[5,[7,0]]]]"))
}

#[test]
fn test_reduce_norm() {
    let mut a = num!("[[6,[5,[4,[3,2]]]],1]");
    a.reduce();
    assert_eq!(a, num!("[[6,[5,[7,0]]],3]"));
}

#[test]
fn test_example() {
    let a = num!("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = num!("[1,1]");
    let mut c = a + b;
    c.reduce();
    assert_eq!(c, num!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
}

#[test]
fn test_magnitude() {
    let a = num!("[9,1]");
    assert_eq!(a.magnitude(), 29);

    let a = num!("[[9,1],[1,9]]");
    assert_eq!(a.magnitude(), 129);

    let a = num!("[[1,2],[[3,4],5]]");
    assert_eq!(a.magnitude(), 143);

    let a = num!("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    assert_eq!(a.magnitude(), 1384);

    let a = num!("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    assert_eq!(a.magnitude(), 445);
}

#[test]
fn test_reduce_long() {
    let mut a = num!("[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]");
    a.reduce();
    let b = num!("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
    assert_eq!(a, b, "\ne: {}, \ng: {}\n", b, a);
}
