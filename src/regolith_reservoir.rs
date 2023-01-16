use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::{Itertools, MinMaxResult};

fn read_input(file_name: &str) -> Vec<Vec<(u32, u32)>> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut input_data = Vec::new();

    loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            }

            let rocks_path = buf.trim()
                .split(" -> ")
                .map(|pair| pair.split(',')
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap())
                .collect::<Vec<_>>();

            input_data.push(rocks_path)
        }
    }

    input_data
}

/// col_number -> (set of number representing row with rock)
/// for example data we will have the following mappings:
/// 494 -> [9]
/// 495 -> [9]
/// 496 -> [6,9]
/// 497 -> [6,9]
/// 498 -> [4,5,6,9]
/// 499 -> [9]
/// 500 -> [9]
/// 501 -> [9]
/// 502 -> [4,5,6,7,8,9]
/// 503 -> [4]
fn process_input(input: Vec<Vec<(u32, u32)>>) -> HashMap<u32, HashSet<u32>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in input.iter() {
        for (first_pair, second_pair) in line.iter().tuple_windows::<(_, _)>() {
            process_input_inner(*first_pair, *second_pair, &mut map)
        }
    }

    // add empty edge columns for convince
    let (min, max) = match map.keys().minmax() {
        MinMaxResult::MinMax(min, max) => (*min, *max),
        _ => panic!(),
    };

    map.insert(min, HashSet::new());
    map.insert(max, HashSet::new());
    map
}

fn process_input_inner(first_pair: (u32, u32), second_pair: (u32, u32), data: &mut HashMap<u32, HashSet<u32>>) {
    if first_pair.0 == second_pair.0 { // vertical path
        let col_number = first_pair.0;
        let (min, max) = sort(first_pair.1, second_pair.1);

        for row in min..=max {
            data.entry(col_number)
                .and_modify(|set| { set.insert(row); })
                .or_insert(HashSet::from([row]));
        }
    } else { // horizontal path
        let row_number = first_pair.1;
        let (min, max) = sort(first_pair.0, second_pair.0);

        for col in min..=max {
            data.entry(col)
                .and_modify(|set| { set.insert(row_number); })
                .or_insert(HashSet::from([row_number]));
        }
    }
}

fn sort(first: u32, second: u32) -> (u32, u32) {
    if first < second {
        (first, second)
    } else {
        (second, first)
    }
}

fn analyze_cone_below((start_col, start_row): (u32, u32), rocks_map: &mut HashMap<u32, HashSet<u32>>) {
    println!("analyzing cone below {:?}", (start_col, start_row));

    let floor_row = match rocks_map[&start_col].iter().find(|&rock_row| *rock_row > start_row) {
        None => todo!("column has no rocks lower then start_row"),
        Some(row) => *row
    };

    let mut sand_row = floor_row - 1;

    loop {
        for direction in [-1, 1] {
            let mut row = sand_row;
            let mut col = start_col;
            let mut found_obstacle: Option<(u32, u32)> = Option::default();

            while row < floor_row {
                row += 1;
                move_in(&mut col, direction);

                // println!("im here: {:?}", (col, row));

                // check tile below next (left-down-down)
                if rocks_map[&col].contains(&row) {
                    found_obstacle = Some((col, row));
                    break;
                }

                // there is obstacle in the left-down tile
                if !rocks_map[&col].contains(&(row + 1)) {
                    analyze_cone_below((col, row), rocks_map);
                }
            }

            if let Some((obstacle_col, obstacle_row)) = found_obstacle {
                let columns: Vec<u32> = if direction == -1 {
                    (obstacle_col..start_col).rev().collect()
                } else {
                    (start_col..obstacle_col).collect()
                };

                for (c, r) in columns.into_iter().zip(sand_row..=obstacle_row) {
                    println!("sand can at {:?}", (c, r));
                    rocks_map.get_mut(&c).map(|val| val.insert(r));
                }
                println!("\t\t rock at {:?}! \t\t {:?}", (obstacle_col, obstacle_row), rocks_map);
            } else {
                // no obstacle found check if there is a floor in the next tile
                if !rocks_map[&col].contains(&row) {
                    analyze_cone_below((col, row), rocks_map);
                }
                // println!("sand can be from {:?} to {:?}", (start_col, sand_row), (move_in(&mut col, -direction), floor_row - 1))
            }
        }

        if sand_row == 0 {
            break;
        }
        sand_row -= 1; // go 1 up
    }
}

fn move_in(col: &mut u32, direction: i32) {
    if direction == -1 {
        *col -= 1;
    } else {
        *col += 1;
    }
}

pub fn regolith_reservoir_part_1(file_name: &str) -> u32 {
    let input = read_input(file_name);
    let mut rocks_map = process_input(input);

    println!("{:?}", rocks_map);

    analyze_cone_below((500, 0), &mut rocks_map);


    0
}
