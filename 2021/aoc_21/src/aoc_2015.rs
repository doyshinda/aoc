use crate::util;
use std::collections::HashMap;

struct MyString {
    s: String,
}

impl MyString {
    fn is_nice(&self) -> bool {
        self.contains_pair() && self.repeats()
    }

    fn contains_pair(&self) -> bool {
        let parts = self.s.clone().chars().collect::<Vec<char>>();
        let mut mapping = HashMap::new();
        for i in 0..parts.len() - 1 {
            let combo = format!("{}{}", parts[i], parts[i + 1]);
            let entry = mapping.entry(combo).or_insert(Vec::new());
            entry.push(i as i32);
        }

        for (_combo, counts) in &mapping {
            if counts.len() > 1 {
                let r = counts.len() - 1;
                for i in 0..r {
                    if (counts[r] - counts[i]) >= 2 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn repeats(&self) -> bool {
        let parts = self.s.clone().chars().collect::<Vec<char>>();
        for i in 0..parts.len() - 2 {
            if parts[i] == parts[i + 2] {
                return true;
            }
        }

        false
    }
}

fn day_5_part_2() -> usize {
    let strings: Vec<MyString> = util::read_input("2015_day5")
        .split('\n')
        .map(|x| MyString{s: x.to_string()})
        .collect();

    strings.iter().filter(|x| x.is_nice()).count()
}

fn day_6_part_1() -> u32 {
    let mut grid = Vec::new();
    for _ in 0..1000 {
        grid.push([0; 1000]);
    }

    for line in util::read_input("2015_day6").split('\n') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match (parts[0], parts[1]) {
            ("toggle", _) => {
                let mut iter = parts[1].split(',');
                let xstart = iter.next().unwrap().parse::<u32>().unwrap();
                let ystart = iter.next().unwrap().parse::<u32>().unwrap();

                let mut iter = parts[3].split(',');
                let xend = iter.next().unwrap().parse::<u32>().unwrap();
                let yend = iter.next().unwrap().parse::<u32>().unwrap();
                for x in xstart..xend + 1{
                    for y in ystart..yend + 1 {
                        grid[y as usize][x as usize] ^= 1;
                    }
                }
            }
            ("turn", c) => {
                let mut iter = parts[2].split(',');
                let xstart = iter.next().unwrap().parse::<u32>().unwrap();
                let ystart = iter.next().unwrap().parse::<u32>().unwrap();

                let mut iter = parts[4].split(',');
                let xend = iter.next().unwrap().parse::<u32>().unwrap();
                let yend = iter.next().unwrap().parse::<u32>().unwrap();
                let val = if c == "off" {
                    0
                } else {
                    1
                };
                for x in xstart..xend + 1{
                    for y in ystart..yend + 1{
                        grid[y as usize][x as usize] = val;
                    }
                }
            }
            _ => (),
        }
    }

    grid.iter().fold(0, |mut acc, row| {
        acc += row.into_iter().filter(|&x| *x == 1).count() as u32;
        acc
    })
    // 0
}

fn day_6_part_2() -> i32 {
    let mut grid = Vec::new();
    for _ in 0..1000 {
        grid.push([0; 1000]);
    }

    for line in util::read_input("2015_day6").split('\n') {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match (parts[0], parts[1]) {
            ("toggle", _) => {
                let mut iter = parts[1].split(',');
                let xstart = iter.next().unwrap().parse::<i32>().unwrap();
                let ystart = iter.next().unwrap().parse::<i32>().unwrap();

                let mut iter = parts[3].split(',');
                let xend = iter.next().unwrap().parse::<i32>().unwrap();
                let yend = iter.next().unwrap().parse::<i32>().unwrap();
                for x in xstart..xend + 1 {
                    for y in ystart..yend + 1 {
                        grid[y as usize][x as usize] += 2;
                    }
                }
            }
            ("turn", c) => {
                let mut iter = parts[2].split(',');
                let xstart = iter.next().unwrap().parse::<i32>().unwrap();
                let ystart = iter.next().unwrap().parse::<i32>().unwrap();

                let mut iter = parts[4].split(',');
                let xend = iter.next().unwrap().parse::<i32>().unwrap();
                let yend = iter.next().unwrap().parse::<i32>().unwrap();
                let val = if c == "off" {
                    -1
                } else {
                    1
                };
                for x in xstart..xend + 1 {
                    for y in ystart..yend + 1 {
                        if grid[y as usize][x as usize] == 0 && c == "off" {
                            continue
                        }
                        grid[y as usize][x as usize] += val;
                    }
                }
            }
            _ => (),
        }
    }

    grid.iter().fold(0, |mut acc, row| {
        acc += row.into_iter().filter(|&x| *x >= 0).sum::<i32>() as i32;
        acc
    })
    // 0
}

#[test]
fn test_day6_part_1() {
    let r = day_6_part_1();
    assert_eq!(r, 543903);
}

#[test]
fn test_day6_part_2() {
    let r = day_6_part_2();
    assert_eq!(r, 14687245);
}


#[test]
fn test_day5_part_2() {
    let r = day_5_part_2();
    assert_eq!(r, 51);
}

#[test]
fn test_is_nice() {
    let s = "qjhvhtzxzqqjkmpb".to_string();
    let ms = MyString{s};
    assert!(ms.is_nice());

    let s = "xxyxx".to_string();
    let ms = MyString{s};
    assert!(ms.is_nice());

    let s = "ztxhjwllrckhhhhh".to_string();
    let ms = MyString{s};
    assert!(ms.is_nice());

    let s = "uurcxstgmygtbstg".to_string();
    let ms = MyString{s};
    assert!(!ms.is_nice());

    let s = "ieodomkazucvgmuy".to_string();
    let ms = MyString{s};
    assert!(!ms.is_nice());

    let s = "aaa".to_string();
    let ms = MyString{s};
    assert!(!ms.is_nice());
}
