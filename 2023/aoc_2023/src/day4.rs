use crate::util;
use std::collections::HashSet;

fn part_1() -> u64 {
    let data = util::read_input("4.input");
    let mut sum = 0;
    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once(" | ").unwrap();
        let (_, winning_nums) = left.split_once(": ").unwrap();
        let winning_nums: Vec<u64> = winning_nums.split_ascii_whitespace().map(|x| unum!(x)).collect();
        let card_nums: Vec<u64> = right.split_ascii_whitespace().map(|x| unum!(x)).collect();

        let wnset: HashSet<u64> = hs!(winning_nums);
        let cnset: HashSet<u64> = hs!(card_nums);

        let inter: Vec<&u64> = wnset.intersection(&cnset).collect();
        let num_inter = inter.len();
        if num_inter == 0 {
            continue;
        }

        sum += 2_u64.pow((num_inter - 1) as u32);
    }

    sum
}

fn part_2() -> u64 {
    let data = util::read_input("4.input");

    hm!(card_counts, u64, u64);

    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once(" | ").unwrap();
        let (card, winning_nums) = left.split_once(": ").unwrap();
        let card = card.replace("Card ", "");
        let card = card.trim();
        let card = unum!(card);

        hm_inc!(card_counts, card, 1);

        let winning_nums: Vec<u64> = winning_nums.split_ascii_whitespace().map(|x| unum!(x)).collect();
        let card_nums: Vec<u64> = right.split_ascii_whitespace().map(|x| unum!(x)).collect();

        let wnset: HashSet<u64> = hs!(winning_nums);
        let cnset: HashSet<u64> = hs!(card_nums);

        let inter: Vec<&u64> = wnset.intersection(&cnset).collect();
        let num_inter = inter.len();
        if num_inter == 0 {
            continue;
        }

        let start = card as usize + 1;
        let end = start + num_inter;

        let multiplier = card_counts.get(&card).unwrap().clone();
        for c in start..end {
            // log!("[{:02}]: {} -> {}", card, c, multiplier);
            hm_inc!(card_counts, c as u64, 1 * multiplier);
        }
        // dbg!(&card_counts);

        // sum += 2_u64.pow((num_inter - 1) as u32);
    }

    let mut sum = 0;
    for (_, v) in &card_counts {
        sum += *v;
    }

    sum
}

run!();
