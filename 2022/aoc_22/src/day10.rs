use crate::util;
use griddy::Grid;

fn part_1() -> u64 {
    let data = util::read_input("10.input");
    let mut instructions = Vec::new();
    let mut cycle_count = 0;
    let mut reg = 1;
    for line in data.lines() {
        if line.starts_with("noop") {
            instructions.push((1, 0));
        } else {
            let (_, v) = line.split_once(' ').unwrap();
            instructions.push((2, inum!(v)));
        }
    }

    let mut sum = 0;
    for i in instructions {
        for _ in 0..i.0 {
            cycle_count += 1;
            if cycle_count == 20
                || cycle_count == 60
                || cycle_count == 100
                || cycle_count == 140
                || cycle_count == 180
                || cycle_count == 220
            {
                let sig = cycle_count * reg;
                sum += sig;
            }
        }
        reg += i.1
    }
    sum as u64
}

fn part_2() -> u64 {
    let data = util::read_input("10.input");
    let mut instructions = Vec::new();
    let mut cycle_count = 0;
    let mut reg = 1i64;
    for line in data.lines() {
        if line.starts_with("noop") {
            instructions.push((1, 0));
        } else {
            let (_, v) = line.split_once(' ').unwrap();
            instructions.push((2, inum!(v)));
        }
    }

    let mut grid = Grid::init(6, 40, '.');
    let mut row = 0;
    let mut col = 0;
    for i in instructions {
        for _ in 0..i.0 {
            if col == reg - 1 || col == reg || col == reg + 1 {
                grid[row][col as usize] = '#';
            }
            cycle_count += 1;
            if cycle_count % 40 == 0 && cycle_count > 1 {
                row += 1_usize;
                col = 0;
            } else {
                col += 1;
            }
        }

        reg += i.1;
    }

    for row in grid.rows() {
        for v in row {
            print!("{}", v)
        }
        println!();
    }
    0
}

run!();
