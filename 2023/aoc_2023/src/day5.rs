use crate::util;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct Category {
    source_start: u64,
    source_end: u64,
    dest_start: u64,
}

impl Category {
    fn from(s: &str) -> Self {
        let vals: Vec<u64> = s.split_ascii_whitespace().map(|x| unum!(x)).collect();
        let num = vals[2] - 1;
        Category{
            source_start: vals[1],
            source_end: vals[1] + num,
            dest_start: vals[0],
        }
    }
}

#[derive(Debug)]
struct Map {
    categories: Vec<Category>,
}

impl Map {
    fn calculate_dest(&self, s: u64) -> u64 {
        for c in &self.categories {
            if c.source_start <= s && s <= c.source_end {
                let offset = s - c.source_start;
                return c.dest_start + offset;
            }
        }

        return s;
    }

    fn from(lines: &[&str]) -> Self {
        let mut categories = vec![];
        for l in lines {
            if l.is_empty() {
                continue;
            }
            categories.push(Category::from(l));
        }

        Map {
            categories,
        }
    }
}

fn part_1() -> u64 {
    let data = util::read_input("5_test.input");
    let mut seeds = vec![];
    hm!(maps, String, Map);

    for section in data.split("\n\n") {
        if section.starts_with("seeds: ") {
            let t = section.replace("seeds: ", "");
            for s in t.split_ascii_whitespace() {
                seeds.push(unum!(s));
            }
            continue;
        }

        // Map section
        let lines: Vec<&str> = section.split("\n").collect();
        let mut m = Map{
            categories: vec![],
        };

        for l in &lines[1..] {
            if l.is_empty() {
                continue;
            }
            m.categories.push(Category::from(l));
        }

        let name = lines[0].replace(" map:", "");
        maps.insert(name, m);
    }

    let mut min_dest = u64::MAX;
    for s in &seeds {
        let dest_val = get_dest(*s, &maps);

        if dest_val < min_dest {
            min_dest = dest_val;
        }
    }
    min_dest
}

fn part_2() -> u64 {
    let data = util::read_input("5_test.input");
    let mut seeds = vec![];
    let mut seed_pairs = vec![];
    hm!(maps, String, Map);

    for section in data.split("\n\n") {
        if section.starts_with("seeds: ") {
            let t = section.replace("seeds: ", "");
            for s in t.split_ascii_whitespace() {
                seed_pairs.push(unum!(s));
            }
            continue;
        }

        // Map section
        let lines: Vec<&str> = section.split("\n").collect();
        let m = Map::from(&lines[1..]);

        let name = lines[0].replace(" map:", "");
        maps.insert(name, m);
    }

    for i in 0..seed_pairs.len() {
        if i % 2 == 0 {
            let start = seed_pairs[i];
            let n = seed_pairs[i + 1];
            seeds.push((start, start + n));
        }
    }

    let amaps = Arc::new(maps);
    let mut threads = vec![];
    let mut answers = vec![];
    for _ in 0..seeds.len() {
        answers.push(u64::MAX);
    }
    let answers = Arc::new(Mutex::new(answers));
    let mut idx = 0;
    for (b, e) in &seeds {
        let breaker = 100000000;
        let end = std::cmp::min(*b + breaker, *e);
        let start = *b;

        let mclone = Arc::clone(&amaps);
        let aclone = Arc::clone(&answers);
        let h = thread::spawn(move || {
            do_work_async(start, end, mclone, idx, aclone);
        });
        threads.push(h);
        idx += 1;
    }

    for h in threads {
        h.join().unwrap();
    }
    
    let r = answers.lock().unwrap();
    *r.iter().min().unwrap()
}

run!();

fn do_work_async(b: u64, e: u64, maps: Arc<HashMap<String, Map>>, i: u64, a: Arc<Mutex<Vec<u64>>>) {
    let mut min_dest = u64::MAX;
    for s in b..e {
        let dest_val = get_dest(s, &maps);

        if dest_val < min_dest {
            min_dest = dest_val;
        }
    }

    let mut vec = a.lock().unwrap();
    vec[i as usize] = min_dest;
}

fn get_dest(seed: u64, maps: &HashMap<String, Map>) -> u64 {
    let dest_val = maps["seed-to-soil"].calculate_dest(seed);
    let dest_val = maps["soil-to-fertilizer"].calculate_dest(dest_val);
    let dest_val = maps["fertilizer-to-water"].calculate_dest(dest_val);
    let dest_val = maps["water-to-light"].calculate_dest(dest_val);
    let dest_val = maps["light-to-temperature"].calculate_dest(dest_val);
    let dest_val = maps["temperature-to-humidity"].calculate_dest(dest_val);
    maps["humidity-to-location"].calculate_dest(dest_val)
}
