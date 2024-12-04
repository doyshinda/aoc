use crate::util;

#[derive(Debug,PartialEq)]
enum Direction {
    Inc,
    Dec,
    Eq,
}

impl Direction {
    fn new(diff: i64) -> Self {
        if diff > 0 {
            Direction::Dec
        } else if diff < 0 {
            Direction::Inc
        } else {
            Direction::Eq
        }
    }
}

fn validate(nums: &[i64]) -> bool {
    if nums.len() < 2 {
        return true;
    }

    let diff = nums[0] - nums[1];
    let mut prev = Direction::new(diff);
    for i in 1..nums.len() {
        let idiff = nums[i - 1].abs_diff(nums[i]);
        let curr = Direction::new(nums[i - 1] - nums[i]);
        if curr != prev {
            return false;
        }

        if !(idiff >= 1 && idiff <= 3) {
            return false;
        }

        prev = curr;
    }

    return true;
}

fn part_1() -> u64 {
    let data = util::read_input("2.input");
    let vals = data.split("\n");

    let mut sum = 0;
    for line in vals {
        let nums: Vec<i64> = line.split_ascii_whitespace().map(|x| inum!(x)).collect();
        let diff = nums[0] - nums[1];

        if diff == 0 {
            continue;
        }

        if validate(&nums) {
            sum += 1;
        }
    }

    sum
}

fn part_2() -> u64 {
    let data = util::read_input("2.input");
    let vals = data.split("\n");

    let mut sum = 0;
    for line in vals {
        let nums: Vec<i64> = line.split_ascii_whitespace().map(|x| inum!(x)).collect();
        log!("{:?}", nums);
        if validate(&nums) {
            log!("safe as is {:?}", nums);
            sum += 1;
            continue;
        }

        // If we remove first value, is it valid?
        if validate(&nums[1..]) {
            sum += 1;
            log!("safe minus first {:?}", nums);
            continue;
        }

        for i in 1..nums.len() {
            let mut num_clone = nums.clone();
            num_clone.remove(i);
            if validate(&num_clone) {
                log!("safe replaced: {:?}", num_clone);
                sum += 1;
                break;
            }
        }
    }

    sum
}

run!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direction() {
        let a = Direction::new(1);
        let b = Direction::new(1);
        assert_eq!(a, b);

        let b = Direction::new(0);
        assert!(a != b)
    }

    #[test]
    fn test_validate_single() {
        let vals = vec![1];
        assert!(validate(&vals))
    }

    #[test]
    fn test_validate_double() {
        let vals = vec![1, 2];
        assert!(validate(&vals))
    }

    #[test]
    fn test_validate_multiple() {
        let vals = vec![1, 2, 5];
        assert!(validate(&vals));

        assert!(!validate(&vec![1, 2, 3, 8]));
        assert!(!validate(&vec![9, 1, 2, 3, 4]));
        assert!(!validate(&vec![1, 2, 8, 3, 4]));
        assert!(validate(&vec![20, 21, 22, 23, 26]));
    }
}
