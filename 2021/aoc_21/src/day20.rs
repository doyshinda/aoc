use crate::util;
use griddy::Grid;

fn part_1() -> u64 {
    let input = util::read_input("20_a");
    let mut lines = input.lines();
    let img_alg = lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut out_image = Vec::new();
    while let Some(row) = lines.next() {
        if row.len() == 0 {
            continue;
        }
        out_image.push(row.chars().collect::<Vec<char>>());
    }

    let mut old_image_infinite = '.';
    let mut new_image_infinite = '.';

    let mut row_size = out_image.len() + 2;
    let mut col_size = out_image[0].len() + 2;
    let mut grid = Grid::init(row_size, col_size, new_image_infinite);
    let in_row_start = 1;
    let in_col_start = 1;
    for y in 0..out_image.len() {
        for x in 0..out_image[0].len() {
            grid[in_row_start + y][in_col_start + x] = out_image[y][x];
        }
    }

    for _ in 0..2 {
        row_size += 2;
        col_size += 2;
        let i = format!("{r}{r}{r}{r}{r}{r}{r}{r}{r}", r = old_image_infinite);
        new_image_infinite = img_alg[parse(i) as usize];
        let mut new_grid = Grid::init(row_size, col_size, new_image_infinite);
        for y in 0..grid.rows_len() {
            for x in 0..grid.cols_len() {
                let in_index = calc_index(&grid, y, x, old_image_infinite);
                new_grid[1 + y][1 + x] = img_alg[in_index as usize];
            }
        }
        old_image_infinite = new_image_infinite;
        grid = new_grid;
    }

    let mut lit = 0;
    for row in grid.rows() {
        lit += row.iter().filter(|&c| *c == '#').count() as u64;
    }
    lit
}

fn part_2() -> u64 {
    let input = util::read_input("20_a");
    let mut lines = input.lines();
    let img_alg = lines.next().unwrap().chars().collect::<Vec<char>>();

    let mut out_image = Vec::new();
    while let Some(row) = lines.next() {
        if row.len() == 0 {
            continue;
        }
        out_image.push(row.chars().collect::<Vec<char>>());
    }

    let mut old_image_infinite = '.';
    let mut new_image_infinite = '.';

    let mut row_size = out_image.len() + 2;
    let mut col_size = out_image[0].len() + 2;
    let mut grid = Grid::init(row_size, col_size, new_image_infinite);
    let in_row_start = 1;
    let in_col_start = 1;
    for y in 0..out_image.len() {
        for x in 0..out_image[0].len() {
            grid[in_row_start + y][in_col_start + x] = out_image[y][x];
        }
    }

    for _ in 0..50 {
        row_size += 2;
        col_size += 2;
        let i = format!("{r}{r}{r}{r}{r}{r}{r}{r}{r}", r = old_image_infinite);
        new_image_infinite = img_alg[parse(i) as usize];
        let mut new_grid = Grid::init(row_size, col_size, new_image_infinite);
        for y in 0..grid.rows_len() {
            for x in 0..grid.cols_len() {
                let in_index = calc_index(&grid, y, x, old_image_infinite);
                new_grid[1 + y][1 + x] = img_alg[in_index as usize];
            }
        }
        old_image_infinite = new_image_infinite;
        grid = new_grid;
    }

    let mut lit = 0;
    for row in grid.rows() {
        lit += row.iter().filter(|&c| *c == '#').count() as u64;
    }
    lit
}

run!();

fn calc_index(grid: &Grid<char>, row: usize, col: usize, r: char) -> u64 {
    let mut n = String::new();
    let right = col + 1;

    /*
     * Row above
     */
    if let Some(up) = row.checked_sub(1) {
        // Up + left
        if let Some(left) = col.checked_sub(1) {
            n.push(grid[up][left]);
        } else {
            n.push(r);
        }

        // Up
        n.push(grid[up][col]);

        // Up + right
        if right < grid.cols_len() {
            n.push(grid[up][right]);
        } else {
            n.push(r);
        }
    } else {
        n.push_str(&format!("{}{}{}", r, r, r));
    }

    /*
     * Middle row
     */
    if let Some(left) = col.checked_sub(1) {
        n.push(grid[row][left]);
    } else {
        n.push(r);
    }

    n.push(grid[row][col]);

    if right < grid.cols_len() {
        n.push(grid[row][right]);
    } else {
        n.push(r);
    }

    /*
     * Bottom row
     */
    let down = row + 1;
    if down < grid.rows_len() {
        if let Some(left) = col.checked_sub(1) {
            n.push(grid[down][left]);
        } else {
            n.push(r);
        }

        n.push(grid[down][col]);
        if right < grid.cols_len() {
            n.push(grid[down][right]);
        } else {
            n.push(r);
        }
    } else {
        n.push_str(&format!("{}{}{}", r, r, r));
    }

    if n.len() != 9 {
        panic!("you fucked up, len is: {}, string = {}", n.len(), n);
    }

    return parse(n);
}

fn parse(s: String) -> u64 {
    let mut n = 0;
    for c in s.chars() {
        n <<= 1;
        match c {
            '.' => n |= 0,
            _ => n |= 1,
        }
    }

    n as u64
}

fn display(grid: &Grid<char>) {
    for row in grid.rows() {
        let mut s = String::new();
        row.iter().for_each(|c| s.push(*c));
        println!("{}", s);
    }
    println!("");
}
