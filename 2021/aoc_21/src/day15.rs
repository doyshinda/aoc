use crate::util;
use griddy::Grid;
use std::collections::HashMap;

fn part_1() -> u64 {
    let data = util::read_input("15_a");
    let mut g = Vec::new();
    for line in data.lines() {
        g.push(line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>());
    }

    let grid = Grid::from_2d(g);
    dijkstra(&grid)
}

fn part_2() -> u64 {
    let data = util::read_input("15_a");
    let mut g = Vec::new();
    for line in data.lines() {
        let original = line.chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>();
        let mut row = original.clone();
        for i in 1..5 {
            row.append(&mut original.iter().map(|v| {
                let mut nv = (*v + i) % 9;
                if nv == 0 {
                    nv = 9;
                }
                nv
            }).collect::<Vec<u64>>());
        }
        g.push(row);

    }
    let mut new_rows = Vec::new();
    for i in 1..5 {
        for row in &g {
            new_rows.push(row.iter().map(|v| {
                let mut nv = (*v + i) % 9;
                if nv == 0 {
                    nv = 9;
                }
                nv
            }).collect::<Vec<u64>>());
        }
    }
    for nr in new_rows {
        g.push(nr);
    }

    let grid = Grid::from_2d(g);

    let ymax = grid.rows_len();
    let xmax = grid.cols_len();
    let mut vals = Grid::init(ymax, xmax, 1000000);
    vals[ymax - 1][xmax - 1] = grid[ymax - 1][xmax - 1];

    build_map(&grid, &mut vals);
    let path = find_path(&vals);

    path.iter().fold(0, |acc, x| acc + grid[x.0][x.1])
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}


fn build_map(grid: &Grid<u64>, vals: &mut Grid<u64>) {
    let ymax = grid.rows_len();
    let xmax = grid.cols_len();

    // This is gross
    for _ in 0..3 {
        let mut row_iter = (0..ymax).rev();
        let mut col_iter = (0..xmax - 1).rev();

        while let Some(y) = row_iter.next() {
            while let Some(x) = col_iter.next() {
                let risk = grid[y][x];

                let mut existing_risk = vals[y][x];
                for n in grid.row_neighbors(y, x).into_iter().chain(grid.col_neighbors(y, x)) {
                    let new_risk = vals[n.0][n.1] + risk;
                    if new_risk < existing_risk {
                        existing_risk = new_risk
                    }
                }

                vals[y][x] = existing_risk;
            }
            col_iter = (0..xmax).rev();
        }
    }
}

fn find_path(vals: &Grid<u64>) -> Vec<(usize, usize)> {
    let mut path = Vec::new();
    let ymax = vals.rows_len();
    let xmax = vals.cols_len();
    let last = (ymax - 1, xmax - 1);
    let mut curr = (0, 0);

    while curr != last {
        let mut m = vals[curr.0][curr.1];
        let mut new_curr = curr;
        for n in vals.col_neighbors(curr.0, curr.1) {
             if vals[n.0][n.1] < m {
                m = vals[n.0][n.1];
                new_curr = (n.0, n.1);
            }
        }
        for n in vals.row_neighbors(curr.0, curr.1) {
            if vals[n.0][n.1] < m {
                m = vals[n.0][n.1];
                new_curr = (n.0, n.1);
            }
        }

        path.push(new_curr);
        curr = new_curr;
    }

    path
}

fn dijkstra(grid: &Grid<u64>) -> u64 {
    let mut dist = HashMap::new();
    let mut queue: Vec<(usize, usize)> = Vec::new();
    let source = (0, 0);
    for y in 0..grid.rows_len() {
        for x in 0..grid.cols_len() {
            let v = (y, x);
            dist.insert(v, 1000000u64);
            queue.push(v);
        }
    }

    dist.insert(source, 0);
    while queue.len() > 0 {
        let c = find_min(&mut queue, &dist);
        let (y, x) = c;

        for n in grid.row_neighbors(y, x).into_iter().chain(grid.col_neighbors(y, x)) {
            if !queue.contains(&n) {
                continue;
            }

            let dist_u = dist.get(&c).unwrap();
            let dist_v = dist.get(&n).unwrap();
            let alt = dist_u + grid[n.0][n.1];

            if alt < *dist_v {
                dist.insert(n, alt);
            }
        }
    }

    *dist.get(&(grid.rows_len() - 1, grid.cols_len() - 1)).unwrap()
}

fn find_min(q: &mut Vec<(usize, usize)>, d: &HashMap<(usize, usize), u64>) -> (usize, usize) {
    let mut min_val = u64::MAX;
    let mut min_idx = 0;
    for (idx, c) in q.iter().enumerate() {
        let val = d.get(c).unwrap();
        if *val < min_val {
            min_val = *val;
            min_idx = idx;
        }
    }

    q.swap_remove(min_idx)
}


fn display(grid: &Grid<u64>) {
    for row in grid.rows() {
        let mut s = String::new();
        for r in row {
            if *r > 100000 {
                s.push_str("  #");
            } else {
                s.push_str(&format!("{:>3}", *r));
            }
        }
        println!("{}", s);
    }
    println!("");
}

#[test]
fn test_grid() {
    let input = vec![
        vec![1, 9, 6, 3, 7, 5],
        vec![1, 9, 1, 1, 1, 1],
        vec![1, 1, 1, 9, 9, 1],
        vec![9, 9, 9, 9, 9, 1],
        vec![9, 9, 6, 9, 9, 1],
        vec![9, 9, 9, 9, 9, 1],
        vec![9, 9, 9, 9, 9, 1],
    ];
    let grid = Grid::from_2d(input);
    let ymax = grid.rows_len();
    let xmax = grid.cols_len();
    let mut vals = Grid::init(ymax, xmax, 1000000);
    vals[ymax - 1][xmax - 1] = grid[ymax - 1][xmax - 1];

    build_map(&grid, &mut vals);
    let path = find_path(&vals);
    let pr = path.iter().fold(0, |acc, x| acc + grid[x.0][x.1]);
    assert_eq!(pr, 13);
}

#[test]
fn test_dijkstra() {
    let input = vec![
        vec![1, 9, 6, 3, 7, 5],
        vec![1, 9, 1, 1, 1, 1],
        vec![1, 1, 1, 9, 9, 1],
        vec![9, 9, 9, 9, 9, 1],
        vec![9, 9, 6, 9, 9, 1],
        vec![9, 9, 9, 9, 9, 1],
        vec![9, 9, 9, 9, 9, 1],
    ];
    let grid = Grid::from_2d(input);
    let r = dijkstra(&grid);
    assert_eq!(r, 13);
}
