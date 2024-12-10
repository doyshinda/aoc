use crate::util;

fn expand(data: &str) -> Vec<isize> {
    let chars: Vec<isize> = data.chars().map(|c| {
        let num = c.to_digit(10).unwrap();
        let num: isize = convert!(num);
        num
    }).collect();

    let mut expanded = vec![];
    let mut id = 0;
    for (idx, c) in chars.iter().enumerate() {
        if (idx % 2) == 0 {
            for _ in 0..*c {
                expanded.push(id);
            }
            id += 1;
        } else {
            for _ in 0..*c {
                expanded.push(-1);
            }
        }
    }

    expanded
}

fn part_1() -> u64 {
    let data = util::read_input("9_test.input");
    let expanded = expand(&data);
    let mut iter = expanded.iter();

    let mut condensed = vec![];
    'outer: loop {
        if let Some(front) = iter.next() {
            match front {
                -1 => {
                    while let Some(back) = iter.next_back() {
                        if *back == -1 {
                            continue;
                        }

                        condensed.push(*back);
                        continue 'outer;
                    }
                    break 'outer;
                }
                c => { condensed.push(*c); },
            }
        } else {
            break;
        }
    }

    condensed.iter().enumerate().map(|(idx, c)| -> usize {
        if *c == -1 {
            0
        } else {
            let num: usize = convert!(*c);
            idx * num
        }
    }).sum::<usize>() as u64
}

fn find_next_block(available: &mut Vec<isize>) -> Vec<(usize, isize)> {
    let mut iter = available.iter().enumerate();
    let mut next_block = vec![];
    while let Some((idx, back)) = iter.next_back() {
        if *back == -1 {
            continue;
        }

        if next_block.len() == 0 {
            next_block.push((idx, *back));
            continue
        }

        if next_block[0].1 == *back {
            next_block.push((idx, *back));
        } else {
            break;
        }
    }

    next_block
}

fn get_blocks(mut available: Vec<isize>) -> Vec<Vec<(usize, isize)>> {
    let mut blocks = vec![];
    while !available.is_empty() {
        let next_block = find_next_block(&mut available);
        if next_block.is_empty() {
            break;
        }

        for (idx, _) in &next_block {
            available.remove(*idx);
        }
        blocks.push(next_block);
    }

    blocks
}

fn part_2() -> u64 {
    let data = util::read_input("9.input");
    let mut expanded = expand(&data);

    let blocks: Vec<Vec<(usize, isize)>> = get_blocks(expanded.clone());

    for block in blocks.iter() {
        let mut replace_idx = None;
        for (idx, window) in expanded.windows(block.len()).enumerate() {
            if window.iter().all(|&x| x == -1) {
                if idx >= block[0].0 {
                    break;
                }
                replace_idx = Some(idx);
                break;
            }
        }

        if let Some(idx) = replace_idx {
            for i in 0..block.len() {
                let (bidx, v) = block[i];
                expanded[idx + i] = v;
                expanded[bidx] = -1;
            }
        }
    }

    expanded.iter().enumerate().map(|(idx, c)| -> usize {
        if *c == -1 {
            0
        } else {
            let num: usize = convert!(*c);
            idx * num
        }
    }).sum::<usize>() as u64
}

run!();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "12345";
        let expanded = expand(input);
        assert_eq!(
            vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2],
            expanded,
        );
    }
}

