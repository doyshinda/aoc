#![macro_use]
#![allow(unused_macros)]
use std::collections::{HashMap, VecDeque};
use griddy::Grid;
use once_cell::sync::Lazy;
use std::fs;
use regex::Regex;

pub static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"-?\d+").unwrap());

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
            let elapsed = start.elapsed();
            print!(
                "====================\npart 1 [{:?}]\n====================\n",
                elapsed,
            );
            println!("{:?}\n\n", answer);

            let start = std::time::Instant::now();
            let answer = part_2();
            let elapsed2 = start.elapsed();
            print!(
                "====================\npart 2 [{:?}]\n====================\n",
                elapsed2,
            );
            println!("{:?}\n\n[{:?}]", answer, elapsed + elapsed2);
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

pub fn dijkstra(grid: &Grid<u64>, x: usize, y: usize) -> u64 {
    let mut dist = HashMap::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let source = (x, y);
    for y in 0..grid.rows_len() {
        for x in 0..grid.cols_len() {
            let v = (y, x);
            dist.insert(v, 1000000u64);
            queue.push(v);
        }
    }

    dist.insert(source, 0);
    while queue.len() > 0 {
        let c = find_min(&mut queue, &dist);
        let (y, x) = c;

        for n in grid.row_neighbors(y, x).into_iter().chain(grid.col_neighbors(y, x)) {
            if !queue.contains(&n) {
                continue;
            }

            let dist_u = dist.get(&c).unwrap();
            let dist_v = dist.get(&n).unwrap();
            let alt = dist_u + grid[n.0][n.1];

            if alt < *dist_v {
                dist.insert(n, alt);
            }
        }
    }

    *dist.get(&(grid.rows_len() - 1, grid.cols_len() - 1)).unwrap()
}

fn find_min(q: &mut Vec<(usize, usize)>, d: &HashMap<(usize, usize), u64>) -> (usize, usize) {
    let mut min_val = u64::MAX;
    let mut min_idx = 0;
    for (idx, c) in q.iter().enumerate() {
        let val = d.get(c).unwrap();
        if *val < min_val {
            min_val = *val;
            min_idx = idx;
        }
    }

    q.swap_remove(min_idx)
}

pub fn print(grid: &Grid<char>) {
    println!("{}", "====================================");
    for r in grid.rows() {
        println!("{:?}", r.iter().collect::<String>());
    }
}

#[derive(Debug)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub cnt: u64,
    pub path: Vec<(usize, usize)>,
}

pub fn dfs<F, T>(grid: &Grid<T>, x_start: usize, start: usize, mut f: F)
where
    F: FnMut(&Node, &Node) -> bool
{
    let n = Node{
        x: x_start,
        y: start,
        cnt: 0,
        path: vec![(x_start, start)],
    };

    let mut next = VecDeque::new();
    next.push_back(n);

    while next.len() > 0 {
        let n = next.pop_back().unwrap();

        for v in grid.immediate_neighbors(n.x, n.y) {
            if n.path.contains(&v) {
                continue;
            }

            let mut candidate = Node{
                x: v.0,
                y: v.1,
                cnt: n.cnt + 1,
                path: n.path.clone(),
            };

            if f(&n, &candidate) {
                candidate.path.push(v);
                next.push_back(candidate);
            }
        }
    }
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

macro_rules! auto_split {
    ($line:tt, $t:ty) => {
        {
            let split = $line.chars().filter(|x| !x.is_ascii_digit()).next().unwrap();
            match split {
                ' ' => $line.split_ascii_whitespace().map(|x| x.parse::<$t>().unwrap()).collect::<Vec<$t>>(),
                _ => $line.split(split).map(|x| x.parse::<$t>().unwrap()).collect::<Vec<$t>>(),
            }
        }
    };
    ($line:tt) => {
        auto_split!($line, u64)
    };
}

macro_rules! uints {
    ($line:tt) => {
        crate::util::RE.find_iter($line).map(|x| x.as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>()
    };
}

macro_rules! usplit_once {
    ($arg:expr, $s:tt) => {
        $arg.split_once($s).and_then(|(a, b)| Some((unum!(a), unum!(b)))).expect("invalid split")
    }
}

macro_rules! convert {
    ($arg:expr) => {
        $arg.try_into().unwrap()
    };
}
