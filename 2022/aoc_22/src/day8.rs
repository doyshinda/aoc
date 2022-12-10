use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("8.input");
    let grid: Vec<Vec<u64>> = data.lines()
        .map(|line| line.chars().map(|c| unum!(c.to_string())).collect::<Vec<u64>>())
        .collect();

    let mut visible = 0;

    for (row_idx, row) in grid.iter().enumerate() {
        for col_idx in 0..row.len() {
            if am_visible_row(row, col_idx) {
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
    let grid: Vec<Vec<u64>> = data.lines()
        .map(|line| line.chars().map(|c| unum!(c.to_string())).collect::<Vec<u64>>())
        .collect();

    let mut score = 0;
    for (row_idx, row) in grid.iter().enumerate() {
        for col_idx in 0..row.len() {
            let this_score = get_score(&grid, row_idx, col_idx);
            if this_score > score {
                score = this_score;
            }
        }
    }

    score
}

run!();

fn get_score(grid: &Vec<Vec<u64>>, ridx: usize, cidx: usize) -> u64 {
    let mut score = score_up(grid, ridx, cidx);
    score *= score_down(grid, ridx, cidx);
    score *= score_left(grid, ridx, cidx);
    score *= score_right(grid, ridx, cidx);
    score
}

fn score_up(grid: &Vec<Vec<u64>>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    for row in grid[..ridx].iter().rev() {
        score += 1;
        if row[cidx] >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn score_down(grid: &Vec<Vec<u64>>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    for row in grid[ridx+1..].iter() {
        score += 1;
        if row[cidx] >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn score_left(grid: &Vec<Vec<u64>>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    let row = &grid[ridx];
    for col in row[..cidx].iter().rev() {
        score += 1;
        if *col >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn score_right(grid: &Vec<Vec<u64>>, ridx: usize, cidx: usize) -> u64 {
    let mut score = 0;
    let row = &grid[ridx];
    for col in row[cidx+1..].iter() {
        score += 1;
        if *col >= grid[ridx][cidx] {
            break;
        }
    }
    score
}

fn am_visible_col(grid: &Vec<Vec<u64>>, me_col_idx: usize, me_row_idx: usize) -> bool {
    if me_row_idx == 0 || me_row_idx == grid.len() - 1 {
        return true;
    }
    let me = grid[me_row_idx][me_col_idx];
    // Check up
    let mut am_visible = true;
    for row in grid[..me_row_idx].iter() {
        if row[me_col_idx] >= me {
            am_visible = false;
            break;
        }
    }

    if am_visible {
        return am_visible;
    }

    // Check down
    am_visible = true;
    for row in grid[me_row_idx+1..].iter() {
        if row[me_col_idx] >= me {
            am_visible = false;
            break;
        }
    }

    return am_visible;
}

fn am_visible_row(row: &Vec<u64>, me_col_idx: usize) -> bool {
    let me = row[me_col_idx];
    let mut am_visible = true;

    // Check left
    for col in row[..me_col_idx].iter() {
        if *col >= me {
            am_visible = false;
            break;
        }
    }
    if am_visible {
        return am_visible;
    }

    // Check right
    am_visible = true;
    for col in row[me_col_idx+1..].iter() {
        if *col >= me {
            am_visible = false;
        }
    }

    return am_visible;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_am_visible_col() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(false, am_visible_col(&grid, 0, 1));
        assert_eq!(true, am_visible_col(&grid, 1, 1));
        assert_eq!(true, am_visible_col(&grid, 2, 1));
        assert_eq!(false, am_visible_col(&grid, 3, 1));
        assert_eq!(false, am_visible_col(&grid, 4, 1));

        assert_eq!(true, am_visible_col(&grid, 1, 0))
    }

    #[test]
    fn test_am_visible_row() {
        let row = vec![2, 5, 5, 1, 2];
        assert_eq!(true, am_visible_row(&row, 0));
        assert_eq!(true, am_visible_row(&row, 1));
        assert_eq!(true, am_visible_row(&row, 2));
        assert_eq!(false, am_visible_row(&row, 3));
        assert_eq!(true, am_visible_row(&row, 4));
    }

    #[test]
    fn test_score() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];
        assert_eq!(1, score_up(&grid, 1, 2));
        assert_eq!(2, score_down(&grid, 1, 2));
        assert_eq!(1, score_left(&grid, 1, 2));
        assert_eq!(2, score_right(&grid, 1, 2));
        assert_eq!(4, get_score(&grid, 1, 2));
    }
}
