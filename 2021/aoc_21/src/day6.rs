use crate::util;

#[derive(Debug)]
struct LanternFish {
    timer: u32,
}

fn new_birth() -> LanternFish {
    LanternFish { timer: 8u32 }
}

impl LanternFish {
    fn cycle(&mut self) -> Option<LanternFish> {
        if self.timer == 0 {
            self.timer = 6;
            return Some(new_birth());
        }

        self.timer -= 1;
        None
    }
}

fn part_1() -> i32 {
    let mut fishes: Vec<LanternFish> = util::read_input("6_a_test")
        .split(',')
        .map(|x| LanternFish {
            timer: x.parse::<u32>().unwrap(),
        })
        .collect();

    for _ in 0..80 {
        let mut new_fishes = Vec::new();
        for f in fishes.iter_mut() {
            match f.cycle() {
                Some(new_fish) => new_fishes.push(new_fish),
                _ => continue,
            }
        }

        for nf in new_fishes {
            fishes.push(nf)
        }
    }

    fishes.len() as i32
}

#[derive(Debug)]
struct Slot {
    count: usize,
    skip: usize,
}

fn part_2() -> usize {
    let mut map = Vec::new();
    for _ in 0..7 {
        map.push(Slot { count: 0, skip: 0 });
    }

    for age in util::read_input("6_a").split(',') {
        let val = age.parse::<usize>().unwrap();
        let slot = val % 7;
        map[slot].count += 1;
    }

    for i in 0..256 {
        let f = &mut map[(i % 7) as usize];
        let double_num = f.count - f.skip;
        f.skip = 0;

        let next_slot = ((i + 2) % 7) as usize;
        let f = &mut map[next_slot];
        f.count += double_num;
        f.skip += double_num;
    }

    map.into_iter().map(|x| x.count).sum()
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}
