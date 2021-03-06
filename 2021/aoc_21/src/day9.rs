use crate::util;
use std::collections::HashSet;

fn part_1() -> usize {
    let mut grid = Vec::new();
    for line in util::read_input("9_a").split('\n') {
        let v: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        grid.push(v);
    }

    let mut lowest = Vec::new();
    let ymax = grid.len() as i32;
    let xmax = grid[0].len() as i32;
    for y in 0..ymax {
        for x in 0..xmax {
            if is_lowest(y as i32, x as i32, &grid, ymax, xmax) {
                lowest.push(grid[y as usize][x as usize]);
            }
        }
    }

    lowest.iter().fold(0, |mut acc, n| {
        acc += n + 1;
        acc
    }) as usize
}

fn is_lowest(y: i32, x: i32, grid: &Vec<Vec<u32>>, ymax: i32, xmax: i32) -> bool {
    let mut candidates = Vec::new();
    if y - 1 >= 0 {
        candidates.push(grid[y as usize - 1][x as usize]);
    }

    if y + 1 < ymax {
        candidates.push(grid[y as usize + 1][x as usize]);
    }

    if x - 1 >= 0 {
        candidates.push(grid[y as usize][x as usize - 1])
    }

    if x + 1 < xmax {
        candidates.push(grid[y as usize][x as usize + 1])
    }

    grid[y as usize][x as usize] < *candidates.iter().min().unwrap()
}

fn part_2() -> usize {
    let mut grid = Vec::new();
    for line in util::read_input("9_a").split('\n') {
        let v: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        grid.push(v);
    }

    let mut basins = Vec::new();
    let ymax = grid.len() as i32;
    let xmax = grid[0].len() as i32;
    for y in 0..ymax {
        for x in 0..xmax {
            if is_lowest(y as i32, x as i32, &grid, ymax, xmax) {
                let bs = basin_size(y as i32, x as i32, &grid, ymax, xmax);
                basins.push(bs);
            }
        }
    }
    basins.sort();
    basins.iter().rev().take(3).product::<u32>() as usize
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}

fn basin_size(y: i32, x: i32, grid: &Vec<Vec<u32>>, ymax: i32, xmax: i32) -> u32 {
    let mut size = 0;

    let mut remaining = Vec::new();
    remaining.push((y, x));

    let mut seen = HashSet::new();
    seen.insert((y, x));

    let neighbors = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    while remaining.len() > 0 {
        let (y, x) = remaining.pop().unwrap();
        if grid[y as usize][x as usize] <= 8 {
            size += 1;
        } else {
            continue;
        }

        for (yd, xd) in &neighbors {
            let mut ydir = y + yd;
            let mut xdir = x + xd;

            while ydir >= 0 && ydir < ymax && xdir >= 0 && xdir < xmax {
                let coord = (ydir, xdir);
                if seen.contains(&coord) {
                    break;
                }

                if grid[ydir as usize][xdir as usize] <= 8 {
                    remaining.push(coord);
                    seen.insert(coord);
                    ydir += yd;
                    xdir += xd;
                } else {
                    break;
                }
            }
        }
    }

    size
}
