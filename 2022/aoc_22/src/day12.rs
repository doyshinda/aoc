use crate::util;
use griddy::Grid;
use std::collections::HashSet;

const INDEXING: &str = "abcdefghijklmnopqrstuvwxyz";

fn part_1() -> i64 {
    let data = util::read_input("12.input");
    let mut grid = Grid::from_2d_unchecked(
        data
            .lines()
            .map(|line| line
                .chars()
                .map(parse_elevation)
                .collect::<Vec<i64>>()
            ).collect::<Vec<Vec<i64>>>());

    let mut steps = Grid::init(grid.rows_len(), grid.cols_len(), i64::MAX);
    let mut start = (0, 0);
    let mut end = (0, 0);
    hs!(seen, (usize, usize));
    'outer: for (row_idx, row) in grid.rows().enumerate() {
        for col in 0..row.len() {
            if row[col] == 0 {
                start = (row_idx, col);
            }
            if row[col] == 27 {
                end = (row_idx, col);
                break 'outer;
            }
        }
    }

    grid[start.0][start.1] = 1;
    grid[end.0][end.1] = 26;
    steps[end.0][end.1] = 0;
    let val = bfs(&grid, &mut steps, &mut seen, start, end);
    val
}

fn part_2() -> i64 {
    let data = util::read_input("12.input");
    let mut grid = Grid::from_2d_unchecked(
        data
            .lines()
            .map(|line| line
                .chars()
                .map(parse_elevation)
                .collect::<Vec<i64>>()
            ).collect::<Vec<Vec<i64>>>());

    let mut start = (0, 0);
    let mut starts = Vec::new();
    let mut end = (0, 0);
    
    for (row_idx, row) in grid.rows().enumerate() {
        for col in 0..row.len() {
            if row[col] == 0 {
                starts.push((row_idx, col));
                start = (row_idx, col);
            }
            if row[col] == 1 {
                starts.push((row_idx, col));
            }
            if row[col] == 27 {
                end = (row_idx, col);
            }
        }
    }

    grid[start.0][start.1] = 1;
    grid[end.0][end.1] = 26;
    let mut min_val = i64::MAX;
    for s in starts {
        hs!(seen, (usize, usize));
        let mut steps = Grid::init(grid.rows_len(), grid.cols_len(), i64::MAX);
        let val = bfs(&grid, &mut steps, &mut seen, s, end);
        if val < min_val {
            min_val = val;
        }
    }
    min_val
}

run!();

fn parse_elevation(c: char) -> i64 {
    match c {
        'S' => 0,
        'E' => 27,
        _ => (INDEXING.find(c).unwrap() + 1) as i64,
    }
}

fn bfs(grid: &Grid<i64>, dist: &mut Grid<i64>, seen: &mut HashSet<(usize, usize)>, start: (usize, usize), end: (usize, usize)) -> i64 {
    let mut queue = vec![start];
    dist[start.0][start.1] = 0;

    while queue.len() != 0 {
        let (y, x) = queue.remove(0);
        let me = grid[y][x];

        if y == end.0 && x == end.1 {
            return dist[end.0][end.1];
        }

        for v in grid.row_neighbors(y, x).into_iter().chain(grid.col_neighbors(y, x)) {
            if seen.contains(&(v.0, v.1)) {
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
                seen.insert((v.0, v.1));
                queue.push((v.0, v.1));
                dist[v.0][v.1] = dist[y][x] + 1;
            }
        }
    }
    i64::MAX
}