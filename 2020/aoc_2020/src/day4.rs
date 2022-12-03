use crate::util;
use regex::Regex;

fn part_1() -> u64 {
    let data = util::read_input("4.input");
    let passports: Vec<_> = data.split("\n\n").map(|passport| passport.replace('\n', " ").trim_end().to_string()).collect();
    let mut valid = 0;
    for p in passports {
        let field_count: u64 = p.split_ascii_whitespace().map(|field| {
            let (name, _) = field.split_once(':').unwrap();
            if name == "cid" {
                0
            } else {
               1
            }
        }).sum();

        if field_count >= 7 {
            valid += 1;
        }
    }
    valid
}

fn part_2() -> u64 {
    let data = util::read_input("4.input");
    let passports: Vec<_> = data.split("\n\n").map(|passport| passport.replace('\n', " ").trim_end().to_string()).collect();
    let mut valid = 0;
    for p in passports {

        let field_count: u64 = p.split_ascii_whitespace().map(|field| {
            let (name, value) = field.split_once(':').unwrap();
            if name == "cid" {
                0
            } else {
               is_valid(name, value)
            }
        }).sum();

        if field_count >= 7 {
            valid += 1;
        }
    }
    valid
}

run!();

fn is_valid(name: &str, value: &str) -> u64 {
    match name {
        "byr" => valid_byr(value),
        "iyr" => valid_iyr(value),
        "eyr" => valid_eyr(value),
        "hgt" => valid_hgt(value),
        "hcl" => valid_hcl(value),
        "ecl" => valid_ecl(value),
        "pid" => valid_pid(value),
        _ => 0,
    }
}

fn valid_byr(value: &str) -> u64 {
    if value.len() != 4 {
        return 0;
    }

    let n = unum!(value);
    if n >= 1920 && n <= 2002 {
        return 1;
    }
    0
}

fn valid_iyr(value: &str) -> u64 {
    if value.len() != 4 {
        return 0;
    }

    let n = unum!(value);
    if n >= 2010 && n <= 2020 {
        return 1;
    }
    0
}

fn valid_eyr(value: &str) -> u64 {
    if value.len() != 4 {
        return 0;
    }

    let n = unum!(value);
    if n >= 2020 && n <= 2030 {
        return 1;
    }
    0
}

fn valid_hgt(value: &str) -> u64 {
    if value.ends_with("cm") {
        let r = value.replace("cm", "");
        let n = unum!(r);
        if n >= 150 && n <= 193 {
            return 1;
        }

        return 0;
    }

    if value.ends_with("in") {
        let r = value.replace("in", "");
        let n = unum!(r);
        if n >= 59 && n <= 76 {
            return 1;
        }

        return 0;
    }

    return 0;
}

fn valid_hcl(value: &str) -> u64 {
    // a # followed by exactly six characters 0-9 or a-f.
    let re = Regex::new(r"^#[0-9abcdef]{6}").unwrap();
    if re.is_match(value) {
        return 1;
    }
    0
}

fn valid_ecl(value: &str) -> u64 {
    match value {
        "amb" | "blu"| "brn"| "gry"| "grn"| "hzl"| "oth" => 1,
        _ => 0,
    }
}

fn valid_pid(value: &str) -> u64 {
    if value.len() == 9 {
        match value.parse::<u64>() {
            Ok(_) => return 1,
            _ => return 0,
        }
    }

    return 0;
}
