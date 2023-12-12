use crate::util;
use griddy::Grid;
use std::collections::HashMap;

const GALAXY: char = '#';

fn part_1() -> u64 {
    calc_distance(2)
}

fn part_2() -> u64 {
    calc_distance(1000000)
}

fn calc_distance(s: usize) -> u64 {
    let data = util::read_input("11.input");
    let grid: Vec<Vec<char>> = data.split("\n").map(|l| l.chars().collect()).collect();
    let mut grid = Grid::from_2d_unchecked(grid);

    let (x_vals, y_vals) = expand2(&mut grid, s);
    hm!(map, (usize, usize), usize);
    let mut pairs = vec![];
    let mut x = 0;
    for r in grid.rows() {
        let mut y = 0;
        for c in r {
            if *c == GALAXY {
                map.insert((x, y), pairs.len());
                pairs.push((x, y));
            }
            y += 1;
        }
        x += 1;
    }
    
    let mut distances = vec![];
    hs!(seen, String);
    let expected_pairs = (pairs.len() * (pairs.len() - 1)) / 2;

    while distances.len() < expected_pairs {
        let g = pairs.pop().unwrap();
        for p in &pairs {
            let a = map[&g];
            let b = map[p];
            let key = if a >= b {
                format!("{}_{}", a, b)
            } else {
                format!("{}_{}", b, a)
            };
            if seen.contains(&key) {
                continue;
            }

            seen.insert(key);

            let x1_scale = x_vals[&g.0];
            let x2_scale = x_vals[&p.0];
            let y1_scale = y_vals[&g.1];
            let y2_scale = y_vals[&p.1];
            distances.push(x1_scale.abs_diff(x2_scale) + y1_scale.abs_diff(y2_scale));
        }
        pairs.insert(0, g);
    }

    distances.iter().sum::<usize>() as u64
}

run!();

fn expand2(grid: &mut Grid<char>, s: usize) -> (HashMap<usize, usize>, HashMap<usize, usize>) {
    let s = s - 1;
    let mut idx = 0;
    let mut scale = 0;
    hm!(x_vals, usize, usize);
    for r in grid.rows() {
        if r.iter().all(|x| *x == '.') {
            x_vals.insert(idx, idx + scale);
            if scale == 0 {
                scale = s;
            } else {
                scale = scale + s;
            }
        } else {
            x_vals.insert(idx, idx + scale);
        }
        idx += 1;
    }

    scale = 0;
    hm!(y_vals, usize, usize);
    for c in 0..grid.cols_len() {
        let mut cols = vec![];
        for r in 0..grid.rows_len() {
            cols.push(grid[r][c]);
        }
        if cols.iter().all(|x| *x == '.') {
            y_vals.insert(c, c + scale);
            if scale == 0 {
                scale = s;
            } else {
                scale = scale + s;
            }
        } else {
            y_vals.insert(c, c + scale);
        }
    }

    (x_vals, y_vals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand2() {
        let mut grid = Grid::from_2d_unchecked(vec![
            ['#', '.', '.', '#'].to_vec(),
            ['.', '.', '.', '.'].to_vec(),
            ['.', '.', '#', '#'].to_vec(),
            ['.', '.', '#', '#'].to_vec(),
            ['.', '.', '.', '.'].to_vec(),
            ['.', '.', '#', '.'].to_vec(),
        ]);

        let (x, y) = expand2(&mut grid, 10);
        assert_eq!(x[&0], 0);
        assert_eq!(x[&1], 1);
        assert_eq!(x[&2], 11);
        assert_eq!(x[&3], 12);
        assert_eq!(x[&4], 13);
        assert_eq!(x[&5], 23);
        assert_eq!(y[&0], 0);
        assert_eq!(y[&1], 1);
        assert_eq!(y[&2], 11);
        assert_eq!(y[&3], 12);
    }
}
