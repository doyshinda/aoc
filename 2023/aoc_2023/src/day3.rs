use crate::util;
use griddy::Grid;

const DOT: i32 = -1;
const SYM: i32 = -2;
const GER: i32 = -3;

fn part_1() -> u64 {
    let mut idx = 0;
    hm!(id_to_num, i32, i32);

    let data = util::read_input("3.input");
    let mut grid = vec![];
    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut p_num = 0;
        let mut row = vec![];
        for c in line.chars() {
            if !c.is_ascii_digit() {
                if p_num != 0 {
                    id_to_num.insert(idx, p_num);
                    idx += 1;
                    p_num = 0;
                }
                if c == '.' {
                    row.push(DOT);
                } else {
                    row.push(SYM);
                }
                continue;
            }

            // First digit in a new part number 
            p_num *= 10;
            p_num += c.to_digit(10).unwrap() as i32;
            row.push(idx);
        }
        if p_num != 0 {
            id_to_num.insert(idx, p_num);
            idx += 1;
        }
        grid.push(row);
    }

    let grid = Grid::from_2d_unchecked(grid);
    let mut r_idx = 0;
    let mut sum = 0;
    for r in grid.rows() {

        let mut c_idx = 0;
        for v in r {
            // Dot, nothing to see here
            if *v == DOT {
                c_idx += 1;
                continue;
            }

            // Symbol, check neighbors
            if *v == SYM {
                hs!(p_num_idx, i32);
                let n = grid.neighbors(r_idx, c_idx);
                for (r, c) in &n {
                    let n_val = &grid[*r][*c];
                    if id_to_num.contains_key(n_val) {
                        p_num_idx.insert(*n_val);
                    }
                }

                for p_num in &p_num_idx {
                    sum += id_to_num.get(p_num).unwrap();
                }
            }
            c_idx += 1;
        }
        r_idx += 1;
    }
    
    sum as u64
}

fn part_2() -> u64 {
    let mut idx = 0;
    hm!(id_to_num, i32, i32);

    let data = util::read_input("3.input");
    let mut grid = vec![];
    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }

        let mut p_num = 0;
        let mut row = vec![];
        for c in line.chars() {
            if !c.is_ascii_digit() {
                if p_num != 0 {
                    id_to_num.insert(idx, p_num);
                    idx += 1;
                    p_num = 0;
                }
                let v = match c {
                    '.' => DOT,
                    '*' => GER,
                    _ => SYM,
                };
                row.push(v);
                continue;
            }

            // First digit in a new part number 
            p_num *= 10;
            p_num += c.to_digit(10).unwrap() as i32;
            row.push(idx);
        }
        if p_num != 0 {
            id_to_num.insert(idx, p_num);
            idx += 1;
        }
        grid.push(row);
    }

    let grid = Grid::from_2d_unchecked(grid);
    let mut r_idx = 0;
    let mut sum = 0;
    for r in grid.rows() {
        log!("{:?}", r);

        let mut c_idx = 0;
        for v in r {
            // Dot, nothing to see here
            if *v == DOT {
                c_idx += 1;
                continue;
            }

            // Gear, check neighbors
            if *v == GER {
                hs!(p_num_idx, i32);
                let n = grid.neighbors(r_idx, c_idx);
                for (r, c) in &n {
                    let n_val = &grid[*r][*c];
                    if id_to_num.contains_key(n_val) {
                        p_num_idx.insert(*n_val);
                    }
                }
                // dbg!(&p_num_idx);
                if p_num_idx.len() == 2 {
                    let mut ratio = 1;
                    for p in &p_num_idx {
                        ratio *= id_to_num.get(p).unwrap();
                    }
                    sum += ratio;
                }
            }
            c_idx += 1;
        }
        r_idx += 1;
    }
    
    sum as u64
}

run!();
