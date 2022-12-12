use crate::util;
use griddy::Grid;
use std::collections::HashMap;

const INDEXING: &str = "abcdefghijklmnopqrstuvwxyz";

fn part_1() -> i64 {
    let data = util::read_input("12_test.input");
    let grid = Grid::from_2d_unchecked(
        data
            .lines()
            .map(|line| line
                .chars()
                .map(parse_elevation)
                .collect::<Vec<i64>>()
            ).collect::<Vec<Vec<i64>>>());

    hm!(seen, (usize, usize), i64);
    for (row_idx, row) in grid.rows().enumerate() {
        for col in 0..row.len() {
            if row[col] == 0 {
                println!("starting from {}, {}", row_idx, col);
                seen.insert((row_idx, col), 0);
                let (_, val) = recurse(&grid, &mut seen, row_idx, col, 0);
                return val;
            }
        }
    }
    0
}

fn part_2() -> i64 {
    log!("Ran part 2");
    0
}

run!();

fn parse_elevation(c: char) -> i64 {
    match c {
        'S' => 0,
        'E' => 27,
        _ => (INDEXING.find(c).unwrap() + 1) as i64,
    }
}

fn recurse(grid: &Grid<i64>, seen: &mut HashMap<(usize, usize), i64>, row: usize, col: usize, steps: i64) -> (bool, i64) {
    if row >= grid.rows_len() || col >= grid.cols_len() {
        return (false, i64::MAX);
    }

    let me = grid[row][col];
    if me == 27 {
        // println!("here: {}", steps);
        return (true, steps);
    }

    let mut min_steps = i64::MAX;
    let mut found = false;
    for v in grid.row_neighbors(row, col) {
        if seen.get(&v).is_some() {
            continue;
        }

        let next_step = grid[v.0][v.1];
        let can_proceed = if next_step <= me {
            true
        } else {
            if (next_step - me).abs() == 1 {
                true
            } else {
                false
            }
        };
        if can_proceed {
            seen.insert(v, steps + 1);
            let (f, s) = recurse(grid, seen, v.0, v.1, steps + 1);
            seen.remove(&v);
            if f {
                found = f;
                if s < min_steps {
                    min_steps = s;
                }
            }
        }
    }

    for v in grid.col_neighbors(row, col) {
        if seen.get(&v).is_some() {
            continue;
        }
        let next_step = grid[v.0][v.1];
        let can_proceed = if next_step <= me {
            true
        } else {
            if (next_step - me).abs() == 1 {
                true
            } else {
                false
            }
        };
        if can_proceed {
            seen.insert(v, steps + 1);
            let (f, s) = recurse(grid, seen, v.0, v.1, steps + 1);
            seen.remove(&v);
            if f {
                found = f;
                if s < min_steps {
                    min_steps = s;
                }
            }
        }
    }

    (found, min_steps)
}
