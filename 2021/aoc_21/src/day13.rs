use crate::util;
use griddy::Grid;

fn display(grid: &Grid<usize>) {
    for row in grid.rows() {
        let mut s = String::new();
        for r in row {
            if *r >= 1 {
                s.push('#');
            } else {
                s.push('.')
            }
        }
        println!("{}", s);
    }
}

fn part_1() -> usize {
    let mut vals: Vec<(u32, u32)> = Vec::new();
    let mut folds: Vec<(char, u32)> = Vec::new();
    for line in util::read_input("13_a").lines() {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("fold") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (c, v) = parts[2].split_once('=').unwrap();
            folds.push((c.chars().next().unwrap(), v.parse::<u32>().unwrap()));
        } else {
            let (a, b) = line.split_once(',').unwrap();
            vals.push((a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()));
        }
    }

    let mut xmax = 0;
    let mut ymax = 0;
    vals.iter().for_each(|(a, b)| {
        if *a > xmax {
            xmax = *a;
        }

        if *b > ymax {
            ymax = *b;
        }
    });
    xmax += 1;
    ymax += 1;

    let mut grid = Vec::with_capacity(ymax as usize);
    for _ in 0..ymax {
        let mut row = Vec::with_capacity(xmax as usize);
        (0..xmax).for_each(|_| row.push(0));
        grid.push(row);
    }

    for (x, y) in vals {
        grid[y as usize][x as usize] = 1;
    }

    let (c, v) = folds[0];
    if c == 'y' {
        let mut new_y = (0..v as usize).rev();
        for y in (v + 1) as usize..ymax as usize {
            let new_y_coord = new_y.next().unwrap();
            for x in 0..xmax as usize {
                grid[new_y_coord as usize][x as usize] += grid[y as usize][x as usize];
            }
        }
        grid.truncate(v as usize);
    } else {
        for y in 0..ymax {
            let mut new_x = (0..v as usize).rev();
            for x in (v + 1) as usize..xmax as usize {
                let new_x_coord = new_x.next().unwrap();
                grid[y as usize][new_x_coord as usize] += grid[y as usize][x as usize];
            }
            grid[y as usize].truncate(v as usize);
        }
    }

    grid.iter().flatten().filter(|&x| *x >= 1).count()
}

fn part_2() -> usize {
    let mut vals: Vec<(usize, usize)> = Vec::new();
    let mut folds: Vec<(char, usize)> = Vec::new();

    let mut xmax = 0;
    let mut ymax = 0;
    for line in util::read_input("13_a").lines() {
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("fold") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let (c, v) = parts[2].split_once('=').unwrap();
            folds.push((c.chars().next().unwrap(), v.parse::<usize>().unwrap()));
        } else {
            let (a, b) = line.split_once(',').unwrap();
            let (a, b) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
            if a > xmax {
                xmax = a;
            }

            if b > ymax {
                ymax = b;
            }
            vals.push((a, b));
        }
    }

    xmax += 1;
    ymax += 1;

    let mut grid = Grid::init(ymax, xmax, 0);

    for (x, y) in vals {
        grid[y][x] = 1;
    }

    for (c, v) in folds {
        if c == 'y' {
            ymax = grid.fold_at_row(v, |new, old| new + old);
        } else {
            for y in 0..ymax {
                let mut new_x = (0..v as usize).rev();
                for x in (v + 1) as usize..xmax as usize {
                    let new_x_coord = new_x.next().unwrap();
                    grid[y as usize][new_x_coord as usize] += grid[y as usize][x as usize];
                }
                grid[y as usize].truncate(v as usize);
            }
            xmax = v;
        }
    }
    display(&grid);
    0
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}
