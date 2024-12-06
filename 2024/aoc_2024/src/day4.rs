use crate::util;
use griddy::{Direction, OptGrid};

fn check_grid(grid: &OptGrid, x: isize, y: isize) -> u64 {
    let mut ans = 0;
    for (rs, cs) in Direction::ALL {
        let one = grid.get(x + (rs * 1), y + (cs * 1)).unwrap_or(&b' ');
        let two = grid.get(x + (rs * 2), y + (cs * 2)).unwrap_or(&b' ');
        let three = grid.get(x + (rs * 3), y + (cs * 3)).unwrap_or(&b' ');

        if *one == b'M' && *two == b'A' && *three == b'S' {
            ans += 1;
        }
    }

    ans
}

fn part_1() -> u64 {
    let data = util::read_input("4.input");
    let mut ans = 0;
    let grid = OptGrid::new(&data, '\n');

    for (x, y, c) in grid.clone() {
        match c {
            b'X' => { ans += check_grid(&grid, x, y) },
            _ => (),
        }
    }

    ans
}

fn part_2() -> u64 {
    let data = util::read_input("4.input");
    let mut ans = 0;
    let grid = OptGrid::new(&data, '\n');
    let valid: u16 = ((b'M' as u16) * 2) + ((b'S' as u16) * 2);

    for (x, y, c) in grid.clone() {
        if c == b'A' {
            let count = Direction::DIAGONAL.iter().fold(0u16, |acc, (rs, cs)| {
                match grid.get(x + rs, y + cs) {
                    Some(v) => acc + (*v as u16),
                    None => acc,
                }
            });

            if count == valid {
                let a = grid.get(x + Direction::UPRIGHT.0, y + Direction::UPRIGHT.1).unwrap();
                let b = grid.get(x + Direction::DOWNLEFT.0, y + Direction::DOWNLEFT.1).unwrap();
                if a != b {
                    ans += 1;
                }
            }
        }
    }

    ans
}

run!();
