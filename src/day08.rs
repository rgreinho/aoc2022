use std::fs;

pub fn day08a() -> String {
    let data = fs::read_to_string("assets/day08.txt").expect("Could not load file");
    let grid = parse_input_a(&data);
    let visible = process_input_a(grid);
    visible.to_string()
}

pub fn day08b() -> String {
    let data = fs::read_to_string("assets/day08.txt").expect("Could not load file");
    let grid = parse_input_a(&data);
    let highest_score = process_input_b(grid);
    highest_score.to_string()
}

pub fn parse_input_a(input: &str) -> Vec<Vec<u8>> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    grid
}

pub fn process_input_a(grid: Vec<Vec<u8>>) -> u32 {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let mut visible = 0;
    for row in 0..row_count {
        for col in 0..col_count {
            if is_visible(&grid, row, col) {
                println!("[{row}, {col}]");
                visible += 1;
            }
        }
    }

    visible
}

pub fn process_input_b(grid: Vec<Vec<u8>>) -> u32 {
    let row_count = grid.len();
    let col_count = grid[0].len();
    let mut scores: Vec<u32> = Vec::new();
    for row in 0..row_count {
        for col in 0..col_count {
            scores.push(scenic_score(&grid, row, col));
        }
    }

    *scores.iter().max().unwrap()
}

pub fn is_visible(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> bool {
    let row_count = grid.len();
    let col_count = grid[0].len();

    // Current tree size.
    let current_tree = grid[row][col];

    // Is it on the edge?
    if row == 0 || row == row_count - 1 || col == 0 || col == col_count - 1 {
        return true;
    }

    let left: Vec<u8> = grid[row][0..col].iter().map(|&t| t).collect();
    let right: Vec<u8> = grid[row][col + 1..col_count].iter().map(|&t| t).collect();
    let mut top: Vec<u8> = Vec::new();
    for i in 0..row {
        top.push(grid[i][col]);
    }
    // let top: Vec<u8> = (0..row).map(|x| grid[row][col]).collect();
    let mut bottom: Vec<u8> = Vec::new();
    for i in row + 1..row_count {
        bottom.push(grid[i][col]);
    }
    // let bottom: Vec<u8> = grid[row + 1..row_count][col].iter().map(|&t| t).collect();
    if left.iter().all(|&t| t < current_tree)
        || right.iter().all(|&t| t < current_tree)
        || top.iter().all(|&t| t < current_tree)
        || bottom.iter().all(|&t| t < current_tree)
    {
        return true;
    }

    false
}

pub fn scenic_score(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> u32 {
    let row_count = grid.len();
    let col_count = grid[0].len();

    // Current tree size.
    let current_tree = grid[row][col];

    // Prepare the tree list for all directions.
    let left: Vec<u8> = grid[row][0..col].iter().map(|&t| t).collect();
    let right: Vec<u8> = grid[row][col + 1..col_count].iter().map(|&t| t).collect();
    let mut top: Vec<u8> = Vec::new();
    for i in 0..row {
        top.push(grid[i][col]);
    }
    let mut bottom: Vec<u8> = Vec::new();
    for i in row + 1..row_count {
        bottom.push(grid[i][col]);
    }

    // Compute the scenic score.
    let mut left_score = 0;
    for item in left.iter().rev() {
        left_score += 1;
        if item >= &current_tree {
            break;
        }
    }
    let mut right_score = 0;
    for item in right {
        right_score += 1;
        if item >= current_tree {
            break;
        }
    }
    let mut top_score = 0;
    for item in top.iter().rev() {
        top_score += 1;
        if item >= &current_tree {
            break;
        }
    }
    let mut bottom_score = 0;
    for item in bottom {
        bottom_score += 1;
        if item >= current_tree {
            break;
        }
    }
    let score = left_score * right_score * top_score * bottom_score;
    // println!(
    //     "[{row}, {col}]: {current_tree} - {left_score}, {right_score}, {top_score}, {bottom_score} => {score}"
    // );
    score
}

#[cfg(test)]
mod test {
    use super::*;

    const RAW_INPUT: &'static str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_day08a_sample() {
        let grid = parse_input_a(RAW_INPUT);
        let visible = process_input_a(grid);
        assert_eq!(visible, 21);
    }

    #[test]
    fn test_day08b_sample() {
        let grid = parse_input_a(RAW_INPUT);
        let highest_score = process_input_b(grid);
        assert_eq!(highest_score, 8);
    }
}
