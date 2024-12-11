use crate::util;
use griddy::Grid;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("10.input");
    let grid: Vec<Vec<u32>> = data.split("\n").map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);

    solve(grid, true)
}

fn part_2() -> u64 {
    let data = util::read_input("10.input");
    let grid: Vec<Vec<u32>> = data.split("\n").map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);

    solve(grid, false)
}

fn solve(grid: Grid<u32>, unique: bool) -> u64 {
    let mut ans = 0;
    for x in 0..grid.rows_len() {
        for y in 0..grid.cols_len() {
            if grid[x][y] == 0 {
                let mut paths: HashSet<Vec<(usize, usize)>> = HashSet::new();
                let mut dest = HashSet::new();
                util::dfs(&grid, x, y, |curr: &util::Node, neigh: &util::Node| -> bool {
                    let my_val = grid[curr.x][curr.y];

                    let nval = grid[neigh.x][neigh.y];
                    if nval <= my_val {
                        return false;
                    }

                    if nval - my_val != 1 {
                        return false;
                    }

                    if nval == 9 {
                        dest.insert((neigh.x, neigh.y));

                        let mut ans = curr.path.clone();
                        ans.push((neigh.x, neigh.y));
                        paths.insert(ans);

                        return false;
                    }

                    return true;
                });

                if unique {
                    ans += dest.len() as u64
                } else {
                    ans += paths.len() as u64
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
        let data = "0123\n1234\n8765\n9876";
        let grid: Vec<Vec<u32>> = data.split("\n").map(|l| l.chars().map(|x| x.to_digit(10).unwrap()).collect()).collect();
        let grid = Grid::from_2d_unchecked(grid);

        let ans = solve(grid, true);
        assert_eq!(1, ans);
    }
}

