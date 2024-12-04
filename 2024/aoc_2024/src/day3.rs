use crate::util;
use regex::Regex;

const DONT: &str = "don't()";
const DO: &str = "do()";

fn part_1() -> u64 {
    let data = util::read_input("3.input");
    let mut sum = 0;
    let re = Regex::new(r"^([0-9]+,[0-9]+\)).*").unwrap();
    for line in data.split("\n") {
        let sections = line.split("mul(");
        for s in sections {
            log!("section: {}", s);
            match re.captures(s) {
                Some(caps) => {
                    let (a, b) = &caps[0].split_once(",").unwrap();
                    match b.find(")") {
                        Some(idx) => {
                            let (r, _) = b.split_at(idx);
                            log!("a: {}, b: {}", a, r);
                            sum += unum!(a) * unum!(r);
                        },
                         _ => (),
                    }
                }
                _ => (),
            }
        }
    }
    sum
}

fn part_2() -> u64 {
    let data = util::read_input("3.input");
    let mut sum = 0;
    let mul = Regex::new(r"mul\((\d+,\d+)\)").unwrap();
    let re = Regex::new(r"(mul\(\d+,\d+\)|don't\(\)|do\(\))").unwrap();
    let mut disabled = false;
    for line in data.split("\n") {
        for c in re.captures_iter(line) {
            let (_, [val]) = c.extract();
            match val {
                DONT => disabled = true,
                DO => disabled = false,
                _ => {
                    let caps = mul.captures(val).unwrap();
                    let (a, b) = &caps[1].split_once(",").unwrap();
                    if !disabled {
                        sum += unum!(a) * unum!(b);
                    }
                }
            }            
        }
    }
    sum
}

run!();
