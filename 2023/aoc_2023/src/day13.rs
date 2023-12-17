use crate::util;
use griddy::Grid;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("13.input");
    let mut grids = vec![];
    for map in data.split("\n\n") {
        let grid = map.split("\n").map(|l| l.chars().collect()).collect();
        let grid = Grid::from_2d_unchecked(grid);
        grids.push(grid);
    }

    let mut total = 0;
    let mut cols_left = 0;
    for t in &grids {
        let g = t.transpose();
        cols_left += calc_reflection(&g);
    }
    total += cols_left;

    let mut rows_above = 0;
    for g in grids {
        rows_above += calc_reflection(&g);
    }
    total += rows_above * 100;
    total as u64    
}

fn part_2() -> u64 {
    let data = util::read_input("13.input");
    let mut grids = vec![];
    for map in data.split("\n\n") {
        let grid = map.split("\n").map(|l| l.chars().collect()).collect();
        let grid = Grid::from_2d_unchecked(grid);
        grids.push(grid);
    }

    let mut total = 0;
    let mut cols_left = 0;
    for t in &grids {
        let mut g = t.transpose();
        hs!(old_cols, (usize, usize));
        calc_reflection2(&g, &mut old_cols);

        for x in 0..g.rows_len() {
            for y in 0..g.cols_len() {
                let old = g[x][y];
                if old == '.' {
                    g[x][y] = '#';
                } else {
                    g[x][y] = '.';
                }
                cols_left += calc_reflection2(&g, &mut old_cols);
                g[x][y] = old;
            }
        }
    }

    total += cols_left;

    let mut rows_above = 0;
    for g in &mut grids {
        hs!(old_rows, (usize, usize));
        calc_reflection2(g, &mut old_rows);

        for x in 0..g.rows_len() {
            for y in 0..g.cols_len() {
                let old = g[x][y];
                if old == '.' {
                    g[x][y] = '#';
                } else {
                    g[x][y] = '.';
                }

                rows_above += calc_reflection2(&g, &mut old_rows);
                g[x][y] = old;
            }
        }
    }

    total += rows_above * 100;
    total as u64
}

run!();

fn calc_reflection(g: &Grid<char>) -> u64 {
    let mut cols_left = 0;
    let rl = g.rows_len();
    for (i, r) in g.rows().enumerate().skip(1) {
        if g[i - 1] == *r {
            let (ls, le, rs, re) = calc_spacing(i, rl);
            let mut reflects = true;
            for (a, b) in std::iter::zip((ls..le).rev(), rs..re) {
                if g[a] != g[b] {
                    reflects = false;
                    break;
                }
            }

            if reflects {
                cols_left += i;
            }
        }
    }

    cols_left as u64
}

fn calc_reflection2(g: &Grid<char>, ridxs: &mut HashSet<(usize, usize)>) -> u64 {
    let mut cols_left = 0;
    let rl = g.rows_len();
    for (i, r) in g.rows().enumerate().skip(1) {
        if ridxs.contains(&(i - 1, i)) {
            continue;
        }
        if g[i - 1] == *r {
            let (ls, le, rs, re) = calc_spacing(i, rl);
            let mut reflects = true;
            for (a, b) in std::iter::zip((ls..le).rev(), rs..re) {
                if g[a] != g[b] {
                    reflects = false;
                    break;
                }
            }

            if reflects {
                cols_left += i;
                ridxs.insert((i - 1, i));
            }
        }
    }

    cols_left as u64
}

fn calc_spacing(i: usize, rl: usize) -> (usize, usize, usize, usize) {
    let spacing = std::cmp::min(i - 1, rl - i - 1);
    let le = i - 1;
    let ls = le - spacing;
    let rs = i + 1;
    let re = rs + spacing;

    (ls, le, rs, re)
}
