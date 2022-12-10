use crate::util;

const COMMAND: &str = "$";
const DIR: &str = "dir";

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
                        directories.push(cmd_args[2]);
                    }
                }
                "ls" => in_ls = true,
                _ => unreachable!(),
            }
            continue;
        }

        if in_ls {
            if line.starts_with(DIR) {
                continue;
            }
            let (f_size, _) = line.split_once(' ').unwrap();
            let u_size = unum!(f_size);

            let mut parents = Vec::new();
            for d in &directories {
                parents.push(d.to_string());
                let curr_dir = parents.join("/");
                hm_inc!(dir_sizes, curr_dir, u_size);
            }
        }
    }

    let mut total = 0;
    for s in dir_sizes.values() {
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
                        directories.push(cmd_args[2]);
                    }
                }
                "ls" => in_ls = true,
                _ => unreachable!(),
            }
            continue;
        }

        if in_ls {
            if line.starts_with(DIR) {
                continue;
            }
            let (f_size, _) = line.split_once(' ').unwrap();
            let u_size = unum!(f_size);

            let mut parents = Vec::new();
            for d in &directories {
                parents.push(d.to_string());
                let curr_dir = parents.join("/");
                hm_inc!(dir_sizes, curr_dir, u_size);
            }
        }
    }

    let total = dir_sizes.get("/").unwrap();
    let unusused_space = 70000000 - total;
    let space_needed = 30000000 - unusused_space;
    let mut candidates = Vec::new();

    for s in dir_sizes.values() {
        if *s >= space_needed {
            candidates.push(*s);
        }
    }
    *candidates.iter().min().unwrap()
}

run!();
