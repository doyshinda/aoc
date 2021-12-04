use crate::util;

fn part_1() -> i32 {
    let changes = util::read_input_vector("2_a");
    let mut forward = 0;
    let mut depth = 0;
    for c in changes {
        match c {
            util::SubVectorChange::Forward(i) => forward += i,
            util::SubVectorChange::Down(i) => depth += i,
            util::SubVectorChange::Up(i) => depth -= i,
        }
    }

    forward * depth
}

fn part_2() -> i32 {
    let changes = util::read_input_vector("2_a");
    let mut forward = 0;
    let mut depth = 0;
    let mut aim = 0;
    for c in changes {
        match c {
            util::SubVectorChange::Forward(i) => {
                forward += i;
                depth += aim * i;
            },
            util::SubVectorChange::Down(i) => aim += i,
            util::SubVectorChange::Up(i) => aim -= i,
        }
    }

    forward * depth
}

pub fn run() {
    let answer = part_1();
    println!("part_1 {:?}", answer);
    let answer = part_2();
    println!("part_2 {:?}", answer);
}
