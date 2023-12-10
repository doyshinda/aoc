use crate::util;

const OPEN: i64 = -2;
const CLOSE: i64 = -1;
const SEPARATOR: i64 = -3;


#[derive(Debug, Eq, Clone, PartialEq)]
struct Packet {
    vals: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
enum Item {
    Num(i64),
    List(Vec<Item>),
}

impl Item {
    fn push(self, v: i64) -> Self {
        match self {
            Item::List(mut vals) => {
                vals.push(Item::Num(v));
                Item::List(vals)
            }
            _ => Item::List(vec![Item::Num(v)]),
        }
    }
}

// 6010 too low
// 6505 too high
fn part_1() -> i64 {
    let data = util::read_input("13.input");
    let mut in_order = 0;
    for (idx, packet_pair) in data.split("\n\n").enumerate() {
        let mut packets = packet_pair.lines();
        let a = parse_packet(packets.next().unwrap());
        let b = parse_packet(packets.next().unwrap());
        if are_in_order(&a, &b) {
            in_order += idx + 1;
        }
    }
    in_order as i64
}

fn part_2() -> u64 {
    log!("Ran part 2");
    0
}

run!();
fn to_str(c: &[i64]) -> String {
    let mut s = String::new();
    for n in c {
        match *n {
            OPEN => s.push('['),
            SEPARATOR => s.push(','),
            CLOSE => s.push(']'),
            _ => s.push_str(&n.to_string()),
        }
    }
    s
}


fn parse_packet(p: &str) -> Packet {
    let chars: Vec<char> = p.chars().collect();
    let mut vals = Vec::new();
    let mut skip = false;

    for r in 0..chars.len() {
        if skip {
            skip = false;
            continue;
        }

        match chars[r] {
            '[' => vals.push(OPEN),
            ']' => vals.push(CLOSE),
            ',' => vals.push(SEPARATOR),
            v => {
                if r + 1 < chars.len() && chars[r + 1].is_ascii_digit() {
                    vals.push(inum!(format!("{}{}", v, chars[r + 1])));
                    skip = true;
                } else {
                    vals.push(inum!(v.to_string()));
                }
            }
        }
    }

    Packet {vals}
}

fn parse_packet_nested(p: &str) -> Vec<Item> {
    let chars: Vec<char> = p.chars().collect();
    let mut skip = false;

    let mut result = Vec::new();
    // let mut list = Vec::new();
    let mut i = 0;

    for r in 0..chars.len() {
        if skip {
            skip = false;
            continue;
        }

        match chars[r] {
            '[' => {
                i += 1;
                while i < result.len() {
                    result.push(Item::Num(0));
                }
            }
            ']' => {
                // result.push(Item::List(list.clone()));
                i -= 1;
            }
            ',' => continue,
            v => {
                if r + 1 < chars.len() && chars[r + 1].is_ascii_digit() {
                   result[i] = result[i].clone().push(inum!(format!("{}{}", v, chars[r + 1])));
                    skip = true;
                } else {
                   result[i] = result[i].clone().push(inum!(v.to_string()));
                }
            }
        }
    }

    result
}

fn are_in_order(left: &Packet, right: &Packet) -> bool {
    // return both_lists(left.vals[1..].to_vec(), right.vals[1..].to_vec());
    return both_lists(left.vals[..].to_vec(), right.vals[..].to_vec());
}

fn both_lists(mut left: Vec<i64>, mut right: Vec<i64>) -> bool {
    let mut ld = 0;
    let mut rd = 0;

    loop {
        println!();
        println!("[{:02}] left: {:?}", ld, to_str(&left));
        println!("[{:02}] riht: {:?}", rd, to_str(&right));

        let a = left[0];
        let b = right[0];

        if a == SEPARATOR {
            left.remove(0);
            continue;
        }

        if b == SEPARATOR {
            right.remove(0);
            continue;
        }

        if a == OPEN {
            ld += 1;
            left.remove(0);
            continue;
        }

        if a == CLOSE {
            ld -= 1;
            left.remove(0);
            continue;
        }

        if b == OPEN {
            rd += 1;
            right.remove(0);
            continue;
        }

        if b == CLOSE {
            rd -= 1;
            right.remove(0);
            continue;
        }

        if left.is_empty() {
            return true;
        }

        if right.is_empty() {
            return false;
        }

        // Both integers
        if a > -1 && b > -1 {
            if a < b {
                return true;
            }

            if a > b {
                return false;
            }

            left.remove(0);
            right.remove(0);
            continue;
        }

        // Both are lists
        if a == OPEN && b == OPEN {
            left.remove(0);
            right.remove(0);
            continue
        }

        // Left is list, right is an integer
        if a == OPEN && b > -1 {
            right.insert(1, CLOSE);
            left.remove(0);
            continue;
        }

        // Left is an integer, right is a list
        if a > -1 && b == OPEN {
            left.insert(1, CLOSE);
            right.remove(0);
            continue;
        }

        if ld < rd {
            return false;
        }
    }
}

#[test]
fn test_both_lists_one() {
    let a = Packet{
        vals: vec![1,1,3,1,1],
    };

    let b = Packet{
        vals: vec![1,1,5,1,1],
    };

    assert_eq!(true, are_in_order(&a, &b));
    assert_eq!(false, are_in_order(&b, &a));
}

#[test]
fn test_both_lists_two() {
    let a = parse_packet("[[1],[2,3,4]]");
    let b = parse_packet("[[1],4]");

    assert_eq!(true, are_in_order(&a, &b));
    assert_eq!(false, are_in_order(&b, &a));
}

#[test]
fn test_both_lists_three() {
    let a = parse_packet("[9]");
    let b = parse_packet("[[8,7,6]]");

    assert_eq!(false, are_in_order(&a, &b));
}

#[test]
fn test_both_lists_four() {
    let a = parse_packet("[[4,4],4,4]");
    let b = parse_packet("[[4,4],4,4,4]");

    assert_eq!(true, are_in_order(&a, &b));
    assert_eq!(false, are_in_order(&b, &a));
}

#[test]
fn test_both_lists_five() {
    let a = parse_packet("[7,7,7,7]");
    let b = parse_packet("[7,7,7]");

    assert_eq!(false, are_in_order(&a, &b));
}

#[test]
fn test_both_lists_six() {
    let a = parse_packet("[]");
    let b = parse_packet("[3]");

    assert_eq!(true, are_in_order(&a, &b));
}

#[test]
fn test_both_lists_seven() {
    let a = parse_packet("[[[]]]");
    let b = parse_packet("[[]]");

    assert_eq!(false, are_in_order(&a, &b));
}

#[test]
fn test_both_lists_eight() {
    let a = parse_packet("[1,[2,[3,[4,[5,6,10]]]],8,9]");
    let b = parse_packet("[1,[2,[3,[4,[5,6,0]]]],8,9]");

    assert_eq!(false, are_in_order(&a, &b));
    assert_eq!(true, are_in_order(&b, &a));
}

#[test]
fn test_both_lists_nine() {
    let a = parse_packet("[[]]");
    let b = parse_packet("[[]]");

    assert_eq!(true, are_in_order(&a, &b));
}

#[test]
fn test_parse_packet() {
    let a = parse_packet("[10,[4]]");
    assert_eq!(vec![OPEN, 10, SEPARATOR, OPEN, 4, CLOSE, CLOSE], a.vals);
}

#[test]
fn test_parse_packet_nested() {
    let a = parse_packet_nested("[10,[4], 8]");
    let expected = Item::List(vec![
        Item::Num(10),
        Item::List(vec![Item::Num(4)]),
    ]);
    assert_eq!(expected, a);
}
