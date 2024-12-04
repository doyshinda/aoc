#![macro_use]
#![allow(unused_macros)]
use std::fs;

const INPUT_DIR: &str = "/home/abe/projects/aoc/2024/aoc_2024/input/";
// const INPUT_DIR: &str = "/tmp/";

#[derive(Debug)]
pub enum SubVectorChange {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub trait Parseable {
    type Output;

    fn parse(input: &str) -> Self::Output;
}

impl Parseable for i32 {
    type Output = i32;

    fn parse(s: &str) -> Self::Output {
        s.parse::<i32>().unwrap()
    }
}

pub fn read_input(name: &str) -> String {
    fs::read_to_string(format!("{}{}", INPUT_DIR, name))
        .expect("Something went wrong reading the file")
}

pub fn read_input_vector<T: Parseable<Output = T>>(name: &str) -> Vec<T> {
    read_input(name)
        .split('\n')
        .into_iter()
        .map(|x| T::parse(x))
        .collect()
}

macro_rules! run {
    () => {
        pub fn run() {
            let start = std::time::Instant::now();
            let answer = part_1();
            print!(
                "====================\npart 1 [{:?}]\n====================\n",
                start.elapsed(),
            );
            println!("{:?}\n\n", answer);

            let start = std::time::Instant::now();
            let answer = part_2();
            print!(
                "====================\npart 2 [{:?}]\n====================\n",
                start.elapsed(),
            );
            println!("{:?}\n\n", answer);
        }
    };
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    let mut r;
    while (a % b) > 0 {
        r = a % b;
        a = b;
        b = r;
    }
    return b;
}

pub fn gcd_vals(vals: &Vec<u64>) -> u64 {
    let mut i = vals.iter().skip(2);
    let mut calc_gcd = gcd(vals[0], vals[1]);
    while let Some(n) = i.next() {
        calc_gcd = gcd(calc_gcd, *n);
    }

    calc_gcd
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

pub fn lcm_vals(vals: &Vec<u64>) -> u64 {
    let mut i = vals.iter().skip(2);
    let mut calc_lcm = lcm(vals[0], vals[1]);
    while let Some(n) = i.next() {
        calc_lcm = lcm(calc_lcm, *n);
    }

    calc_lcm
}

macro_rules! log {
    ($($arg:tt)*) => {
        match std::env::var("DEBUG") {
            Ok(_) => println!($($arg)*),
            _ => (),
        }
    };
}

macro_rules! unum {
    ($arg:tt) => {
        $arg.parse::<u64>().unwrap()
    };

    ($arg:expr) => {
        $arg.parse::<u64>().unwrap()
    };
}

macro_rules! usnum {
    ($arg:tt) => {
        $arg.parse::<usize>().unwrap()
    };
    ($arg:expr) => {
        $arg.parse::<usize>().unwrap()
    };
}

macro_rules! inum {
    ($arg:tt) => {
        $arg.parse::<i64>().unwrap()
    };

    ($arg:expr) => {
        $arg.parse::<i64>().unwrap()
    };
}

macro_rules! hs {
    () => {
        std::collections::HashSet::new()
    };
    ($name:ident, $t:ty) => {
        let mut $name: std::collections::HashSet<$t> = std::collections::HashSet::new();
    };
    ($arg:expr) => {
        std::collections::HashSet::from_iter($arg.iter().cloned())
    };
    ($name:ident, $t:ty, $arg:expr) => {
        let $name: std::collections::HashSet<$t> =
            std::collections::HashSet::from_iter($arg.iter().cloned());
    };
}

macro_rules! hm {
    () => {
        std::collections::HashMap::new()
    };
    ($name:ident, $t:ty, $t2:ty) => {
        let mut $name: std::collections::HashMap<$t, $t2> = std::collections::HashMap::new();
    };
    ($arg:expr) => {
        std::collections::HashMap::from_iter($arg.iter().cloned())
    };
    ($name:ident, $t:ty, $t2:ty, $arg:expr) => {
        let $name: std::collections::HashMap<$t, $t2> =
            std::collections::HashMap::from_iter($arg.iter().cloned());
    };
}

macro_rules! hm_inc {
    ($name:ident, $key:expr, $value:expr) => {
        $name
            .entry($key)
            .and_modify(|c| *c += $value)
            .or_insert($value);
    };
}
