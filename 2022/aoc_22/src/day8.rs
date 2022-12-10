use crate::util;
use griddy::Grid;

fn part_1() -> u64 {
    let data = util::read_input("8.input");
    let grid = Grid::from_2d_unchecked(
        data.lines()
            .map(|line| {
                line.chars()
                    .map(|c| unum!(c.to_string()))
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>(),
    );

    let mut visible = 0;

    for row_idx in 0..grid.rows_len() {
        for col_idx in 0..grid.cols_len() {
            if am_visible_row(&grid, row_idx, col_idx) {
                visible += 1;
                continue;
            }

            if am_visible_col(&grid, col_idx, row_idx) {
                visible += 1;
                continue;
            }
        }
    }
    visible as u64
}

fn part_2() -> u64 {
    let data = util::read_input("8.input");
    let grid = Grid::from_2d_unchecked(
        data.lines()
            .map(|line| {
                line.chars()
                    .map(|c| unum!(c.to_string()))
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>(),
    );

    let mut score = 0;
    for row_idx in 0..grid.rows_len() {
        for col_idx in 0..grid.cols_len() {
            let this_score = get_score(&grid, row_idx, col_idx);
            if this_score > score {
                score = this_score;
            }
        }
    }

    score
}

run!();

fn get_score(grid: &Grid<u64>, ridx: usize, cidx: usize) -> u64 {
    let mut score = score_up(grid, ridx, cidx);
    score *= score_down(grid, ridx, cidx);
    score *= score_left(grid, ridx, cidx);
    score *= score_right(grid, ridx, cidx);
    score
}

fn score_up(grid: &Grid<u64>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    for (r, c) in grid.col_up_coords(ridx, cidx).iter().rev() {
        score += 1;
        if grid[*r][*c] >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn score_down(grid: &Grid<u64>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    for (r, c) in grid.col_down_coords(ridx, cidx).iter() {
        score += 1;
        if grid[*r][*c] >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn score_left(grid: &Grid<u64>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    for (r, c) in grid.row_left_coords(ridx, cidx).iter().rev() {
        score += 1;
        if grid[*r][*c] >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn score_right(grid: &Grid<u64>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    for (r, c) in grid.row_right_coords(ridx, cidx).iter() {
        score += 1;
        if grid[*r][*c] >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn am_visible_col(grid: &Grid<u64>, col_idx: usize, row_idx: usize) -> bool {
    if row_idx == 0 || row_idx == grid.rows_len() - 1 {
        return true;
    }
    let me = grid[row_idx][col_idx];
    // Check up
    let mut am_visible = true;
    for (r, c) in grid.col_up_coords(row_idx, col_idx) {
        if grid[r][c] >= me {
            am_visible = false;
            break;
        }
    }

    if am_visible {
        return am_visible;
    }

    // Check down
    am_visible = true;
    for (r, c) in grid.col_down_coords(row_idx, col_idx) {
        if grid[r][c] >= me {
            am_visible = false;
            break;
        }
    }

    am_visible
}

fn am_visible_row(grid: &Grid<u64>, row_idx: usize, col_idx: usize) -> bool {
    let me = grid[row_idx][col_idx];
    let mut am_visible = true;

    // Check left
    for (r, c) in grid.row_left_coords(row_idx, col_idx) {
        if grid[r][c] >= me {
            am_visible = false;
            break;
        }
    }
    if am_visible {
        return am_visible;
    }

    // Check right
    am_visible = true;
    for (r, c) in grid.row_right_coords(row_idx, col_idx) {
        if grid[r][c] >= me {
            am_visible = false;
            break;
        }
    }

    am_visible
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_am_visible_col() {
        let grid = Grid::from_2d_unchecked(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        assert_eq!(false, am_visible_col(&grid, 0, 1));
        assert_eq!(true, am_visible_col(&grid, 1, 1));
        assert_eq!(true, am_visible_col(&grid, 2, 1));
        assert_eq!(false, am_visible_col(&grid, 3, 1));
        assert_eq!(false, am_visible_col(&grid, 4, 1));

        assert_eq!(true, am_visible_col(&grid, 1, 0))
    }

    #[test]
    fn test_am_visible_row() {
        let grid = Grid::from_2d_unchecked(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        assert_eq!(true, am_visible_row(&grid, 1, 0));
        assert_eq!(true, am_visible_row(&grid, 1, 1));
        assert_eq!(true, am_visible_row(&grid, 1, 2));
        assert_eq!(false, am_visible_row(&grid, 1, 3));
        assert_eq!(true, am_visible_row(&grid, 1, 4));
    }

    #[test]
    fn test_score() {
        let grid = Grid::from_2d_unchecked(vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ]);
        assert_eq!(1, score_up(&grid, 1, 2));
        assert_eq!(2, score_down(&grid, 1, 2));
        assert_eq!(1, score_left(&grid, 1, 2));
        assert_eq!(2, score_right(&grid, 1, 2));
        assert_eq!(4, get_score(&grid, 1, 2));
    }
}
