use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

const BOARD: [u8; 11] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
const MAX_SCORE: u8 = 21;
lazy_static! {
    static ref CACHE: Mutex<HashMap<(u8, u8, u8, u8, u8), (u64, u64)>> = Mutex::new(HashMap::new());
}

fn part_1() -> u64 {
    let max_score = 1000;
    let mut dice = (1..101).cycle();
    let mut score_p1: u64 = 0;
    let mut pos_p1 = 8;

    let mut score_p2: u64 = 0;
    let mut pos_p2 = 4;

    let mut num_turns = 0;
    while score_p1 < max_score && score_p2 < max_score {
        let mut roll = 0;
        for _ in 0..3 {
            roll += dice.next().unwrap();
        }

        if num_turns % 2 == 0 {
            pos_p1 = (pos_p1 + roll) % 10;
            if pos_p1 == 0 {
                pos_p1 = 10;
            }
            score_p1 += BOARD[pos_p1] as u64;
        } else {
            pos_p2 = (pos_p2 + roll) % 10;
            if pos_p2 == 0 {
                pos_p2 = 10;
            }
            score_p2 += BOARD[pos_p2] as u64;
        }
        num_turns += 1;
    }

    if score_p1 <= score_p2 {
        (num_turns * 3 * score_p1) as u64
    } else {
        (num_turns * 3 * score_p2) as u64
    }
}

fn turn(s1: u8, p1: u8, s2: u8, p2: u8, player: u8) -> (u64, u64) {
    let m = CACHE.lock().unwrap();
    let key = (s1, p1, s2, p2, player);
    if let Some(ans) = m.get(&key) {
        return *ans;
    }
    drop(m);

    if s1 >= 21 {
        return (1, 0);
    }
    if s2 >= 21 {
        return (0, 1);
    }

    let mut wins_1 = 0;
    let mut wins_2 = 0;
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let roll = i + j + k;
                if player == 1 {
                    let mut new_p1 = (p1 + roll) % 10;
                    if new_p1 == 0 {
                        new_p1 = 10;
                    }
                    let new_score_p1 = s1 + BOARD[new_p1 as usize];
                    let (rw1, rw2) = turn(new_score_p1, new_p1, s2, p2, 2);
                    wins_1 += rw1;
                    wins_2 += rw2;
                } else {
                    let mut new_p2 = (p2 + roll) % 10;
                    if new_p2 == 0 {
                        new_p2 = 10;
                    }
                    let new_score_p2 = s2 + BOARD[new_p2 as usize];
                    let (rw1, rw2) = turn(s1, p1, new_score_p2, new_p2, 1);
                    wins_1 += rw1;
                    wins_2 += rw2;
                }
            }
        }
    }
    let key = (s1, p1, s2, p2, player);
    let mut m = CACHE.lock().unwrap();
    m.insert(key, (wins_1, wins_2));
    (wins_1, wins_2)
}

fn part_2() -> u64 {
    let score_p1 = 0;
    let pos_p1 = 8;

    let score_p2 = 0;
    let pos_p2 = 4;

    let (a, b) = turn(score_p1, pos_p1, score_p2, pos_p2, 1);
    a.max(b)
}

run!();
