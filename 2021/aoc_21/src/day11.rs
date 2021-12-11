use crate::util;

#[derive(Debug)]
struct Octo {
    energy: u32,
    flash_step: i32,
}

impl Octo {
    fn parse(c: char) -> Octo {
        Octo {
            energy: c.to_digit(10).unwrap(),
            flash_step: -1,
        }
    }

    fn flash(&mut self, i: i32) -> bool {
        if i > self.flash_step {
            self.energy += 1;
        }

        if self.energy > 9 {
            self.flash_step = i;
            self.energy = 0;
            return true;
        }

        false
    }
}

fn part_1() -> u32 {
    let mut octos: Vec<Vec<Octo>> = util::read_input("11_a")
        .split('\n')
        .map(|s| s.chars().map(|c| Octo::parse(c)).collect::<Vec<Octo>>())
        .collect();

    let ymax = octos.len();
    let xmax = octos[0].len();
    let mut flashes = 0;
    for i in 0..100 {
        let mut flash_neighbors = Vec::new();
        for y in 0..ymax {
            for x in 0..xmax {
                if octos[y][x].flash(i) {
                    flashes += 1;
                    neighbors(y as i32, x as i32, ymax, xmax)
                        .into_iter()
                        .for_each(|n| flash_neighbors.push(n));
                }
            }
        }

        while flash_neighbors.len() > 0 {
            let (y, x) = flash_neighbors.pop().unwrap();
            if octos[y as usize][x as usize].flash(i) {
                flashes += 1;
                neighbors(y, x, ymax, xmax)
                    .into_iter()
                    .for_each(|n| flash_neighbors.push(n));
            }
        }
    }

    flashes
}

fn neighbors(y: i32, x: i32, ymax: usize, xmax: usize) -> Vec<(i32, i32)> {
    let mut r = Vec::new();
    let ymax = ymax as i32;
    let xmax = xmax as i32;
    let up = y - 1;
    let down = y + 1;
    let right = x + 1;
    let left = x - 1;

    if up >= 0 {
        r.push((up, x));
        if left >= 0 {
            r.push((up, left));
        }
        if right < xmax {
            r.push((up, right));
        }
    }

    if down < ymax {
        r.push((down, x));
        if left >= 0 {
            r.push((down, left));
        }
        if right < xmax {
            r.push((down, right));
        }
    }

    if left >= 0 {
        r.push((y, left));
    }

    if right < xmax {
        r.push((y, right));
    }
    r
}

fn part_2() -> u32 {
    let mut octos: Vec<Vec<Octo>> = util::read_input("11_a")
        .split('\n')
        .map(|s| s.chars().map(|c| Octo::parse(c)).collect::<Vec<Octo>>())
        .collect();

    let ymax = octos.len();
    let xmax = octos[0].len();
    let flash_max = ymax * xmax;
    let mut i = 1;

    loop {
        let mut flashes = 0;
        let mut flash_neighbors = Vec::new();
        for y in 0..ymax {
            for x in 0..xmax {
                if octos[y][x].flash(i) {
                    flashes += 1;
                    neighbors(y as i32, x as i32, ymax, xmax)
                        .into_iter()
                        .for_each(|n| flash_neighbors.push(n));
                }
            }
        }

        while flash_neighbors.len() > 0 {
            let (y, x) = flash_neighbors.pop().unwrap();
            if octos[y as usize][x as usize].flash(i) {
                flashes += 1;
                neighbors(y, x, ymax, xmax)
                    .into_iter()
                    .for_each(|n| flash_neighbors.push(n));
            }
        }

        if flashes == flash_max {
            return i as u32;
        }

        i += 1;
    }
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}

#[test]
fn test_parse_octo() {
    let o = Octo::parse('5');
    assert_eq!(o.energy, 5);
}

#[test]
fn test_neighbors() {
    let n = neighbors(0, 0, 10, 10);
    assert_eq!(n, vec![(1, 0), (1, 1), (0, 1)]);
}
