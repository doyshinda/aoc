use crate::util;

fn part_1() -> i32 {
    let data: Vec<i32> = util::read_input_vector("1_a");
    let mut last = data[0];
    let mut count = 0;
    for d in &data[1..] {
        if d > &last {
            count += 1;
        }
        last = *d;
    }
    count
}

fn part_2() -> i32 {
    let data: Vec<i32> = util::read_input_vector("1_a");
    if data.len() < 3 {
        println!("0");
        return 0
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
    count
}

pub fn run() {
    let answer = part_1();
    println!("part_1 {:?}", answer);
    let answer = part_2();
    println!("part_2 {:?}", answer);
}
