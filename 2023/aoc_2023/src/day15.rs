use crate::util;

const MINUS: char = '-';
const ADD: char = '=';

#[derive(Debug)]
struct Op {
    label: String,
    op: char,
    val: u32,
}

impl Op {
    fn from(s: &str) -> Self {
        if s.ends_with("-") {
            return Self{
                label: s.chars().take(s.len() - 1).collect::<String>(),
                op: MINUS,
                val: 0,
            }
        }

        let (left, right) = s.split_once("=").unwrap();

        let label = left;
        let mut iter = right.chars();

        let mut val = 0;
        while let Some(n) = iter.next() {
            val *= 10;
            match n.to_digit(10) {
                Some(v) => val += v,
                None => continue,
            }
        }

        Self {
            label: label.to_string(),
            op: ADD,
            val: val as u32,
        }
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_len: u32,
}

#[derive(Debug)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn remove_lens(&mut self, label: &String) {
        let mut idx = None;
        for (i, l) in self.lenses.iter().enumerate() {
            if l.label == *label {
                idx = Some(i);
                break;
            }
        }

        if idx.is_some() {
            self.lenses.remove(idx.unwrap());
        }
    }

    fn add_lens(&mut self, o: Op) {
        for (i, l) in self.lenses.iter().enumerate() {
            if l.label == o.label {
                self.lenses[i] = Lens{
                    label: o.label,
                    focal_len: o.val,
                };
                return;
            }
        }

        self.lenses.push(Lens{
            label: o.label,
            focal_len: o.val,
        })
    }
}

fn part_1() -> u64 {
    let data = util::read_input("15_test.input");
    let mut vals = vec![];
    for v in data.split(',') {
        vals.push(hash(v));
    }
    vals.iter().sum::<u32>() as u64
}

fn part_2() -> u64 {
    let mut boxes = vec![];
    for _ in 0..256 {
        boxes.push(Box{lenses: vec![]});
    }

    let data = util::read_input("15.input");

    for v in data.split(',') {
        let v = v.trim_end_matches("\n");
        let op = Op::from(v);
        let box_num = hash(&op.label) as usize;
        match op.op {
            MINUS => boxes[box_num].remove_lens(&op.label),
            _ => boxes[box_num].add_lens(op),
        }
    }

    for b in &boxes {
        if b.lenses.len() == 0 {
            continue;
        }
    }

    let mut total = 0;
    for i in 0..256 {
        total += sum_box(i, &boxes[i]);
    }
    
    total
}

run!();
fn sum_box(i: usize, b: &Box) -> u64 {
    let box_val = i as u32 + 1;
    let mut total = 0;
    for (i, l) in b.lenses.iter().enumerate() {
        let fp = box_val * (i as u32 + 1) * l.focal_len;
        total += fp;
    }

    total as u64
}

fn hash(s: &str) -> u32 {
    let mut total: u32 = 0;
    for c in s.chars() {
        if c == '\n' {
            continue;
        }
        total += c as u32;
        total *= 17;
        total %= 256;
    }

    total
}