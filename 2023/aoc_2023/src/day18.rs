use crate::util;
use griddy::Grid;

#[derive(Debug)]
struct Dig {
    direction: char,
    num: i64,
    bignum: i64,
    color: String,
}

impl Dig {
    fn from(l: &str) -> Self {
        let vals: Vec<&str> = l.split_ascii_whitespace().collect();
        let direction = match vals[0] {
            "R" => 'r',
            "U" => 'u',
            "D" => 'd',
            _ => 'l',
        };
        let num = inum!(vals[1]);

        Self {
            direction: direction,
            num: num,
            bignum: num,
            color: vals[2].to_string(),
        }
    }

    fn from2(l: &str) -> Self {
        let vals: Vec<&str> = l.split_ascii_whitespace().collect();
        let num = i64::from_str_radix(&vals[2][2..7], 16).unwrap();
        let direction = match vals[2].chars().nth(7).unwrap() {
            '0' => 'r',
            '3' => 'u',
            '1' => 'd',
            _ => 'l',
        };

        Self {
            direction: direction,
            num: inum!(vals[1]),
            bignum: num,
            color: vals[2].to_string(),
        }
    }
}

fn part_1() -> u64 {
    let data = util::read_input("18.input");
    let data = data.trim();

    let digs: Vec<Dig> = data.split("\n").map(|l| Dig::from(l)).collect();
    let mut net_v: i32 = 0;
    let mut net_h: i32 = 0;

    let mut min_v: i32 = 0;
    let mut min_h: i32 = 0;

    let mut max_v: i32 = 0;
    let mut max_h: i32 = 0;

    for d in &digs {
        match d.direction {
            'r' => net_h += d.num as i32,
            'd' => net_v += d.num as i32,
            'l' => net_h -= d.num as i32,
            _ => net_v -= d.num as i32,
        };

        if net_h < min_h {
            min_h = net_h;
        }

        if net_h > max_h {
            max_h = net_h;
        }

        if net_v < min_v {
            min_v = net_v;
        }

        if net_v > max_v {
            max_v = net_v;
        }

    }

    let r = max_v - min_v + 1;
    let c = max_h - min_h + 1;
    let mut grid = Grid::init(r as usize, c as usize, '.');

    let mut x = (min_v * -1) as usize;
    let mut y = (min_h * -1) as usize;
    grid[x][y] = 'F';
    let mut prev: char = 'b';

    for d in digs {
        let n = match d.direction {
            'r' => grid.row_right_coords(x, y),
            'd' => grid.col_down_coords(x, y),
            'l' => {
                let mut m = grid.row_left_coords(x, y);
                m.reverse();
                m
            }
            _ => {
                let mut m = grid.col_up_coords(x, y);
                m.reverse();
                m
            }
        };

        let mut cnt = 0;
        for v in n {
            if cnt >= d.num {
                break;
            }

            if cnt == 0 {
                let mut new = grid[x][y];
                if (prev == 'r' && d.direction == 'd') || (prev == 'u' && d.direction == 'l') {
                    new = '7';
                }

                if (prev == 'r' && d.direction == 'u') || (prev == 'd' && d.direction == 'l') {
                    new = 'J';
                }

                if (prev == 'l' && d.direction == 'u') || (prev == 'd' && d.direction == 'r') {
                    new = 'L';
                }

                if (prev == 'l' && d.direction == 'd') || (prev == 'u' && d.direction == 'r') {
                    new = 'F';
                }

                grid[x][y] = new;
            }

            if d.direction == 'd' || d.direction == 'u' {
                if grid[v.0][v.1] == '.' {
                    grid[v.0][v.1] = '|';
                }
            } else {
                if grid[v.0][v.1] == '.' {
                    grid[v.0][v.1] = '#';
                }
            }

            x = v.0;
            y = v.1;
            cnt += 1;
        }

        prev = d.direction;
        if cnt < d.num {
            panic!("missing");
        }
    }

    let mut total = 0;
    for r in grid.rows() {
        let mut pit_cnt = 0;
        let mut wall_cnt = 0;
        for v in r {
            if *v == '|' || *v == 'L' || *v == 'J' {
                wall_cnt += 1;
                pit_cnt += 1;
                continue;
            }

            if *v == '#' || *v == 'F' || *v == '7' {
                pit_cnt += 1;
                continue;
            }

            if wall_cnt % 2 == 1 {
                pit_cnt += 1;
            }
        }

        total += pit_cnt;
    }

    total as u64
}

fn part_2() -> u64 {
    0
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
    fn test_from_2() {
        let d = Dig::from2("R 6 (#70c710)");
        assert_eq!(461937, d.num);
        assert_eq!(d.direction, 'r');
    }
}