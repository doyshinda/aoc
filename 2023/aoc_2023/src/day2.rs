use crate::util;

const MAX_RED: u64 = 12;
const MAX_GREEN: u64 = 13;
const MAX_BLUE: u64 = 14;

// 237 too low
fn part_1() -> u64 {
    let data = util::read_input("2.input");

    let mut sum = 0;
    'outer: for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        let (game, rest) = line.split_once(": ").unwrap();
        let game = game.replace("Game ", "");
        let game = game.parse::<u64>().unwrap();

        let sets = rest.split("; ");
        for set in sets {
            let cubes = set.split(", ");
            for cube in cubes {
                let (n, color) = cube.split_once(" ").unwrap();
                let max_poss = match color {
                    "blue" => MAX_BLUE,
                    "green" => MAX_GREEN,
                    _ => MAX_RED,
                };
                if n.parse::<u64>().unwrap() > max_poss {
                    continue 'outer;
                }
            }
        }
        sum += game;
    }
    sum
}

fn part_2() -> u64 {
    let data = util::read_input("2.input");

    let mut sum = 0;
    for line in data.split("\n") {
        if line.is_empty() {
            continue;
        }
        let (_, rest) = line.split_once(": ").unwrap();

        let mut color_counts = vec![0, 0, 0];
        let sets = rest.split("; ");

        for set in sets {
            let cubes = set.split(", ");
            for cube in cubes {
                let (n, color) = cube.split_once(" ").unwrap();
                let idx = match color {
                    "blue" => 2,
                    "green" => 1,
                    _ => 0,
                };
                let cc = unum!(n);
                if cc > color_counts[idx] {
                    color_counts[idx] = cc;
                    // dbg!(&color_counts);
                }
            }
        }
        log!("c{:?}", color_counts);
        let power = color_counts.iter().fold(1, |acc, e| {
            if *e == u64::MAX {
                acc
            } else {
                acc * e
            }});
        log!("power: {}", power);
        sum += power;
    }
    sum
}

run!();
