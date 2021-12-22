use crate::util;
use griddy::Grid;

fn get_range(r: &str) -> (i32, i32) {
    let (s, e) = r.split_once("..").unwrap();
    (s.parse::<i32>().unwrap(), e.parse::<i32>().unwrap())
}

fn part_1() -> u64 {
    let data = util::read_input("22_a");
    let mut cube = Vec::new();
    for _ in 0..101 {
        cube.push(Grid::init(101, 101, 0));
    }
    'outer: for line in data.lines() {
        let (state, coord_str) = line.split_once(" ").unwrap();
        let state = match state {
            "on" => 1,
            "off" => 0,
            _ => continue,
        };
        let coords: Vec<&str> = coord_str.split(',').collect();
        let mut x_range = (0, 0);
        let mut y_range = (0, 0);
        let mut z_range = (0, 0);

        for c in coords {
            if c.starts_with("x") {
                let (_, range) = c.split_once('=').unwrap();
                let (s, e) = get_range(range);
                if s >= -50 && e <= 50 {
                    x_range = (s + 50, e + 50)
                } else {
                    continue 'outer;
                }
            }
            if c.starts_with("y") {
                let (_, range) = c.split_once('=').unwrap();
                let (s, e) = get_range(range);
                if s >= -50 && e <= 50 {
                    y_range = (s + 50, e + 50)
                } else {
                    continue 'outer;
                }
            }
            if c.starts_with("z") {
                let (_, range) = c.split_once('=').unwrap();
                let (s, e) = get_range(range);
                if s >= -50 && e <= 50 {
                    z_range = (s + 50, e + 50)
                } else {
                    continue 'outer;
                }
            }
        }

        for z in z_range.0..=z_range.1 {
            for y in y_range.0..=y_range.1 {
                for x in x_range.0..=x_range.1 {
                    cube[z as usize][y as usize][x as usize] = state;
                }
            }
        }
    }
    let mut count = 0;
    for c in cube {
        for row in c.rows() {
            count += row.iter().filter(|&x| *x == 1).count() as u64;
        }
    }
    count
}

fn part_2() -> u64 {
    0
}

run!();
