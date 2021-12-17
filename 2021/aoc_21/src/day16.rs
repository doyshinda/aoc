use crate::util;

const SUM: u64 = 0;
const PRODUCT: u64 = 1;
const MINIMUM: u64 = 2;
const MAXIMUM: u64 = 3;
const LITERAL: u64 = 4;
const GREATER: u64 = 5;
const LESS: u64 = 6;
const EQUAL: u64 = 7;

#[derive(Debug)]
struct Packet {
    version: u64,
    type_id: u64,
    literal: Option<u64>,
    vals: Vec<char>,
    subs: Vec<Packet>,
}

impl Packet {
    fn from_str(s: &str) -> Packet {
        let mut t = String::new();
        s.chars()
            .for_each(|c| t.push_str(&format!("{:>04b}", c.to_digit(16).unwrap())));
        let vals = t.chars().collect::<Vec<char>>();
        let (p, _) = Packet::from_vec(vals);
        p
    }

    fn from_vec(v: Vec<char>) -> (Packet, usize) {
        let version = version(&v);
        let type_id = type_id(&v);
        if type_id == LITERAL {
            let (l, used) = parse_literal(&v, 6);
            let end = (used + 6) as usize;
            let p = Packet {
                version: version,
                type_id: type_id,
                literal: Some(l),
                vals: v[..end].to_vec(),
                subs: Vec::new(),
            };
            return (p, end);
        }
        let mut p = Packet {
            version: version,
            type_id: type_id,
            literal: None,
            vals: v,
            subs: Vec::new(),
        };

        let len = p.parse_sub_packets();
        (p, len)
    }

    fn parse_sub_packets(&mut self) -> usize {
        let mut packets = Vec::new();

        if self.vals[6] == '0' {
            let mut num_bits = parse(&self.vals, 7, 15) as usize;
            let mut offset = 22;
            let orig = num_bits + offset;
            while num_bits > 0 {
                let (p, bits_used) = Packet::from_vec(self.vals[offset..].to_vec());
                packets.push(p);
                match num_bits.checked_sub(bits_used) {
                    Some(v) => num_bits = v,
                    _ => num_bits = 0,
                }
                offset += bits_used;
            }
            self.subs = packets;
            orig
        } else {
            let num_packets = parse(&self.vals, 7, 11) as usize;
            let mut offset = 18;
            for _i in 0..num_packets {
                let (p, bits_used) = Packet::from_vec(self.vals[offset..].to_vec());
                offset += bits_used;
                packets.push(p);
            }
            self.subs = packets;
            offset
        }
    }

    fn value(&self) -> u64 {
        match self.type_id {
            SUM => self.subs.iter().fold(0, |acc, p| acc + p.value()),
            PRODUCT => self.subs.iter().fold(1, |acc, p| acc * p.value()),
            MINIMUM => self.subs.iter().map(|p| p.value()).min().unwrap(),
            MAXIMUM => self.subs.iter().map(|p| p.value()).max().unwrap(),
            LITERAL => self.literal.unwrap(),
            GREATER => {
                if self.subs[0].value() > self.subs[1].value() {
                    1
                } else {
                    0
                }
            }
            LESS => {
                if self.subs[0].value() < self.subs[1].value() {
                    1
                } else {
                    0
                }
            }
            EQUAL => {
                if self.subs[0].value() == self.subs[1].value() {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Not supposted to be here"),
        }
    }
}

fn part_1() -> u64 {
    let input = util::read_input("16_a");
    let p = Packet::from_str(&input);
    let mut s = 0;
    let mut queue = vec![p];
    while queue.len() > 0 {
        let sp = queue.pop().unwrap();
        s += sp.version;
        for child in sp.subs {
            queue.push(child);
        }
    }
    s
}

fn part_2() -> u64 {
    let input = util::read_input("16_a");
    let p = Packet::from_str(&input);
    p.value()
}

pub fn run() {
    let start = std::time::Instant::now();
    let answer = part_1();
    println!("part_1 {:?} , took {:?}", answer, start.elapsed());

    let start = std::time::Instant::now();
    let answer = part_2();
    println!("part_2 {:?} , took {:?}", answer, start.elapsed());
}

fn parse(v: &Vec<char>, s: usize, e: usize) -> u64 {
    let mut n = 0;
    for i in s..s + e {
        n <<= 1;
        n |= v[i].to_digit(16).unwrap();
    }
    n as u64
}

fn version(v: &Vec<char>) -> u64 {
    parse(v, 0, 3)
}

fn type_id(v: &Vec<char>) -> u64 {
    parse(v, 3, 3)
}

fn parse_literal(v: &Vec<char>, s: usize) -> (u64, u64) {
    let mut next = s;
    let mut n = 0;
    while v[next] == '1' {
        n <<= 4;
        n |= parse(v, next + 1, 4);
        next += 5;
    }
    n <<= 4;
    n |= parse(v, next + 1, 4);
    next += 5;
    (n, (next - s) as u64)
}

#[test]
fn test_hex_to_int() {
    let r = '1'.to_digit(16).unwrap();
    assert_eq!(r, 1);
    let r = 'F'.to_digit(16).unwrap();
    assert_eq!(r, 15);
}

#[test]
fn test_parse_literal_packet() {
    let p = Packet::from_str("D2FE28");
    assert_eq!(p.version, 6);
    assert_eq!(p.type_id, 4);
    let (l, r) = parse_literal(&p.vals, 6);
    assert_eq!(r, 15);
    assert_eq!(l, 2021);
    assert_eq!(p.subs.len(), 0);
}

#[test]
fn test_parse_operator_type_zero() {
    let p = Packet::from_str("38006F45291200");
    assert_eq!(p.version, 1);
    assert_eq!(p.type_id, 6);
    assert_eq!(p.subs.len(), 2);
}

#[test]
fn test_parse_operator_type_one() {
    let p = Packet::from_str("EE00D40C823060");
    assert_eq!(p.version, 7);
    assert_eq!(p.type_id, 3);
    assert_eq!(p.subs.len(), 3);
}

#[test]
fn test_versions_sum() {
    let test_cases = vec![
        ("8A004A801A8002F478", 16),
        ("620080001611562C8802118E34", 12),
        ("C0015000016115A2E0802F182340", 23),
        ("A0016C880162017C3686B18A3D4780", 31),
    ];
    for (input, expected) in test_cases {
        let p = Packet::from_str(input);
        let mut s = 0;
        let mut queue = vec![p];
        while queue.len() > 0 {
            let sp = queue.pop().unwrap();
            s += sp.version;
            for child in sp.subs {
                queue.push(child);
            }
        }
        assert_eq!(s, expected);
    }
}

#[test]
fn test_values() {
    let p = Packet::from_str("C200B40A82");
    assert_eq!(p.value(), 3);

    let p = Packet::from_str("04005AC33890");
    assert_eq!(p.value(), 54);

    let p = Packet::from_str("880086C3E88112");
    assert_eq!(p.value(), 7);

    let p = Packet::from_str("CE00C43D881120");
    assert_eq!(p.value(), 9);

    let p = Packet::from_str("D8005AC2A8F0");
    assert_eq!(p.value(), 1);

    let p = Packet::from_str("F600BC2D8F");
    assert_eq!(p.value(), 0);

    let p = Packet::from_str("9C005AC2F8F0");
    assert_eq!(p.value(), 0);

    let p = Packet::from_str("9C0141080250320F1802104A08");
    assert_eq!(p.value(), 1);
}
