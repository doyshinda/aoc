use crate::util;
use griddy::Grid;

const BLOCK: char = '#';
const FREE: char = '.';
const ROCK : char = 'O';


fn part_1() -> u64 {
    let data = util::read_input("14.input");
    let grid = data.split("\n").map(|l| l.chars().collect()).collect();

    let grid = Grid::from_2d_unchecked(grid);
    let mut t = grid.transpose();
    roll(&mut t);
    let grid = t.transpose();
    load_on_north_beam(&grid) as u64
}

fn part_2() -> u64 {
    let data = util::read_input("14.input");
    let grid = data.trim().split("\n").map(|l| l.chars().collect()).collect();
    let mut grid = Grid::from_2d_unchecked(grid);

    hm!(seen, usize, usize);
    hm!(counts, String, usize);

    for i in 0..10000 {
        grid = cycle(&mut grid);

        if i < 2000 {
            continue
        }
        let k = key(&grid);
        if counts.contains_key(&k) {
            break;
        }

        let total = load_on_north_beam(&grid);
        counts.insert(k, total);
        seen.insert(i + 1, total);

    }

    let want_mod = 1_000_000_000 % seen.len();
    for (i, v) in &seen {
        if i % seen.len() == want_mod {
            return *v as u64;
        }
    }

    0
}

run!();

fn load_on_north_beam(grid: &Grid<char>) -> usize {
    let mut total = 0;
    let t = grid.transpose();

    for r in t.rows() {
        for (i, v) in r.iter().enumerate() {
            if *v == ROCK {
                let idx_val = r.len() - i;
                total += idx_val;
            }
        }
    }

    total
}

fn calc_key(grid: &Grid<char>) -> String {
    let mut s: String = String::new();
    for r in grid.rows() {
        for c in r {
            s.push(*c);
        }
    }

    s
}

fn calc_load(cube_idx: i32, cnt: usize, rlen: usize) -> usize {
    let start_idx = if cube_idx == -1 {
        rlen
    } else {
        rlen - cube_idx as usize - 1
    };
    
    if cnt > start_idx {
        return 0;
    }

    let mut total = 0;
    let mut iter = (1..=start_idx).rev();
    for _ in 0..cnt {
        if let Some(next) = iter.next() {
            total += next;
        }
    }

    total
}

fn key(grid: &Grid<char>) -> String {
    let mut s = String::new();
    for r in grid.rows() {
        for c in r {
            s.push(*c);
        }
    }
    s
}

fn cycle(grid: &mut Grid<char>) -> Grid<char> {
    // North
    let mut t = grid.transpose();
    roll(&mut t);
    let mut grid = t.transpose();

    // West
    roll(&mut grid);

    // South
    let mut t = grid.transpose();
    t.flip_y();
    roll(&mut t);
    t.flip_y();
    let mut grid = t.transpose();

    // East
    grid.flip_y();
    roll(&mut grid);
    grid.flip_y();

    grid
}

fn roll(grid: &mut Grid<char>) {
    for mut row in grid.rows_mut() {
        roll_row(&mut row);
    }
}

fn roll_row(r: &mut Vec<char>) {
    let mut free_idx = None;

    for i in 0..r.len() {
        if r[i] == BLOCK {
            free_idx = None;
            continue
        }

        if r[i] == FREE && free_idx.is_none() {
            free_idx = Some(i);
            continue;
        }

        if r[i] == ROCK {
            if let Some(idx) = free_idx {
                r.swap(i, idx);
                free_idx = Some(idx + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_load() {
        let l = calc_load(0, 0, 10);
        assert_eq!(0, l);

        let l = calc_load(0, 1, 10);
        assert_eq!(9, l);

        let l = calc_load(-1, 4, 10);
        assert_eq!(34, l);

        let l = calc_load(8, 0, 10);
        assert_eq!(0, l);

        let l = calc_load(8, 5, 10);
        assert_eq!(0, l);

        assert_eq!(1, calc_load(8, 1, 10));
        assert_eq!(3, calc_load(7, 2, 10));
        assert_eq!(2, calc_load(7, 1, 10));
        assert_eq!(6, calc_load(6, 3, 10));
        assert_eq!(10, calc_load(5, 4, 10));
        assert_eq!(15, calc_load(4, 5, 10));
        assert_eq!(21, calc_load(3, 6, 10));
        assert_eq!(28, calc_load(2, 7, 10));
        assert_eq!(36, calc_load(1, 8, 10));
        assert_eq!(45, calc_load(0, 9, 10));
        assert_eq!(55, calc_load(-1, 10, 10));
    }

    #[test]
    fn test_roll_row_no_moves() {
        let mut row = vec!['.', '#', 'O', '.', '#', 'O', '.', '.', '.', '.'];
        roll_row(&mut row);
        let expected = vec!['.', '#', 'O', '.', '#', 'O', '.', '.', '.', '.'];
        assert_eq!(expected, row);
    }

    #[test]
    fn test_roll_row_all_shift() {
        let mut row = vec!['#', '#', '.', '.', 'O', '.', 'O', '.', 'O', 'O'];
        roll_row(&mut row);
        let expected = vec!['#', '#', 'O', 'O', 'O', 'O', '.', '.', '.', '.'];
        assert_eq!(expected, row);
    }

    #[test]
    fn test_roll_row_one_shift() {
        let mut row = vec!['.', '#', 'O', '.', '#', '.', 'O', '.', '.', '.'];
        roll_row(&mut row);
        let expected = vec!['.', '#', 'O', '.', '#', 'O', '.', '.', '.', '.'];
        assert_eq!(expected, row);
    }

    #[test]
    fn test_cycle() {
        let a = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

        let t = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

        let p = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

        let s = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";
        
        let grid = t.split("\n").map(|l| l.chars().collect()).collect();
        let mut grid = Grid::from_2d_unchecked(grid);
        grid = cycle(&mut grid);

        let agrid = a.split("\n").map(|l| l.chars().collect()).collect();
        let expected = Grid::from_2d_unchecked(agrid);
        for (e, g) in std::iter::zip(expected.rows(), grid.rows()) {
            assert_eq!(e, g);
        }

        grid = cycle(&mut grid);
        let pgrid = p.split("\n").map(|l| l.chars().collect()).collect();
        let expected = Grid::from_2d_unchecked(pgrid);
        for (e, g) in std::iter::zip(expected.rows(), grid.rows()) {
            assert_eq!(e, g);
        }

        grid = cycle(&mut grid);
        let sgrid = s.split("\n").map(|l| l.chars().collect()).collect();
        let expected = Grid::from_2d_unchecked(sgrid);
        for (e, g) in std::iter::zip(expected.rows(), grid.rows()) {
            assert_eq!(e, g);
        }
    }
}
