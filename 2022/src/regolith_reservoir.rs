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


    let (min, max) = match map.keys().minmax() {
        MinMaxResult::MinMax(min, max) => (*min, *max),
        _ => panic!(),
    };

    // add empty edge columns for convenience
    map.insert(min - 1, HashSet::new());
    map.insert(max + 1, HashSet::new());
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

fn check_tile((tile_col, tile_row): (u32, u32), obstacles_map: &mut HashMap<u32, HashSet<u32>>, sum: &mut u32) -> bool {
    println!("checking tile: {:?}", (tile_col, tile_row));

    let next_row = tile_row + 1;

    if obstacles_map[&(tile_col)].contains(&next_row) { // check tile below
        let col_to_left = tile_col - 1;
        let col_to_right = tile_col + 1;
        let is_left_down_occupied = obstacles_map[&col_to_left].contains(&next_row);
        let is_right_down_occupied = obstacles_map[&col_to_right].contains(&next_row);

        return match (is_left_down_occupied, is_right_down_occupied) {
            (true, true) => {
                fill_and_add(tile_col, tile_row, obstacles_map, sum);
                true
            }
            (false, true) => {
                print!("going left!\t");
                let result = check_tile((col_to_left, next_row), obstacles_map, sum);
                if result {
                    fill_and_add(tile_col, tile_row, obstacles_map, sum);
                }

                result
            }
            (true, false) => {
                print!("going right!\t");
                let result = check_tile((col_to_right, next_row), obstacles_map, sum);
                if result {
                    fill_and_add(tile_col, tile_row, obstacles_map, sum);
                }

                result
            }
            (false, false) => {
                print!("going left!\t");
                let result_left = check_tile((col_to_left, next_row), obstacles_map, sum);
                if result_left {
                    print!("going right!\t");
                    let result_right = check_tile((col_to_right, next_row), obstacles_map, sum);
                    if result_right {
                        fill_and_add(tile_col, tile_row, obstacles_map, sum);
                    }

                    return result_right;
                }

                result_left
            }
        };
    } else { // no tile one below
        match obstacles_map[&tile_col].iter().find(|&rock_row| *rock_row > tile_row) {
            None => false, // column has no rocks below current tile
            Some(_) => {
                print!("going down!\t");
                if check_tile((tile_col, tile_row + 1), obstacles_map, sum) {
                    // re-check tile above
                    print!("going up!\t");
                    check_tile((tile_col, tile_row), obstacles_map, sum)
                } else {
                    false
                }
            }
        }
    }
}

fn fill_and_add(tile_col: u32, tile_row: u32, obstacles_map: &mut HashMap<u32, HashSet<u32>>, sum: &mut u32) {
    // fill current tile with sand
    println!("\tfilling tile at: {:?}", (tile_col, tile_row));
    obstacles_map.get_mut(&tile_col).map(|val| val.insert(tile_row));
    *sum += 1;
}


fn process_input_2(input: Vec<Vec<(u32, u32)>>) -> HashMap<u32, HashSet<u32>> {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in input.iter() {
        for (first_pair, second_pair) in line.iter().tuple_windows::<(_, _)>() {
            process_input_inner(*first_pair, *second_pair, &mut map)
        }
    }


    let (min, max) = match map.keys().minmax() {
        MinMaxResult::MinMax(min, max) => (*min, *max),
        _ => panic!(),
    };

    // add empty edge columns for convenience
    for i in 1..=250 {
        map.insert(min - i, HashSet::new());
        map.insert(max + i, HashSet::new());
    }

    let min_row = map.iter()
        .filter_map(|(_col, set)| set.iter().max())
        .max()
        .unwrap();

    let new_min_rock = min_row + 2;

    for (_, set) in map.iter_mut()
    {
        set.insert(new_min_rock);
    }

    map
}

pub fn regolith_reservoir_part_1(file_name: &str) -> u32 {
    let input = read_input(file_name);
    let mut rocks_map = process_input(input);

    println!("{:?}", rocks_map);

    let mut sum = 0;

    check_tile((500, 0), &mut rocks_map, &mut sum);

    sum
}

pub fn regolith_reservoir_part_2(file_name: &str) -> u32 {
    let input = read_input(file_name);
    let mut rocks_map = process_input_2(input);

    println!("{:?}", rocks_map);

    let mut sum = 0;

    check_tile((500, 0), &mut rocks_map, &mut sum);

    sum
}