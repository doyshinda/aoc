use crate::util;
use crate::util::Parseable;
use either::Either;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::recognize,
    sequence::separated_pair,
    IResult,
};

struct Vector {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Vector {
    fn is_diag(&self) -> bool {
        (self.x1 - self.x2).abs() == (self.y1 - self.y2).abs()
    }
}

fn x(input: &str) -> IResult<&str, i32> {
    recognize(digit1)(input).map(|(i, res)| (i, res.parse::<i32>().unwrap()))
}

fn coord(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(x, tag(","), x)(input)
}

fn vector(input: &str) -> IResult<&str, ((i32, i32), (i32, i32))> {
    separated_pair(coord, tag(" -> "), coord)(input)
}

impl Parseable for Vector {
    type Output = Vector;
    fn parse(s: &str) -> Self::Output {
        let (_, r) = vector(s).unwrap();
        Vector{
            x1: r.0.0,
            x2: r.1.0,
            y1: r.0.1,
            y2: r.1.1,
        }
    }
}

fn part_1() -> i32 {
    let vectors: Vec<Vector> = util::read_input_vector("5_a");
    let size = 1000;
    let mut answer = Vec::with_capacity(size);
    for _ in 0..size {
        let v = vec![0; size];
        answer.push(v)
    }
    for v in vectors {
        if v.x1 == v.x2 {
            let (start, end) = if v.y1 >= v.y2 {
                (v.y2, v.y1 + 1)
            } else {
                (v.y1, v.y2 + 1)
            };
            for n in start..end {
                answer[n as usize][v.x1 as usize] += 1;
            }
            continue
        }
        if v.y1 == v.y2 {
            let (start, end) = if v.x1 >= v.x2 {
                (v.x2, v.x1 + 1)
            } else {
                (v.x1, v.x2 + 1)
            };
            for n in start..end {
                answer[v.y1 as usize][n as usize] += 1;
            }
        }
    }

    answer
        .into_iter()
        .map(|x| x.into_iter().filter(|x| *x >= 2).count() as i32)
        .sum::<i32>()
}

fn part_2() -> i32 {
    let vectors: Vec<Vector> = util::read_input_vector("5_a");
    let size = 1000;
    let mut answer = Vec::with_capacity(size);
    for _ in 0..size {
        let v = vec![0; size];
        answer.push(v)
    }
    for v in vectors {
        if v.is_diag() {
            let num = (v.y1 - v.y2).abs() + 1;
            let mut yiter = if v.y1 >= v.y2 {
                Either::Left((v.y2..=v.y1).rev())
            } else {
                Either::Right(v.y1..=v.y2)
            };
            let mut xiter = if v.x1 >= v.x2 {
                Either::Left((v.x2..=v.x1).rev())
            } else {
                Either::Right(v.x1..=v.x2)
            };

            for _ in 0..num {
                let (x, y) = (xiter.next().unwrap(), yiter.next().unwrap());
                answer[y as usize][x as usize] += 1;
            }
            continue;
        }
        if v.x1 == v.x2 {
            let (start, end) = if v.y1 >= v.y2 {
                (v.y2, v.y1 + 1)
            } else {
                (v.y1, v.y2 + 1)
            };
            for n in start..end {
                answer[n as usize][v.x1 as usize] += 1;
            }
            continue;
        }
        if v.y1 == v.y2 {
            let (start, end) = if v.x1 >= v.x2 {
                (v.x2, v.x1 + 1)
            } else {
                (v.x1, v.x2 + 1)
            };
            for n in start..end {
                answer[v.y1 as usize][n as usize] += 1;
            }
        }
    }
    answer
        .into_iter()
        .map(|x| x.into_iter().filter(|x| *x >= 2).count() as i32)
        .sum::<i32>()
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?}, took {}us", answer, start.elapsed().as_micros());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?}, took {}us", answer, start.elapsed().as_micros());
}

#[test]
fn test_parse() {
    let inp = "0,9 -> 5,9";
    let v = Vector::parse(&inp);
    assert_eq!(v.x1, 0);
    assert_eq!(v.x2, 5);
    assert_eq!(v.y1, 9);
    assert_eq!(v.y2, 9);
}
