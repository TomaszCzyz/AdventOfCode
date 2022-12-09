use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file_name: &str) -> Vec<Vec<i32>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|result| result.unwrap())
        .map(|line| line.chars()
            .map(|char| char.to_string().parse::<i32>().unwrap())
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn treetop_tree_house_part_1(file_name: &str) -> usize {
    let rows = read_input(file_name);
    // println!("{}",rows[1][0]);
    let length = rows.len();
    let width = rows[0].len();

    let mut visible_cords = HashSet::new();

    // from left to right
    let mut row_index = 1;

    while row_index < length - 1 {
        let mut max_height = rows[row_index][0];
        let mut col_index = 1;

        while col_index < width - 1 {
            let current_height = rows[row_index][col_index];

            analyze_visibility("->", &mut visible_cords, row_index, col_index, &mut max_height, current_height);

            if current_height == 9 {
                break;
            }

            col_index += 1;
        }
        row_index += 1;
    }

    // from right to left
    let mut row_index = 1;

    while row_index < length - 1 {
        let mut max_height = rows[row_index][width - 1];
        let mut col_index = width - 1;

        while col_index > 0 {
            let current_height = rows[row_index][col_index];

            analyze_visibility("<-", &mut visible_cords, row_index, col_index, &mut max_height, current_height);

            if current_height == 9 {
                break;
            }

            col_index -= 1;
        }
        row_index += 1;
    }

    // from top to bottom
    let mut col_index = 1;

    while col_index < width - 1 {
        let mut max_height = rows[0][col_index];
        let mut row_index = 1;

        while row_index < length - 1 {
            let current_height = rows[row_index][col_index];

            analyze_visibility("↓", &mut visible_cords, row_index, col_index, &mut max_height, current_height);

            if current_height == 9 {
                break;
            }
            row_index += 1;
        }
        col_index += 1;
    }

    // from bottom to top
    let mut col_index = 1;

    while col_index < width - 1 {
        let mut max_height = rows[length - 1][col_index];
        let mut row_index = length - 1;

        while row_index > 0 {
            let current_height = rows[row_index][col_index];

            analyze_visibility("↑", &mut visible_cords, row_index, col_index, &mut max_height, current_height);

            if current_height == 9 {
                break;
            }
            row_index -= 1;
        }
        col_index += 1;
    }

    let visible_generalized = visible_cords.iter().map(|(&_, r, c)| (r, c)).collect::<HashSet<_>>();

    println!("{:?}", visible_cords);
    println!("{:?}", visible_generalized);

    (width + length) * 2 - 4 + visible_generalized.len()
}

fn analyze_visibility<'a>(direction: &'a str, visible_cords: &mut HashSet<(&'a str, usize, usize)>, row_index: usize, col_index: usize, max_height: &mut i32, current_height: i32) {
    if current_height > *max_height {
        visible_cords.insert((direction, row_index, col_index));
        *max_height = current_height;
    }
}

pub fn treetop_tree_house_part_2(file_name: &str) {
    let rows = read_input(file_name);
    let length = rows.len();
    let width = rows[0].len();

    let mut rl_scenic_score = vec![vec![0; width]; length];
    let mut tb_scenic_score = vec![vec![0; width]; length];
    let mut bt_scenic_score = vec![vec![0; width]; length];
    let mut lr_scenic_score = vec![vec![0; width]; length];

    let mut row_index = 1;

    while row_index < length - 1 {
        let mut col_index = 1;

        while col_index < width - 1 {
            let current_height = rows[row_index][col_index];

            let mut dist = 1;
            while col_index as i32 - dist as i32 >= 0 {
                let previous_height = rows[row_index][col_index - dist];

                if previous_height >= current_height {
                    break;
                }
                dist += 1;
            }

            lr_scenic_score[row_index][col_index] = dist;

            col_index += 1;
        }
        row_index += 1;
    }

    // from right to left
    let mut row_index = 1;

    while row_index < length - 1 {
        let mut col_index = width - 1;

        while col_index > 0 {
            let current_height = rows[row_index][col_index];

            let mut dist = 1;
            while col_index + dist < width {
                let next_height = rows[row_index][col_index + dist];

                println!("comparing: {current_height} with {next_height}");

                if next_height >= current_height {
                    break;
                }
                dist += 1;
            }

            rl_scenic_score[row_index][col_index] = dist;

            col_index -= 1;
        }
        row_index += 1;
    }


    print(&mut lr_scenic_score);
    print(&mut rl_scenic_score);
    // println!("{:?}", rl_scenic_score);
}

fn print(rl_scenic_score: &mut [Vec<usize>]) {
    for row in rl_scenic_score.iter() {
        let row_str = row
            .iter()
            .map(|elem| format!("{:>3}", elem))
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", row_str);
    }
    println!()
}