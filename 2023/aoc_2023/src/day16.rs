use crate::util;
use griddy::Grid;

const EMPTY: char = '.';
const V_SPLIT: char = '|';
const H_SPLIT: char = '-';
const L_SLANT: char = '\\';
const R_SLANT: char = '/';

fn part_1() -> u64 {
    let data = util::read_input("16.input");
    let grid = data.trim().split("\n").map(|l| l.chars().collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);
    calc_energized(&grid, (0, 0, 'r'))
}

fn part_2() -> u64 {
    let data = util::read_input("16.input");
    let grid = data.trim().split("\n").map(|l| l.chars().collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);
    let mut max = 0;
    for r in 0..grid.rows_len() {
        let e = calc_energized(&grid, (r, 0, 'r'));
        if e > max {
            max = e;
        }
        let e = calc_energized(&grid, (r, grid.cols_len() - 1, 'l'));
        if e > max {
            max = e;
        }
    }

    for c in 0..grid.cols_len() {
        let e = calc_energized(&grid, (0, c, 'd'));
        if e > max {
            max = e;
        }
        let e = calc_energized(&grid, (grid.rows_len() - 1, c, 'u'));
        if e > max {
            max = e;
        }
    }

    max
}

run!();

fn print(g: &Grid<char>) {
    for r in g.rows() {
        println!("{:?}", r.iter().collect::<String>());
    }
}

fn calc_energized(grid: &Grid<char>, start: (usize, usize, char)) -> u64 {
    hs!(energized, (usize, usize));

    let mut next = vec![];
    next.push(start);

    let vert_up = |x, y, next: &mut Vec<(usize, usize, char)>| {
        let un = grid.col_up_coords(x, y);
        if !un.is_empty() {
            let (unx, uny) = un.last().unwrap();
            next.push((*unx, *uny, 'u'));
        }
    };

    let vert_down = |x, y, next: &mut Vec<(usize, usize, char)>| {
        let dn = grid.col_down_coords(x, y);
        if !dn.is_empty() {
            let (unx, uny) = dn[0];
            next.push((unx, uny, 'd'));
        }
    };  

    let vert_calc = |x, y, next: &mut Vec<(usize, usize, char)>| {
        vert_up(x, y, next);
        vert_down(x, y, next);
    };

    let left = |x, y, next: &mut Vec<(usize, usize, char)>| {
        let rn = grid.row_left_coords(x, y);
        if !rn.is_empty() {
            let (rnx, rny) = rn.last().unwrap();
            next.push((*rnx, *rny, 'l'));
        }
    };

    let right = |x, y, next: &mut Vec<(usize, usize, char)>| {
        let rn = grid.row_right_coords(x, y);
        if !rn.is_empty() {
            let (rnx, rny) = rn[0];
            next.push((rnx, rny, 'r'));
        }
    };

    let horz_calc = |x, y, next: &mut Vec<(usize, usize, char)>| {
        left(x, y, next);
        right(x, y, next);
    };

    let mut idx = 0;
    hs!(seen, (usize, usize, char));
    while !next.is_empty() {
        idx += 1;
        let key = next.pop().unwrap();
        if seen.contains(&key) {
            continue;
        }

        seen.insert(key);
        let (x, y, d) = key;

        if x >= grid.rows_len() || y >= grid.cols_len() {
            continue;
        }

        // println!("{} {} {}", x, y, d);

        energized.insert((x, y));
        let c = grid[x][y];

        if d == 'r' {
            if c == EMPTY || c == H_SPLIT {
                next.push((x, y + 1, d));
            }

            if c == V_SPLIT {
                vert_calc(x, y, &mut next);
            }

            if c == R_SLANT {
                vert_up(x, y, &mut next);
            }

            if c == L_SLANT {
                vert_down(x, y, &mut next);
            }
        }

        if d == 'u' {
            if c == EMPTY || c == V_SPLIT {
                if let Some(nx) = x.checked_sub(1) {
                    next.push((nx, y, d));
                }
            }

            if c == H_SPLIT {
                horz_calc(x, y, &mut next);
            }

            if c == L_SLANT {
                left(x, y, &mut next);
            }

            if c == R_SLANT {
                right(x, y, &mut next);
            }
        }

        if d == 'l' {
            if c == H_SPLIT || c == EMPTY {
                if let Some(ny) = y.checked_sub(1) {
                    next.push((x, ny, d));
                }
            }
            if c == V_SPLIT {
                vert_calc(x, y, &mut next);
            }
            if c == L_SLANT {
                vert_up(x, y, &mut next);
            }
            if c == R_SLANT {
                vert_down(x, y, &mut next);
            }
        }

        if d == 'd' {
            if c == V_SPLIT || c == EMPTY {
                next.push((x + 1, y, d));
            }
            if c == H_SPLIT {
                horz_calc(x, y, &mut next);
            }
            if c == L_SLANT {
                right(x, y, &mut next);
            }
            if c == R_SLANT {
                left(x, y, &mut next);
            }
        }
    }

    energized.len() as u64
}
