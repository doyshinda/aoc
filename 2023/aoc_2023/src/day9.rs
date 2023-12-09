use crate::util;

#[derive(Debug)]
struct History {
    vals: Vec<Vec<i64>>,
}

impl History {
    fn from(l: &str) -> Self {
        let v: Vec<i64> = l.split_ascii_whitespace().map(|x| inum!(x)).collect();
        let vals = vec![v];
        Self { vals }
    }

    fn reduce(&mut self) -> i64 {
        let mut idx = 0;
        let mut done = false;
        while !done {
            let mut diffs = vec![];
            for i in 1..self.vals[idx].len() {
                diffs.push(self.vals[idx][i] - self.vals[idx][i - 1]);
            }
            done = diffs.iter().all(|x| *x == 0);
            if done {
                diffs.push(0);
            }
            self.vals.push(diffs);
            idx += 1;
        }

        // self.vals[idx - 1].push(0);
        for j in (1..self.vals.len()).rev() {
            let next = self.vals[j][self.vals[j].len() - 1];
            let last = self.vals[j - 1][self.vals[j - 1].len() - 1];
            // println!("{} -> {} {}", j, next, last);
            self.vals[j - 1].push(next + last);
        }

        return self.vals[0][self.vals[0].len() - 1];
    }

    fn reduce_front(&mut self) -> i64 {
        let mut idx = 0;
        let mut done = false;
        while !done {
            let mut diffs = vec![];
            for i in 1..self.vals[idx].len() {
                diffs.push(self.vals[idx][i] - self.vals[idx][i - 1]);
            }
            done = diffs.iter().all(|x| *x == 0);
            if done {
                diffs.insert(0, 0);
            }
            self.vals.push(diffs);
            idx += 1;
        }

        // self.vals[idx - 1].push(0);
        for j in (1..self.vals.len()).rev() {
            let next = self.vals[j][0];
            let last = self.vals[j - 1][0];
            self.vals[j - 1].insert(0, last - next);
        }

        return self.vals[0][0];
    }
}

fn part_1() -> u64 {
    let data = util::read_input("9.input");
    let mut readings = vec![];
    for line in data.split('\n') {
        if line.is_empty() {
            continue;
        }

        readings.push(History::from(line));
    }
    let mut sum = 0;
    for r in &mut readings {
        sum += r.reduce();
        // dbg!(&r);
    }
    sum as u64
}

fn part_2() -> u64 {
    let data = util::read_input("9.input");
    let mut readings = vec![];
    for line in data.split('\n') {
        if line.is_empty() {
            continue;
        }

        readings.push(History::from(line));
    }
    let mut sum = 0;
    for r in &mut readings {
        sum += r.reduce_front();
        // dbg!(&r);
    }
    if sum < 0 {
        println!("{}", sum);
        return 0;
    }
    sum as u64
}

run!();
