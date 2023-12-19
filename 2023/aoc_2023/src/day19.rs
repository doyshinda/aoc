use crate::util;

#[derive(Debug)]
struct Instruction {
    m_val: char,
    num: u64,
    next: String,
    op: char,
}

impl Instruction {
    fn from(l: &str) -> Vec<Self> {
        let mut v = vec![];

        for i in l.split(',') {
            if i.contains('<') {
                let (m_val, other) = i.split_once('<').unwrap();
                let (n, next) = other.split_once(":").unwrap();
                v.push(Instruction{
                    m_val: m_val.chars().nth(0).unwrap(),
                    num: unum!(n),
                    next: next.to_string(),
                    op: '<',
                });
                continue;
            }

            if i.contains('>') {
                let (m_val, other) = i.split_once('>').unwrap();
                let (n, next) = other.split_once(":").unwrap();
                v.push(Instruction{
                    m_val: m_val.chars().nth(0).unwrap(),
                    num: unum!(n),
                    next: next.to_string(),
                    op: '>',
                });
                continue;
            }
            v.push(Instruction{
                m_val: 'e',
                num: 0,
                next: i.to_string(),
                op: '=',
            });
        }

        v
    }

    fn execute(&self, p: &Part) -> Option<String> {
        let cmp = match self.m_val {
            'x' => p.x_val,
            'm' => p.m_val,
            'a' => p.a_val,
            's' => p.s_val,
            _ => return Some(self.next.clone()),
        };

        match self.op {
            '>' => {
                if cmp > self.num {
                    return Some(self.next.clone());
                }
            }
            '<' => {
                if cmp < self.num {
                    return Some(self.next.clone());
                }
            }
            _ => return Some(self.next.clone()),
        }

        None
    }
}

#[derive(Debug)]
struct Part {
    x_val: u64,
    m_val: u64,
    a_val: u64,
    s_val: u64,
}

impl Part {
    fn from(l: &str) -> Self {
        let v = &l[1..].trim_end_matches("}");
        let vals: Vec<&str> = v.split(',').collect();
        Self {
            x_val: unum!(&vals[0][2..]),
            m_val: unum!(&vals[1][2..]),
            a_val: unum!(&vals[2][2..]),
            s_val: unum!(&vals[3][2..]),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    instructions: Vec<Instruction>,
}

impl Workflow {
    fn from(l: &str) -> Self {
        let s_idx = l.find('{').unwrap();
        let name = &l[0..s_idx];
        let instructions = &l[s_idx + 1..].trim_end_matches("}");
        Self {
            name: name.to_string(),
            instructions: Instruction::from(instructions),
        }
    }

    fn execute(&self, part: &Part) -> String {
        for i in &self.instructions {
            if let Some(n) = i.execute(part) {
                return n;
            }
        }

        return "R".to_string();
    }
}

fn part_1() -> u64 {
    let data = util::read_input("19.input");
    let (workflows, parts) = data.trim().split_once("\n\n").unwrap();
    hm!(wf_map, String, Workflow);
    for w in workflows.split('\n') {
        let wf = Workflow::from(w);
        wf_map.insert(wf.name.clone(), wf);
    }

    let mut accepted = vec![];

    for p in parts.split("\n") {
        let part = Part::from(p);
        let mut next = "in".to_string();
        while next != "A".to_string() && next != "R".to_string() {
            next = wf_map[&next].execute(&part);
        }

        if next == "A".to_string() {
            accepted.push(part);
        }
    }

    let mut total = 0;
    for p in accepted {
        total += p.x_val;
        total += p.m_val;
        total += p.a_val;
        total += p.s_val;
    }
    total
}

fn part_2() -> u64 {
    0
}

run!();
