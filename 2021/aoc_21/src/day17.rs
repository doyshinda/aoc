use crate::util;

#[derive(Debug, Default)]
struct Probe {
    y_pos: i64,
    x_pos: i64,
    y_vel: i64,
    x_vel: i64,
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    max_height: i64,
}

impl Probe {
    fn step(&mut self) -> bool {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
        if self.x_vel > 0 {
            self.x_vel -= 1;
        } else if self.x_vel < 0 {
            self.x_vel += 1;
        }
        self.y_vel -= 1;

        if self.y_pos > self.max_height {
            self.max_height = self.y_pos;
        }

        self.y_pos < self.y_min
    }

    fn reset(&mut self) {
        self.y_pos = 0;
        self.x_pos = 0;
        self.y_vel = 0;
        self.x_vel = 0;
        self.max_height = 0;
    }

    fn set_velocity(&mut self, x_vel: i64, y_vel: i64) {
        self.x_vel = x_vel;
        self.y_vel = y_vel;
    }

    fn set_target(&mut self, xmin: i64, xmax: i64, ymin: i64, ymax: i64) {
        self.x_min = xmin;
        self.x_max = xmax;
        self.y_min = ymin;
        self.y_max = ymax;
    }

    fn in_target(&self) -> bool {
        self.x_pos >= self.x_min
            && self.x_pos <= self.x_max
            && self.y_pos >= self.y_min
            && self.y_pos <= self.y_max
    }
}

fn part_1() -> i64 {
    let mut s = util::read_input("17_a");
    s.replace_range(0..13, "");
    let p: Vec<&str> = s.split(", ").collect();
    let (xmin, xmax) = p[0][2..].split_once("..").unwrap();
    let (ymin, ymax) = p[1][2..].split_once("..").unwrap();

    let mut probe = Probe::default();
    probe.set_target(
        xmin.parse::<i64>().unwrap(),
        xmax.parse::<i64>().unwrap(),
        ymin.parse::<i64>().unwrap(),
        ymax.parse::<i64>().unwrap(),
    );

    let mut max_height = 0;
    for y in 0..probe.y_min * -1 {
        for x in 0..probe.x_max / 2 {
            probe.reset();
            probe.set_velocity(x, y);
            for _step in 0..500 {
                if probe.step() {
                    break;
                }
                if probe.in_target() {
                    if probe.max_height > max_height {
                        max_height = probe.max_height;
                    }
                    break;
                }
            }
        }
    }
    max_height
}

fn part_2() -> i64 {
    let mut s = util::read_input("17_a");
    s.replace_range(0..13, "");
    let p: Vec<&str> = s.split(", ").collect();
    let (xmin, xmax) = p[0][2..].split_once("..").unwrap();
    let (ymin, ymax) = p[1][2..].split_once("..").unwrap();

    let mut probe = Probe::default();
    probe.set_target(
        xmin.parse::<i64>().unwrap(),
        xmax.parse::<i64>().unwrap(),
        ymin.parse::<i64>().unwrap(),
        ymax.parse::<i64>().unwrap(),
    );

    let mut count = 0;
    for y in probe.y_min..probe.y_min * -1 {
        for x in 0..probe.x_max * 2 {
            probe.reset();
            probe.set_velocity(x, y);
            for _step in 0..500 {
                if probe.step() {
                    break;
                }
                if probe.in_target() {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}
