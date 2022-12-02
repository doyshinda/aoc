use crate::util;
use ndarray::{arr1, arr2, Array1};
use std::collections::HashSet;

#[derive(Debug)]
struct Beacon {
    x: i32,
    y: i32,
    z: i32,
}

const DEGREES: f32 = std::f32::consts::PI / 180.0;

impl Beacon {
    fn new(x: &str, y: &str, z: &str) -> Beacon {
        Beacon {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
            z: z.parse::<i32>().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Scanner {
    beacons: Vec<Beacon>,
}

impl Scanner {
    fn parse(s: &str) -> Scanner {
        let mut beacons = Vec::new();
        for line in s.split('\n') {
            if line.len() == 0 {
                continue;
            }

            let (x, r) = line.split_once(',').unwrap();
            let (y, z) = r.split_once(',').unwrap();
            beacons.push(Beacon::new(x, y, z));
        }
        Scanner {
            beacons,
        }
    }
}

fn sin(i: f32) -> f32 {
    (i * DEGREES).sin().round()
}

fn cos(i: f32) -> f32 {
    let n = (i * DEGREES).cos();
    if n < 0.0 {
        n.round() * -1.0
    } else {
        n.round()
    }
}

macro_rules! rot {
    ( $a:literal, $b:literal ) => {
        match $a {
            'x' => arr2(&[
                    [1.0f32, 0.0f32, 0.0f32],
                    [0.0f32, cos($b), -sin($b)],
                    [0.0f32, sin($b), cos($b)],
                ]),
            'y' => arr2(&[
                    [cos($b), 0.0f32, sin($b)],
                    [0.0f32, 1.0f32, 0.0f32],
                    [-sin($b), 0.0f32, cos($b)],
                ]),
            'z' => arr2(&[
                    [cos($b), -sin($b), 0.0f32],
                    [sin($b), cos($b), 0.0f32],
                    [0.0f32, 0.0f32, 1.0f32],
                ]),
            _ => panic!(),
        }
    }
}

fn to_tuple(a: Array1<f32>) -> (i32, i32, i32) {
    (a[0] as i32, a[1] as i32, a[2] as i32)
}
fn part_1() -> i32 {
    let data = util::read_input("19_a_test");
    let mut lines = data.split("--- scanner ---");
    lines.next();
    let mut s = Vec::new();
    for line in lines {
        s.push(Scanner::parse(line));
    }

    let x = arr1(&[8.0f32, 0.0f32, 7.0f32]);
    let rots = vec![
        rot!('x', 90.0),
        rot!('x', 180.0),
        rot!('x', 270.0),
        rot!('y', 90.0),
        rot!('y', 180.0),
        rot!('y', 270.0),
        rot!('z', 90.0),
        rot!('z', 180.0),
        rot!('z', 270.0),
    ];
    // let x = arr1(&[0.0f32, 5.0f32, 0.0f32]);
    let mut uniques = HashSet::new();
    for (_i, r) in rots.into_iter().enumerate() {
        let ins = to_tuple(r.dot(&x));
        // println!("{} -> {:?}", i, ins);
        uniques.insert(ins);
    }

    for u in &uniques {
        println!("{:?}", u);
    }
    0
}

fn part_2() -> i32 {
    0
}

run!();
