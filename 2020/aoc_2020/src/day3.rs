use crate::util;

fn part_1() -> u64 {
    let data = util::read_input("3.input");
    let tree = '#';
    let mut tree_count = 0;
    let mut grid = Vec::new();
    for line in data.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let bottom = grid.len();
    let col_count = grid[0].len();

    let mut curr_row = 0;
    let mut curr_col = 0;
    while curr_row < bottom {
        if grid[curr_row][curr_col] == tree {
            tree_count += 1;
        }
        curr_col = (curr_col + 3) % col_count;
        curr_row += 1;
    }
    tree_count
}

fn part_2() -> u64 {
    let data = util::read_input("3.input");
    let tree = '#';
    
    let mut grid = Vec::new();
    for line in data.lines() {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    let bottom = grid.len();
    let col_count = grid[0].len();
    let mut tree_counts: Vec<u64> = vec![];

    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    for (x, y) in slopes {
        let mut tree_count = 0;
        let mut curr_row = 0;
        let mut curr_col = 0;
        while curr_row < bottom {
            if grid[curr_row][curr_col] == tree {
                tree_count += 1;
            }
            curr_col = (curr_col + x) % col_count;
            curr_row += y;
        }
        tree_counts.push(tree_count);
    }

    return tree_counts.iter().fold(1, |acc, x| acc * x);
}

run!();
