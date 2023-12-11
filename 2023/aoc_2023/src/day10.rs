use crate::util;
use griddy::Grid;
use std::collections::HashMap;

const P: char = '|'; // is a vertical pipe connecting north and south.
const D: char = '-'; // is a horizontal pipe connecting east and west.
const L: char = 'L'; // is a 90-degree bend connecting north and east.
const J: char = 'J'; // is a 90-degree bend connecting north and west.
const Z: char = '7'; // is a 90-degree bend connecting south and west.
const F: char = 'F'; // is a 90-degree bend connecting south and east.
const DOT: char = '.'; // is ground; there is no pipe in this tile.
const SRT: char = 'S'; // is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

type Coord = (usize, usize);

#[derive(Debug)]
enum Direction {
    N(Coord),
    S(Coord),
    E(Coord),
    W(Coord),
}

#[derive(Debug)]
struct Node {
    north: [char; 3],
    south: [char; 3],
    east: [char; 3],
    west: [char; 3],
}

#[derive(Debug)]
struct Pipe {
    parent: Coord,
    step_count: u64,
    loc: Coord,
    path: Vec<Coord>,
}

impl Pipe {
    fn new(parent: Coord, loc: Coord, step_count: u64) -> Self {
        let path = vec![];
        Pipe {
            parent,
            step_count,
            loc,
            path,
        }
    }
}

impl Node {
    fn connects_east(&self, v: char) -> bool {
        if v == DOT {
            return false;
        }
        self.east.iter().any(|x| *x == v)
    }

    fn connects_west(&self, v: char) -> bool {
        if v == DOT {
            return false;
        }
        self.west.iter().any(|x| *x == v)
    }

    fn connects_north(&self, v: char) -> bool {
        if v == DOT {
            return false;
        }
        self.north.iter().any(|x| *x == v)
    }

    fn connects_south(&self, v: char) -> bool {
        if v == DOT {
            return false;
        }
        self.south.iter().any(|x| *x == v)
    }
}

fn part_1() -> u64 {
    let data = util::read_input("10.input");
    let grid = data.split('\n').map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();

    let map: HashMap<char, Node> = get_map();

    let mut grid = Grid::from_2d_unchecked(grid);
    let mut start = (0, 0);
    'outer: for (x, row) in grid.rows().enumerate() {
        let mut y = 0;
        for c in row {
            if *c == SRT {
                start = (x, y);
                grid[x][y] = calc_start(&grid, &map, start);
                break 'outer;
            }
            y += 1;
        }
    }

    let mut next = vec![Pipe::new(start, start, 0)];

    while next.len() > 0 {
        let curr = next.pop().unwrap();
        if curr.loc == start && curr.step_count != 0 {
            return curr.step_count / 2;
        }

        let c_val = grid[curr.loc.0][curr.loc.1];
        let node = &map[&c_val];
        for d in neighbors(&grid, &curr) {
            if let Some(p) = connects(&grid, &curr, &node, d) {
                next.push(p);
            }
        }
    }

    0
}

fn part_2() -> u64 {
    let data = util::read_input("10.input");
    let grid = data.split('\n').map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();

    let map: HashMap<char, Node> = get_map();

    let mut grid = Grid::from_2d_unchecked(grid);
    let mut start = (0, 0);
    'outer: for (x, row) in grid.rows().enumerate() {
        let mut y = 0;
        for c in row {
            if *c == SRT {
                start = (x, y);
                grid[x][y] = calc_start(&grid, &map, start);
                break 'outer;
            }
            y += 1;
        }
    }

    let mut next = vec![Pipe::new(start, start, 0)];

    while next.len() > 0 {
        let curr = next.pop().unwrap();
        if curr.loc == start && curr.step_count != 0 {
            let mut new_g = Grid::init(grid.rows_len(), grid.cols_len(), '.');
            for (x, y) in curr.path {
                new_g[x][y] = grid[x][y];
            }

            let mut total = 0;
            for r in new_g.rows() {
                let mut s: String = r.iter().fold(String::new(), |mut a, v| {a.push(*v); a});
                s = s.trim_end_matches(".").to_string();

                let mut is_north = false;
                let toggle = |i_val: &mut bool| {
                    if *i_val {
                        *i_val = false;
                    } else {
                        *i_val = true;
                    }
                };
                for v in s.chars() {
                    if v == P || v == L || v == J {
                        toggle(&mut is_north);
                        continue
                    }

                    if v == DOT && is_north {
                        total += 1;
                    }
                }
            }
            return total;
        }

        let c_val = grid[curr.loc.0][curr.loc.1];
        let node = &map[&c_val];
        for d in neighbors(&grid, &curr) {
            if let Some(p) = connects(&grid, &curr, &node, d) {
                next.push(p);
            }
        }
    }

    0
}

