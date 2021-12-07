use crate::util;

fn part_1() -> usize {
    let positions: Vec<i32> = util::read_input("7_a")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let mut fuel_used: Vec<i32> = Vec::new();
    let max = positions.iter().max().unwrap();
    for i in 0..*max {
        fuel_used.push(positions.iter().map(|x| (i - x).abs()).sum());
    }

    match fuel_used.iter().min() {
        Some(v) => *v as usize,
        _ => 0,
    }
}

fn part_2() -> usize {
    let positions: Vec<i32> = util::read_input("7_a")
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut fuel_used: Vec<i32> = Vec::new();
    let max = positions.iter().max().unwrap();
    for i in 0..*max {
        fuel_used.push(
            positions
                .iter()
                .map(|x| {
                    let n = (i - x).abs();
                    (n * (n + 1)) / 2
                })
                .sum(),
        );
    }

    match fuel_used.iter().min() {
        Some(v) => *v as usize,
        _ => 0,
    }
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}
