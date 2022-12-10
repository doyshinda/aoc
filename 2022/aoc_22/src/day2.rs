use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("2.input");
    let x_score = 1;
    let y_score = 2;
    let z_score = 3;

    let mut total = 0;
    for line in data.split('\n') {
        let mut v = line.chars();
        let o = v.next().unwrap();
        v.next();
        let mut m = v.next().unwrap();
        match m {
            'X' => {
                total += x_score;
                m = 'A';
            }
            'Y' => {
                total += y_score;
                m = 'B';
            }
            'Z' => {
                total += z_score;
                m = 'C';
            }
            _ => (),
        }

        if o == m {
            total += 3;
            continue;
        }

        if o == 'A' && m == 'B' {
            total += 6;
            continue;
        }

        if o == 'B' && m == 'C' {
            total += 6;
            continue;
        }

        if o == 'C' && m == 'A' {
            total += 6;
            continue;
        }
    }

    total
}

fn part_2() -> u64 {
    let data = util::read_input("2.input");

    let mut total = 0;
    for line in data.split('\n') {
        let mut v = line.chars();
        let o = v.next().unwrap();
        v.next();
        let m = v.next().unwrap();
        match m {
            'X' => {
                total += lose(o);
            }
            'Y' => {
                total += draw(o);
            }
            'Z' => {
                total += win(o);
            }
            _ => (),
        }
    }

    total
}

run!();

fn lose(o: char) -> u64 {
    if o == 'A' {
        return 3;
    }

    if o == 'B' {
        return 1;
    }

    if o == 'C' {
        return 2;
    }

    0
}

fn draw(o: char) -> u64 {
    if o == 'A' {
        return 1 + 3;
    }

    if o == 'B' {
        return 2 + 3;
    }

    if o == 'C' {
        return 3 + 3;
    }

    0
}

fn win(o: char) -> u64 {
    if o == 'A' {
        return 2 + 6;
    }

    if o == 'B' {
        return 3 + 6;
    }

    if o == 'C' {
        return 1 + 6;
    }

    0
}
