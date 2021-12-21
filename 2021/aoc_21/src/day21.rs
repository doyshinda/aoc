fn part_1() -> u64 {
    let max_score = 1000;
    let mut dice = (1..101).cycle();
    let mut score_p1 = 0;
    let mut pos_p1 = 8;

    let mut score_p2 = 0;
    let mut pos_p2 = 4;

    let mut num_turns = 0;
    let board = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
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
            score_p1 += board[pos_p1];
        } else {
            pos_p2 = (pos_p2 + roll) % 10;
            if pos_p2 == 0 {
                pos_p2 = 10;
            }
            score_p2 += board[pos_p2];
        }
        num_turns += 1;
    }

    if score_p1 <= score_p2 {
        num_turns * 3 * score_p1
    } else {
        num_turns * 3 * score_p2
    }
}

fn part_2() -> u64 {
    0
}

run!();
