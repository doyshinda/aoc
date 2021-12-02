#![allow(dead_code)]

mod util;

fn one_a() {
    let data: Vec<i32> = util::read_input_vector("1_a");
    let mut last = data[0];
    let mut count = 0;
    for d in &data[1..] {
        if d > &last {
            count += 1;
        }
        last = *d;
    }
    println!("count: {:?}", count);
}

fn one_b() {
    let data: Vec<i32> = util::read_input_vector("1_a");
    if data.len() < 3 {
        print!("0");
        return
    }

    let data_len = data.len();
    let mut last = 0;
    let mut count = 0;
    let mut idx = 0;
    for d in &data {
        if idx + 2 >= data_len {
            break
        }
        let temp = *d + data[idx + 1] + data[idx + 2];
        if last > 0 {
            if temp > last {
                count += 1;
            }
        }
        last = temp;
        idx += 1;
    }
    println!("count: {:?}", count);
}

fn two_a() {
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

    println!("answer: {}", forward * depth);
}

fn two_b() {
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

    println!("answer: {}", forward * depth);
}

fn main() {
    // one_a();
    // one_b();
    // two_a();
    two_b();
}
