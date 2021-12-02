use std::fs;

const INPUT_DIR: &str = "/home/abe/misc/adventofcode/2021/aoc_21/input/";

#[derive(Debug)]
pub enum SubVectorChange {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl SubVectorChange {
    fn parse(s: &str) -> Self {
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

pub fn read_input(name: &str) -> String {
    let contents = fs::read_to_string(format!("{}{}.input", INPUT_DIR, name))
        .expect("Something went wrong reading the file");

    contents
}

pub fn read_input_vec_i32(name: &str) -> Vec<i32> {
    let contents = read_input(name);
    return contents.split('\n').map(|x| x.parse::<i32>().unwrap()).collect();
}

pub fn read_input_vector_changes(name: &str) -> Vec<SubVectorChange> {
    let contents = read_input(name);
    return contents.split('\n').map(|x| SubVectorChange::parse(x)).collect();
}
