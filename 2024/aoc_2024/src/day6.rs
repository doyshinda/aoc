use crate::util;
use griddy::Grid;
use std::collections::HashSet;

fn part_1() -> u64 {
    let seen = locations("6.input");
    seen.len() as u64
}

fn locations(fname: &str) -> HashSet<(usize, usize)> {
    let data = util::read_input(fname);
    let grid = data.split("\n").map(|l| l.chars().collect()).collect();
    let mut grid = Grid::from_2d_unchecked(grid);

    let (mut xs, mut ys) = grid.find(&'^').unwrap();

    hs!(seen, (usize, usize));
    seen.insert((xs, ys));

    let binding = vec![
        Grid::col_up_coords,
        Grid::row_right_coords,
        Grid::col_down_coords,
        Grid::row_left_coords,
    ];
    let mut funcs = binding.iter().cycle();

    loop {
        let f = funcs.next().unwrap();
        let pos = f(&grid, xs, ys);
        if pos.is_empty() {
            break;
        }

        for (x, y) in pos {
            match grid[x][y] {
                '.' | '^' | 'X' => {
                    xs = x;
                    ys = y;
                    seen.insert((x, y));
                    grid[x][y] = 'X';
                    continue;
                }
                '#' => {
                    break;
                }
                _ => unreachable!(),
            }
        }

        if xs == 0 || xs == grid.rows_len() - 1 || ys == 0 || ys == grid.cols_len() - 1 {
            break;
        }
    }

    seen
}

fn key(vals: &[(usize, usize)], loc: (usize, usize), direction: char) -> String {
    let mut s: String = vals.iter().map(|(x, y)| x.to_string() + &y.to_string()).collect();
    s.push('-');
    s.push_str(&(loc.0).to_string());
    s.push_str(&(loc.1).to_string());
    s.push(direction);

    s
}

fn part_2() -> u64 {
    let fname = "6.input";
    let data = util::read_input(fname);
    let grid = data.split("\n").map(|l| l.chars().collect()).collect();
    let mut grid = Grid::from_2d_unchecked(grid);

    let (oxs, oys) = grid.find(&'^').unwrap();

    let binding = vec![
        ('U', Grid::col_up_coords as fn(&Grid<char>, usize, usize) -> Vec<(usize, usize)>),
        ('R', Grid::row_right_coords),
        ('D', Grid::col_down_coords),
        ('L', Grid::row_left_coords),
    ];

    let mut ans = 0;
    let vlocs = locations(fname);
    hs!(seen, String);

    for (xo, yo) in vlocs {
        if xo == oxs && yo == oys {
            continue;
        }
        log!("checking ({}, {})", xo, yo);
        let mut xs = oxs;
        let mut ys = oys;

        let original = grid[xo][yo];
        grid[xo][yo] = '#';

        let mut funcs = binding.iter().cycle();
        seen.clear();

        'outer: loop {
            let (d, f) = funcs.next().unwrap();
            let pos = f(&grid, xs, ys);
            if pos.is_empty() {
                break;
            }

            let k = key(&pos, (xs, ys), *d);
            if seen.contains(&k) {
                log!("({}, {}) match: {}", xo, yo, k);
                ans += 1;
                break 'outer;
            }
            seen.insert(k);

            for (x, y) in &pos {
                match grid[*x][*y] {
                    '.' | '^' | 'X' => {
                        xs = *x;
                        ys = *y;
                        continue;
                    }
                    '#' => {
                        break;
                    }
                    _ => unreachable!(),
                }
            }

            if xs == 0 || xs == grid.rows_len() - 1 || ys == 0 || ys == grid.cols_len() - 1 {
                break;
            }
        }
        grid[xo][yo] = original;
    }
    ans
}

run!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, 1);
    }
}

fn print(grid: &Grid<char>) {
    println!("{}", "====================================");
    for r in grid.rows() {
        println!("{:?}", r.iter().collect::<String>());
    }
}
