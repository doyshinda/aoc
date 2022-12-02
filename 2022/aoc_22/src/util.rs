#![macro_use]
#![allow(unused_macros)]
use std::fs;

const INPUT_DIR: &str = "/home/abe/misc/adventofcode/2022/aoc_22/input/";

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
    read_input(name).split('\n').into_iter().map(|x| T::parse(x)).collect()
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

macro_rules! log {
    ($($arg:tt)*) => {
        match std::env::var("DEBUG") {
            Ok(_) => println!($($arg)*),
            _ => (),
        }
    };
}
