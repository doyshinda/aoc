use crate::util;

const J: u64 = 1;
const A: u64 = 14;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}

impl HandType {
    fn from(c: &Vec<u64>) -> Self {
        hm!(counts, u64, u64);
        for v in c {
            hm_inc!(counts, *v, 1);
        }

        if counts.len() == 1 {
            return HandType::Five;
        }

        let values: Vec<u64> = counts.values().map(|x| *x).collect();
        for v in &values {
            if *v == 4 {
                return HandType::Four;
            }
        }

        if values == [2, 3] || values == [3, 2] {
            return HandType::Full;
        }

        for v in &values {
            if *v == 3 {
                return HandType::Three;
            }
        }

        if values == [2, 2, 1] ||
            values == [2, 1, 2] ||
            values == [1, 2, 2] {
            return HandType::Two;
        }

        for v in &values {
            if *v == 2 {
                return HandType::One;
            }
        }

        HandType::High
    }
}

#[derive(Debug)]
struct Hand {
    orig: String,
    cards: Vec<u64>,
    bid: u64,
    htype: HandType,
}

impl Hand {
    fn from(l: &str) -> Self {
        let (left, right) = l.split_once(" ").unwrap();
        let mut cards = vec![];
        let mut orig = String::new();
        for c in left.chars() {
            let v = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => unimplemented!(),
            };
            orig.push(c);

            cards.push(v);
        }

        let htype = HandType::from(&cards);

        Hand {
            orig: orig,
            cards: cards,
            bid: unum!(right),
            htype: htype,
        }
    }

    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        if self.htype > other.htype {
            return std::cmp::Ordering::Greater;
        }

        if self.htype == other.htype {
            for (a, b) in std::iter::zip(self.cards.iter(), other.cards.iter()) {
                if a > b {
                    return std::cmp::Ordering::Greater;
                }
                if a < b {
                    return std::cmp::Ordering::Less;
                }
            }
            return std::cmp::Ordering::Equal;
        }

        return std::cmp::Ordering::Less;
    }
}

fn part_1() -> u64 {
    let data = util::read_input("7.input");
    let mut hands = vec![];
    for line in data.split('\n') {
        if line.is_empty() {
            continue;
        }
        hands.push(Hand::from(line));
    }

    hands.sort_by(|a, b| a.cmp(b));
    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        let val = (i + 1) as u64 * h.bid;
        sum += val;
    }
    sum
}

impl HandType {
    fn from2(c: &Vec<u64>) -> HandType {
        calc_best(c)
    }
}

impl Hand {
    fn from2(l: &str) -> Hand {
        let (left, right) = l.split_once(" ").unwrap();
        let mut cards = vec![];
        let mut orig = String::new();
        for c in left.chars() {
            let v = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 1,
                'T' => 10,
                '9' => 9,
                '8' => 8,
                '7' => 7,
                '6' => 6,
                '5' => 5,
                '4' => 4,
                '3' => 3,
                '2' => 2,
                _ => unimplemented!(),
            };
            orig.push(c);

            cards.push(v);
        }

        let htype = HandType::from2(&cards);

        Hand {
            orig: orig,
            cards: cards,
            bid: unum!(right),
            htype: htype,
        }
    }
}

fn part_2() -> u64 {
    let data = util::read_input("7.input");
    let mut hands = vec![];
    for line in data.split('\n') {
        if line.is_empty() {
            continue;
        }
        hands.push(Hand::from2(line));
    }

    hands.sort_by(|a, b| a.cmp(b));

    let mut sum = 0;
    for (i, h) in hands.iter().enumerate() {
        let val = (i + 1) as u64 * h.bid;
        sum += val;
    }
    sum
}

fn calc_orig(c: &Vec<u64>) -> String {
    let mut s = String::new();
    for v in c {
        let add = match *v {
            1 => 'J',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            10 => 'T',
            12 => 'Q',
            13 => 'K',
            14 => 'A',
            _ => todo!(),
        };
        s.push(add);
    }

    return s;
}

run!();

fn calc_best(c: &Vec<u64>) -> HandType {
    let j_count = c.iter().filter(|&&x| x == J).count();
    if j_count == 0 {
        return HandType::from(c);
    }
    if j_count == 4 || j_count == 5 {
        return HandType::Five;
    }

    let mut idx = 0;
    let mut high_hand = HandType::One;
    for v in c {
        if *v != J {
            idx += 1;
            continue;
        }

        let mut temp = c.clone();
        for candidate in 2..=A {
            if candidate == 11 {
                continue;
            }
            temp[idx] = candidate;
            let htype = calc_best(&temp);
            if htype > high_hand {
                high_hand = htype;
            }
        }

        idx += 1;
    }

    high_hand
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_best() {
        let cards = vec![13, 10, 1, 1, 10];
        assert_eq!(HandType::Four, calc_best(&cards));
    }

    #[test]
    fn test_equal() {
        let a = Hand::from2("92J9J 80");
        let b = Hand::from2("92J9J 80");
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);

        let a = Hand::from2("TJJJJ 80");
        let b = Hand::from2("TJJJJ 80");
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Equal);
        assert_eq!(a.htype, HandType::Five);
    }

    #[test]
    fn test_less() {
        let a = Hand::from2("9JJJJ 80");
        let b = Hand::from2("TJJJJ 80");
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_greater() {
        let a = Hand::from2("QJJJJ 80");
        let b = Hand::from2("JJTJJ 80");
        assert_eq!(a.cmp(&b), std::cmp::Ordering::Greater);
        assert_eq!(a.cards, vec![12, 1, 1, 1, 1]);
        assert_eq!(b.cards, vec![1, 1, 10, 1, 1]);
    }

    #[test]
    fn test_all_jokers() {
        let a = Hand::from2("JJJJJ 80");
        assert_eq!(a.htype, HandType::Five);
    }

    #[test]
    fn test_full() {
        let a = Hand::from2("82828 875");
        assert_eq!(a.htype, HandType::Full);

        let a = Hand::from2("33366 350");
        assert_eq!(a.htype, HandType::Full);
    }

    #[test]
    fn test_four() {
        let a = Hand::from2("33J43 320");
        assert_eq!(a.htype, HandType::Four);
    }

    #[test]
    fn test_three() {
        let a = Hand::from2("AT777 727");
        assert_eq!(a.htype, HandType::Three);

        let a = Hand::from2("ATJ77 727");
        assert_eq!(a.htype, HandType::Three);
    }
}
