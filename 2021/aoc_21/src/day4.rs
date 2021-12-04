use crate::util;

#[derive(Clone)]
struct Slot {
    marked: bool,
    value: i32,
}

impl Slot {
    fn mark(&mut self) {
        self.marked = true;
    }
}
#[derive(Clone)]
struct Row {
    vals: Vec<Slot>,
}

impl Row {
    fn parse(s: &str) -> Row {
        Row {
            vals: s
                .split_whitespace()
                .map(|x| Slot {
                    marked: false,
                    value: x.parse::<i32>().unwrap(),
                })
                .collect(),
        }
    }

    fn is_complete(&self) -> bool {
        self.vals.iter().all(|x| x.marked)
    }
}

#[derive(Clone)]
struct Board {
    rows: Vec<Row>,
    completed: bool,
}

impl Board {
    fn parse(data: &[&str]) -> Board {
        let rows = vec![
            Row::parse(&data[1]),
            Row::parse(&data[2]),
            Row::parse(&data[3]),
            Row::parse(&data[4]),
            Row::parse(&data[5]),
        ];
        Board{rows, completed: false}
    }

    fn is_complete(&self) -> bool {
        self.rows.iter().any(|x| x.is_complete()) || {
            for i in 0..5 {
                if self.rows.iter().all(|x| x.vals[i].marked) {
                    return true;
                }
            }
        return false;
        }
    }

    fn mark(&mut self, value: i32) {
        for r in self.rows.iter_mut() {
            for v in r.vals.iter_mut() {
                if v.value == value {
                    v.mark();
                }
            }
        }
    }

    fn score(&self, last_value: i32) -> i32 {
        let mut sum = 0;
        for r in &self.rows {
            sum += r.vals.iter().filter(|x| x.marked == false).map(|x| x.value).sum::<i32>();
        }
        sum * last_value
    }
}

fn parse_input(name: &str) -> (Vec<i32>, Vec<Board>) {
    let data = util::read_input(name);
    let lines: Vec<&str> = data.split('\n').collect();
    let bingo_numbers: Vec<i32> = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut count = 1;
    let mut boards = Vec::new();
    while count < lines.len() {
        boards.push(Board::parse(&lines[count..count + 6]));
        count += 6;
    }

    (bingo_numbers, boards)
}

fn part_1() -> i32 {
    let (bingo_numbers, mut boards) = parse_input("4_a");

    for b in bingo_numbers {
        for board in boards.iter_mut() {
            board.mark(b);
            if board.is_complete() {
                return board.score(b);
            }
        }
    }

    return 0;
}

fn part_2() -> i32 {
    let (bingo_numbers, mut boards) = parse_input("4_a");

    let mut last_board = boards[0].clone();
    let mut last_num = 0;
    for b in bingo_numbers {
        for board in boards.iter_mut() {
            if board.completed {
                continue
            }
            board.mark(b);
            if board.is_complete() {
                board.completed = true;
                last_board = board.clone();
                last_num = b;
            }
        }
    }

    return last_board.score(last_num);
}

pub fn run() {
    let answer = part_1();
    println!("part_1 {:?}", answer);
    let answer = part_2();
    println!("part_2 {:?}", answer);
}

#[cfg(test)]
mod test{
    use super::*;

    const INPUT: [&'static str; 6] = [
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
    ];

    #[test]
    fn test_board_complete_row() {
        let mut board = Board::parse(&INPUT);
        assert!(!board.is_complete());
        for i in 0..5 {
            board.rows[0].vals[i].marked = true;
        }
        assert!(board.is_complete());
    }

    #[test]
    fn test_board_complete_col() {
        let mut board = Board::parse(&INPUT);
        assert!(!board.is_complete());
        for i in 0..5 {
            board.rows[i].vals[0].marked = true;
        }
        assert!(board.is_complete());
    }

    #[test]
    fn test_mark() {
        let mut board = Board::parse(&INPUT);
        assert!(!board.rows[0].vals[0].marked);
        board.mark(22);
        assert!(board.rows[0].vals[0].marked);
    }

    #[test]
    fn test_score() {
        let mut board = Board::parse(&INPUT);
        assert_eq!(board.score(10), 3000);
        board.mark(22);
        assert_eq!(board.score(10), 2780);
    }
}