run!();

fn calc_start(g: &Grid<char>, m: &HashMap<char, Node>, s: Coord) -> char {
    for (p, n) in m {
        let mut c = 0;

        if let Some(x) = s.0.checked_sub(1) {
            if n.connects_north(g[x][s.1]) {
                c += 1
            }
        }
        if n.connects_south(g[s.0 + 1][s.1]) {
            c += 1
        }
        if n.connects_east(g[s.0][s.1 + 1]) {
            c += 1
        }
        if let Some(y) = s.1.checked_sub(1) {
            if n.connects_west(g[s.0][y]) {
                c += 1
            }
        }

        if c == 2 {
            return *p
        }
    }

    SRT
}

fn neighbors(g: &Grid<char>, p: &Pipe) -> Vec<Direction> {
    let mut r = vec![];
    let rrn = g.row_right_coords(p.loc.0, p.loc.1);
    if !rrn.is_empty() {
        let n = rrn[0];
        if g[n.0][n.1] != DOT && n != p.parent {
            r.push(Direction::E(n));
        }
    }

    let rrn = g.row_left_coords(p.loc.0, p.loc.1);
    if !rrn.is_empty() {
        let n = rrn.last().unwrap();
        if g[n.0][n.1] != DOT && *n != p.parent {
            r.push(Direction::W(*n));
        }
    }

    let rrn = g.col_up_coords(p.loc.0, p.loc.1);
    if !rrn.is_empty() {
        let n = rrn.last().unwrap();
        if g[n.0][n.1] != DOT && *n != p.parent {
            r.push(Direction::N(*n));
        }
    }

    let rrn = g.col_down_coords(p.loc.0, p.loc.1);
    if !rrn.is_empty() {
        let n = rrn[0];
        if g[n.0][n.1] != DOT && n != p.parent {
            r.push(Direction::S(n));
        }
    }

    r
}

fn connects(grid: &Grid<char>, curr: &Pipe, node: &Node, d: Direction) -> Option<Pipe> {
    let mut _n_cpy = (0, 0);
    let func = match d {
        Direction::N(n) => { _n_cpy = n; Node::connects_north },
        Direction::S(n) => { _n_cpy = n; Node::connects_south },
        Direction::E(n) => { _n_cpy = n; Node::connects_east },
        Direction::W(n) => { _n_cpy = n; Node::connects_west },
    };

    let n_val = grid[_n_cpy.0][_n_cpy.1];
    if func(&node, n_val) {
        let mut p = Pipe::new(curr.loc, _n_cpy, curr.step_count+1);
        p.path = curr.path.clone();
        p.path.push(_n_cpy);
        return Some(p);
    }

    None
}

fn get_map() -> HashMap<char, Node> {
    HashMap::from([
        (P, Node {
            north: [P, F, Z],
            south: [P, L, J],
            east: [DOT, DOT, DOT],
            west: [DOT, DOT, DOT],
        }),
        (D, Node {
            west: [D, F, L],
            east: [D, Z, J],
            north: [DOT, DOT, DOT],
            south: [DOT, DOT, DOT],
        }),
        (F, Node {
            north: [DOT, DOT, DOT],
            south: [P, L, J],
            west: [DOT, DOT, DOT],
            east: [D, Z, J],
        }),
        (Z, Node {
            north: [DOT, DOT, DOT],
            south: [P, L, J],
            west: [D, L, F],
            east: [DOT, DOT, DOT],
        }),
        (L, Node {
            north: [P, F, Z],
            south: [DOT, DOT, DOT],
            west: [DOT, DOT, DOT],
            east: [D, Z, J],
        }),
        (J, Node {
            north: [P, F, Z],
            south: [DOT, DOT, DOT],
            west: [D, L, F],
            east: [DOT, DOT, DOT],
        })
    ])
}
