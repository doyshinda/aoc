use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("9.input");
    let moves: Vec<(char, i64)> = data
        .lines()
        .map(|line| {
            let (direction, count) = line.split_once(' ').unwrap();
            (direction.chars().next().unwrap(), inum!(count))
        })
        .collect();
    hs!(seen, (i64, i64));
    let mut knots = vec![(0, 0), (0, 0)];

    for m in &moves {
        let op = match m.0 {
            'L' => sub_y,
            'R' => inc_y,
            'U' => sub_x,
            'D' => inc_x,
            _ => unreachable!(),
        };
        for _ in 0..(m.1 as usize) {
            op(&mut knots[0]);
            for t in 1..knots.len() {
                knots[t] = update_knot(&knots[t - 1], &knots[t]);
            }
            seen.insert(knots[1]);
        }
    }

    seen.len() as u64
}

fn sub_x(val: &mut (i64, i64)) {
    val.0 -= 1;
}

fn inc_x(val: &mut (i64, i64)) {
    val.0 += 1;
}

fn sub_y(val: &mut (i64, i64)) {
    val.1 -= 1;
}

fn inc_y(val: &mut (i64, i64)) {
    val.1 += 1;
}

fn part_2() -> u64 {
    let data = util::read_input("9.input");
    let moves: Vec<(char, i64)> = data
        .lines()
        .map(|line| {
            let (direction, count) = line.split_once(' ').unwrap();
            (direction.chars().next().unwrap(), inum!(count))
        })
        .collect();
    hs!(seen, (i64, i64));
    let mut knots = vec![
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    for m in &moves {
        let op = match m.0 {
            'L' => sub_y,
            'R' => inc_y,
            'U' => sub_x,
            'D' => inc_x,
            _ => unreachable!(),
        };
        for _ in 0..(m.1 as usize) {
            op(&mut knots[0]);
            for t in 1..knots.len() {
                knots[t] = update_knot(&knots[t - 1], &knots[t]);
            }
            seen.insert(knots[9]);
        }
    }

    seen.len() as u64
}

run!();

fn update_knot(head: &(i64, i64), tail: &(i64, i64)) -> (i64, i64) {
    // Same row
    if head.0 == tail.0 {
        let diff = head.1 - tail.1;
        let delta = if diff > 0 { 1 } else { -1 };
        if diff.abs() > 1 {
            return (tail.0, tail.1 + delta);
        }
        return (tail.0, tail.1);
    }

    // Same col
    if head.1 == tail.1 {
        let diff = head.0 - tail.0;
        let delta = if diff > 0 { 1 } else { -1 };
        if diff.abs() > 1 {
            return (tail.0 + delta, tail.1);
        }
        return (tail.0, tail.1);
    }

    // Diag
    let mut resp = (tail.0, tail.1);
    let x_diff = head.0 - tail.0;
    let x_delta = if x_diff > 0 { 1 } else { -1 };
    let y_diff = head.1 - tail.1;
    let y_delta = if y_diff > 0 { 1 } else { -1 };
    if x_diff.abs() > 1 || y_diff.abs() > 1 {
        resp.0 += x_delta;
        resp.1 += y_delta;
    }

    resp
}
