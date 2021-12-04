#![allow(dead_code)]

mod util;
mod day4;

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

fn three_a() {
    let bin_values: Vec<util::DiagnosticReport> = util::read_input_vector("3_a");
    let bin_len = bin_values.len();
    let diag_len = bin_values[0].chars.len();
    let mut radix = String::with_capacity(diag_len);

    for i in 0..diag_len {
        let f0_count = bin_values.iter().filter(|x| x.chars[i] == '0').count();
        match f0_count > bin_len - f0_count {
            true => radix.push('0'),
            _ => radix.push('1'),
        }
    }

    let ival = i32::from_str_radix(&radix, 2).unwrap();
    let mask_str = "1".repeat(diag_len);
    let mask = i32::from_str_radix(&mask_str, 2).unwrap();
    println!("answer: {:?}", ival * (ival ^ mask));
}

fn three_b() {
    let mut bin_values: Vec<util::DiagnosticReport> = util::read_input_vector("3_a");
    let source = bin_values.clone();
    let diag_len = bin_values[0].chars.len();
    let mut oxy = 0;
    for i in 0..diag_len as usize {
        // let f0: Vec<util::DiagnosticReport> = bin_values.clone().into_iter().filter(|x| x.chars[i] == '0').collect();
        // let f1: Vec<util::DiagnosticReport> = bin_values.into_iter().filter(|x| x.chars[i] == '1').collect();
        let mut f0 = Vec::new();
        let mut f1 = Vec::new();
        for b in bin_values.into_iter() {
            match b.chars[i] == '0' {
                true => f0.push(b),
                _ => f1.push(b),
            }
        }
        match f1.len() >= f0.len() {
            true => bin_values = f1,
            _ => bin_values = f0,
        }

        if bin_values.len() == 1 {
            oxy = i32::from_str_radix(&bin_values[0].original, 2).unwrap();
            break
        }
    }

    bin_values = source;
    let mut scrub = 0;
    for i in 0..diag_len as usize {
        // let f0: Vec<util::DiagnosticReport> = bin_values.clone().into_iter().filter(|x| x.chars[i] == '0').collect();
        // let f1: Vec<util::DiagnosticReport> = bin_values.into_iter().filter(|x| x.chars[i] == '1').collect();
        let mut f0 = Vec::new();
        let mut f1 = Vec::new();
        for b in bin_values.into_iter() {
            match b.chars[i] == '0' {
                true => f0.push(b),
                _ => f1.push(b),
            }
        }
        match f0.len() <= f1.len() {
            true => bin_values = f0,
            _ => bin_values = f1,
        }

        if bin_values.len() == 1 {
            scrub = i32::from_str_radix(&bin_values[0].original, 2).unwrap();
            break
        }
    }

    println!("answer: {:?}", oxy * scrub);
}

fn main() {
    // one_a();
    // one_b();
    // two_a();
    // two_b();
    // three_a();
    // three_b();
    day4::run();
}
