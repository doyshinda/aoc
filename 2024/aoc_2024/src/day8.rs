use crate::util;
use griddy::Grid;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("8.input");
    solve(&data, &s_part_1)
}

fn part_2() -> u64 {
    let data = util::read_input("8.input");
    solve(&data, &s_part_2)
}

fn solve(data: &str, f: &SolveFn) -> u64 {
    let grid = data.split("\n").map(|l| l.chars().collect()).collect();
    let grid = Grid::from_2d_unchecked(grid);
    hm!(nodes, char, Vec<Ituple>);

    for x in 0..grid.rows_len() {
        for y in 0..grid.cols_len() {
            match grid[x][y] {
                '.' => continue,
                c => {
                    let x = convert!(x);
                    let y = convert!(y);
                    nodes.entry(c).and_modify(|v| v.push((x, y))).or_insert(vec![(x, y)]);
                },
            }
        }
    }

    hs!(antinodes, Ituple);
    let rows = convert!(grid.rows_len());
    let cols = convert!(grid.cols_len());
    for (_, locs) in nodes {
        for n in &locs {
            for other in &locs {
                if n == other {
                    continue;
                }

                let mhd = manhattan_distance(*n, *other);
                let mhd = (mhd.0 * -1, mhd.1 * -1);
                f(n, &mut antinodes, rows, cols, mhd);
            }
        }
    }

    antinodes.len() as u64
}

type Ituple = (isize, isize);
type SolveFn = dyn Fn(&Ituple, &mut HashSet<Ituple>, isize, isize, Ituple);

fn s_part_1(n: &Ituple, antinodes: &mut HashSet<Ituple>, rows: isize, cols: isize, mhd: Ituple) {
    let node = (n.0 + mhd.0, n.1 + mhd.1);
    if node.0 > -1 && node.0 < rows && node.1 > -1 && node.1 < cols {
        antinodes.insert(node);
    }
}

fn s_part_2(n: &Ituple, antinodes: &mut HashSet<Ituple>, rows: isize, cols: isize, mhd: Ituple) {
    antinodes.insert(*n);
    let mut prev = (n.0, n.1);
    loop {
        let node = (prev.0 + mhd.0, prev.1 + mhd.1);
        if node.0 > -1 && node.0 < rows && node.1 > -1 && node.1 < cols {
            antinodes.insert(node);
            prev = node;
            continue;
        } else {
            break;
        }
    }
}

fn manhattan_distance(a: Ituple, b: Ituple) -> Ituple {
    (b.0 - a.0, b.1 - a.1)
}

run!();

fn print(grid: &Grid<char>) {
    println!("{}", "====================================");
    for r in grid.rows() {
        println!("{:?}", r.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!((0, 0), manhattan_distance((1, 1), (1, 1)));
        assert_eq!((1, 1), manhattan_distance((1, 1), (2, 2)));
        assert_eq!((1, 0), manhattan_distance((1, 1), (2, 1)));
        assert_eq!((-1, 0), manhattan_distance((2, 1), (1, 1)));
        assert_eq!((-1, -1), manhattan_distance((2, 2), (1, 1)));
    }
}

