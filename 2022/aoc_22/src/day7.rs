use crate::util;

const COMMAND: &'static str = "$";
const DIR: &'static str = "dir";

fn part_1() -> u64 {
    let data = util::read_input("7.input");
    let mut directories = Vec::new();
    hm!(dir_sizes, String, u64);
    let mut in_ls = false;
    for line in data.lines() {
        if line.starts_with(COMMAND) {
            let cmd_args: Vec<_> = line.split_ascii_whitespace().collect();
            match cmd_args[1] {
                "cd" => {
                    in_ls = false;
                    if cmd_args[2] == ".." {
                        directories.pop().unwrap();
                    } else {
                        directories.push(cmd_args[2].clone());
                    }
                }
                "ls" => in_ls = true,
                _ => unreachable!(),
            }
            continue
        }

        if in_ls {
            if line.starts_with(DIR) {
                continue;
            }
            let (f_size, _) = line.split_once(" ").unwrap();
            let u_size = unum!(f_size);

            let mut parents = Vec::new();
            for d in &directories {
                parents.push(d.to_string());
                let curr_dir = parents.join("/");
                dir_sizes.entry(curr_dir).and_modify(|c| *c += u_size).or_insert(u_size);

            }
        }
    }

    let mut total = 0;
    for (_, s) in &dir_sizes{
        if *s <= 100000 {
            total += *s;
        }
    }
    total
}

fn part_2() -> u64 {
    let data = util::read_input("7.input");
    let mut directories = Vec::new();
    hm!(dir_sizes, String, u64);
    let mut in_ls = false;
    for line in data.lines() {
        if line.starts_with(COMMAND) {
            let cmd_args: Vec<_> = line.split_ascii_whitespace().collect();
            match cmd_args[1] {
                "cd" => {
                    in_ls = false;
                    if cmd_args[2] == ".." {
                        directories.pop().unwrap();
                    } else {
                        directories.push(cmd_args[2].clone());
                    }
                }
                "ls" => in_ls = true,
                _ => unreachable!(),
            }
            continue
        }

        if in_ls {
            if line.starts_with(DIR) {
                continue;
            }
            let (f_size, _) = line.split_once(" ").unwrap();
            let u_size = unum!(f_size);

            let mut parents = Vec::new();
            for d in &directories {
                parents.push(d.to_string());
                let curr_dir = parents.join("/");
                dir_sizes.entry(curr_dir).and_modify(|c| *c += u_size).or_insert(u_size);

            }
        }
    }

    let total = dir_sizes.get("/").unwrap();
    let unusused_space = 70000000 - total;
    let space_needed = 30000000 - unusused_space;
    let mut candidates = Vec::new();

    for (_, s) in &dir_sizes{
        if *s >= space_needed {
            candidates.push(*s);
        }
    }
    *candidates.iter().min().unwrap()
}

run!();
