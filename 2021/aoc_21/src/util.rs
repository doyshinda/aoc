use std::fs;

const INPUT_DIR: &str = "/home/abe/misc/adventofcode/2021/aoc_21/input/";

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

impl Parseable for SubVectorChange {
    type Output = SubVectorChange;

    fn parse(s: &str) -> Self::Output {
        let vals: Vec<&str> = s.split(' ').collect();
        let ival: i32 = vals[1].parse::<i32>().unwrap();
        match vals[0] {
            "forward" => SubVectorChange::Forward(ival),
            "down" => SubVectorChange::Down(ival),
            "up" => SubVectorChange::Up(ival),
            _ => panic!("unknown vector change"),
        }
    }
}

impl Parseable for i32 {
    type Output = i32;

    fn parse(s: &str) -> Self::Output {
        s.parse::<i32>().unwrap()
    }
}


#[derive(Debug, Clone)]
pub struct DiagnosticReport{
    pub chars: Vec<char>,
    pub original: String,
}

impl Parseable for DiagnosticReport {
    type Output = DiagnosticReport;

    fn parse(s: &str) -> Self::Output {
        let o = s.to_string();
        let v = s.chars().collect();

        DiagnosticReport{
            chars: v,
            original: o,
        }
    }
}

pub fn read_input(name: &str) -> String {
    fs::read_to_string(format!("{}{}.input", INPUT_DIR, name))
        .expect("Something went wrong reading the file")
}

pub fn read_input_vector<T: Parseable<Output = T>>(name: &str) -> Vec<T> {
    read_input(name).split('\n').into_iter().map(|x| T::parse(x)).collect()
}
