use crate::util;
use griddy::Grid;

fn check_neighbors(grid: &Grid<char>, coords: &[(usize, usize)]) -> u64 {
    if grid[coords[0].0][coords[0].1] == 'M'
        && grid[coords[1].0][coords[1].1] == 'A'
        && grid[coords[2].0][coords[2].1] == 'S'
    {
        return 1;
    }
    0
}

fn check_grid(grid: &Grid<char>, x: usize, y: usize) -> u64 {
    let mut ans = 0;
    let funcs = vec![
        Grid::row_left_coords,
        Grid::row_right_coords,
        Grid::col_up_coords,
        Grid::col_down_coords,
        Grid::diag_right_up,
        Grid::diag_right_down,
        Grid::diag_left_up,
        Grid::diag_left_down,
    ];

    for f in funcs {
        let rlc = f(&grid, x, y);
        if rlc.len() >= 3 {
            ans += check_neighbors(grid, &rlc);
        }
    }

    ans
}

fn part_1() -> u64 {
    let data = util::read_input("4.input");
    let mut ans = 0;
    let grid = data.split("\n").map(|l| l.chars().collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);

    for x in 0..grid.rows_len() {
        for y in 0..grid.cols_len() {
            if grid[x][y] == 'X' {
                ans += check_grid(&grid, x, y);
            }
        }
    }

    ans
}

fn part_2() -> u64 {
    let data = util::read_input("4.input");
    let mut ans = 0;
    let grid = data.split("\n").map(|l| l.chars().collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);

    for x in 0..grid.rows_len() {
        for y in 0..grid.cols_len() {
            hm!(map, char, usize);
            if grid[x][y] == 'A' {
                let diag = grid.diag_neighbors(x, y);
                for (a, b) in &diag {
                    hm_inc!(map, grid[*a][*b], 1);
                }
                let mut mcount = 0;
                let mut scount = 0;
                if let Some(count) = map.get(&'M') {
                    mcount = *count;
                }
                if let Some(count) = map.get(&'S') {
                    scount = *count;
                }

                if scount == 2 && mcount == 2 {
                    let a = grid[diag[0].0][diag[0].1];
                    let b = grid[diag[3].0][diag[3].1];
                    if a != b {
                        ans += 1;
                    }
                }
            }
        }
    }

    ans
}

run!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = util::read_input("4_test.input");
        let mut ans = 0;
        let grid = data.split("\n").map(|l| l.chars().collect()).collect();
        let grid = Grid::from_2d_unchecked(grid);

        let coords = [(1, 5), (2, 6), (3, 7)];
        ans = check_neighbors(&grid, &coords);
        assert_eq!(1, ans);
    }
}
