use crate::util;

fn part_1() -> i32 {
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
    ival * (ival ^ mask)
}

fn part_2() -> i32 {
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

    oxy * scrub
}

pub fn run() {
    let answer = part_1();
    println!("part_1 {:?}", answer);
    let answer = part_2();
    println!("part_2 {:?}", answer);
}
