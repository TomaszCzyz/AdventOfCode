#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;

fn read_input(file_name: &str) -> Vec<Direction> {
    fs::read_to_string(file_name)
        .unwrap()
        .chars()
        .map(|ch| if ch == '>' { Direction::Right } else { Direction::Left })
        .collect::<Vec<_>>()
}

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    col: u32,
    row: u32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Rock {
    Dash,
    Plus,
    Knee,
    Pipe,
    Square,
}

/// The origin of each rock is located in left-down corner of its 'box'
/// In the case of Rock::Plus it is outside of the shape.
/// X###
///
/// .#.
/// ###
/// X#.
///
/// ..#
/// ..#
/// X##
///
/// #
/// #
/// #
/// X
///
/// ##
/// X#
impl Rock {
    fn get_collision_points_from_below(self, pos: Point) -> Vec<Point> {
        match self {
            Rock::Dash => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row },
                    Point { col: pos.col + 3, row: pos.row },
                ])
            }
            Rock::Plus => {
                Vec::from([
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row + 1 },
                ])
            }
            Rock::Knee => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row },
                ])
            }
            Rock::Pipe => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                ])
            }
            Rock::Square => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row },
                ])
            }
        }
    }

    fn get_collision_points_from_right(self, pos: Point) -> Vec<Point> {
        match self {
            Rock::Dash => {
                Vec::from([
                    Point { col: pos.col + 3, row: pos.row },
                ])
            }
            Rock::Plus => {
                Vec::from([
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row + 1 },
                    Point { col: pos.col + 1, row: pos.row + 2 },
                ])
            }
            Rock::Knee => {
                Vec::from([
                    Point { col: pos.col + 2, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row + 1 },
                    Point { col: pos.col + 2, row: pos.row + 2 },
                ])
            }
            Rock::Pipe => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col, row: pos.row + 2 },
                    Point { col: pos.col, row: pos.row + 3 },
                ])
            }
            Rock::Square => {
                Vec::from([
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row + 1 },
                ])
            }
        }
    }

    fn get_collision_points_from_left(self, pos: Point) -> Vec<Point> {
        match self {
            Rock::Dash => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                ])
            }
            Rock::Plus => {
                Vec::from([
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row + 2 },
                ])
            }
            Rock::Knee => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row + 1 },
                    Point { col: pos.col + 2, row: pos.row + 2 },
                ])
            }
            Rock::Pipe => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col, row: pos.row + 2 },
                    Point { col: pos.col, row: pos.row + 3 },
                ])
            }
            Rock::Square => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col, row: pos.row + 1 },
                ])
            }
        }
    }

    fn get_structural_points(self, pos: Point) -> Vec<Point> {
        match self {
            Rock::Dash => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row },
                    Point { col: pos.col + 3, row: pos.row },
                ])
            }
            Rock::Plus => {
                Vec::from([
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row + 1 },
                    Point { col: pos.col + 1, row: pos.row + 2 },
                    Point { col: pos.col + 2, row: pos.row + 1 },
                ])
            }
            Rock::Knee => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row },
                    Point { col: pos.col + 2, row: pos.row + 1 },
                    Point { col: pos.col + 2, row: pos.row + 2 },
                ])
            }
            Rock::Pipe => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col, row: pos.row + 2 },
                    Point { col: pos.col, row: pos.row + 3 },
                ])
            }
            Rock::Square => {
                Vec::from([
                    Point { col: pos.col, row: pos.row },
                    Point { col: pos.col, row: pos.row + 1 },
                    Point { col: pos.col + 1, row: pos.row },
                    Point { col: pos.col + 1, row: pos.row + 1 },
                ])
            }
        }
    }

    fn get_edge_pos(self) -> (u32, u32) {
        match self {
            Rock::Dash => (0, 3),
            Rock::Plus => (0, 4),
            Rock::Knee => (0, 4),
            Rock::Pipe => (0, 6),
            Rock::Square => (0, 5),
        }
    }
}

#[derive(Debug)]
struct RocksLoopedIterator {
    rocks: [Rock; 5],
    current_index: usize,
    total_index: usize,
}

impl Default for RocksLoopedIterator {
    fn default() -> Self {
        Self {
            rocks: [Rock::Dash, Rock::Plus, Rock::Knee, Rock::Pipe, Rock::Square],
            current_index: 0,
            total_index: 0,
        }
    }
}

impl Iterator for RocksLoopedIterator {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let rock = self.rocks[self.current_index];

        if self.current_index == self.rocks.len() - 1 {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        self.total_index += 1;
        Some(rock)
    }
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct FallingRock {
    rock: Rock,
    pos: Point,
    falling_time: usize,
}

fn move_rock_down(rock: &mut FallingRock) {
    rock.pos.row -= 1;
    rock.falling_time += 1;

    // println!("move down to position: {:?}", rock.pos);
}

fn is_collision(columns: &HashMap<u32, HashSet<u32>>, falling_rock: &mut FallingRock) -> bool {
    for possible_collision_point in falling_rock.rock.get_collision_points_from_below(falling_rock.pos).iter() {
        if possible_collision_point.row == 0 {
            return true;
        }

        if columns[&possible_collision_point.col].contains(&(possible_collision_point.row - 1)) {
            return true;
        }
    }

    false
}

fn push_rock(direction: &Direction, columns: &HashMap<u32, HashSet<u32>>, falling_rock: &mut FallingRock) {
    let (left_edge_pos, right_edge_pos) = falling_rock.rock.get_edge_pos();

    match direction {
        Direction::Left => {
            if falling_rock.pos.col == left_edge_pos {
                return;
            }

            let possible_collision_points = falling_rock.rock.get_collision_points_from_left(falling_rock.pos);
            for p in possible_collision_points.iter() {
                if columns[&(p.col - 1)].contains(&p.row) {
                    return;
                }
            }

            falling_rock.pos.col -= 1;
            // println!("push rock to the left to position: {:?}", falling_rock.pos);
        }
        Direction::Right => {
            if falling_rock.pos.col == right_edge_pos {
                return;
            }

            let possible_collision_points = falling_rock.rock.get_collision_points_from_right(falling_rock.pos);
            for p in possible_collision_points.iter() {
                if columns[&(p.col + 1)].contains(&p.row) {
                    return;
                }
            }

            falling_rock.pos.col += 1;
            // println!("push rock to the right to position: {:?}", falling_rock.pos);
        }
    }
}


#[derive(Debug)]
struct HistoryInfo {
    height: usize,
    rocks_count: usize,
}

pub fn pyroclastic_flow(file_name: &str, rocks_number: usize) -> usize {
    let input = read_input(file_name);

    let mut columns: HashMap<u32, HashSet<u32>> = HashMap::new();

    for x in 0..=6 {
        columns.insert(x, HashSet::from([0]));
    }

    // simulation variables
    let mut rocks_iter = RocksLoopedIterator::default();
    let mut direction_counter: usize = 0;
    let mut rocks_left = rocks_number;
    let mut falling_rock_op: Option<FallingRock> = None;

    // repetition detection variables
    let mut repetition_detected = false;
    let mut iteration_history: Vec<HistoryInfo> = vec![HistoryInfo { height: 0, rocks_count: 0 }];
    let mut tower_height_to_add = 0_usize;

    loop {
        // looking for pattern repetition
        if !repetition_detected && direction_counter > 0 && direction_counter % input.len() == 0 {
            let new_height = *columns.values()
                .map(|set| set.iter().max().unwrap())
                .max()
                .unwrap();

            iteration_history.push(HistoryInfo {
                height: new_height as usize,
                rocks_count: rocks_iter.total_index,
            });

            if let Some((height_in_pattern, rocks_in_pattern)) = analyze_iteration_history(&iteration_history) {
                repetition_detected = true;
                let pattern_repetition_numer = rocks_left / rocks_in_pattern;

                tower_height_to_add = pattern_repetition_numer * height_in_pattern;
                rocks_left -= pattern_repetition_numer * rocks_in_pattern - 1;
            }
        }

        // spawning new rock
        if falling_rock_op.is_none() {
            let max_row = *columns.values()
                .map(|set| set.iter().max().unwrap())
                .max()
                .unwrap() + 4;

            let spawn_pos = Point {
                col: 2,
                row: max_row,
            };

            let new_falling_rock = FallingRock {
                rock: rocks_iter.next().unwrap(),
                falling_time: 0,
                pos: spawn_pos,
            };
            // println!("spawned {} - {:?}", rocks_counter, new_falling_rock);

            falling_rock_op = Some(new_falling_rock);
            rocks_left -= 1;
        }

        let falling_rock = falling_rock_op.as_mut().unwrap();
        let direction = &input[direction_counter % input.len()];

        push_rock(direction, &columns, falling_rock);

        let mut collision_occurred = false;
        if falling_rock.falling_time >= 3 && is_collision(&columns, falling_rock) {
            collision_occurred = true;
            // println!("collision detected! rock's position: {:?}", falling_rock.pos);

            for structural_pos in falling_rock.rock.get_structural_points(falling_rock.pos) {
                columns.get_mut(&structural_pos.col).map(|val| val.insert(structural_pos.row));
            }
        }

        if collision_occurred {
            falling_rock_op = None;
        } else {
            move_rock_down(falling_rock);
        }

        if rocks_left == 0 {
            break;
        }

        direction_counter += 1;
    }

    *columns.values()
        .map(|set| set.iter().max().unwrap())
        .max()
        .unwrap() as usize + tower_height_to_add
}

fn print_tower(columns: &mut HashMap<u32, HashSet<u32>>, height: usize) {
    for i in (0..height as u32).rev() {
        print!("{}\t", i);
        for col in 0..7 {
            if columns[&col].contains(&i) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn analyze_iteration_history(iteration_history: &[HistoryInfo]) -> Option<(usize, usize)> {
    let diff_vec = iteration_history.windows(2)
        .map(|infos| {
            let first_info = &infos[0];
            let second_info = &infos[1];
            let height_diff = second_info.height - first_info.height;
            let rocks_count_diff = second_info.rocks_count - first_info.rocks_count;

            (height_diff, rocks_count_diff)
        })
        .collect::<Vec<_>>();

    // println!("diff_vec: {:#?}", diff_vec);

    let last_entry = diff_vec.last().unwrap();
    let last_entry_pos = diff_vec.len() - 1;

    return match diff_vec.iter().rev().skip(1).position(|x| x == last_entry) {
        None => None,
        Some(equal_entry_pos_rev) => {
            let equal_entry_pos = diff_vec.len() - 1 - (equal_entry_pos_rev + 1);

            // there must be at least 2x elements (double the pattern length)
            let offset = last_entry_pos - equal_entry_pos;
            if diff_vec.len() < 2 * offset {
                return None;
            }

            for i in 1..offset {
                // println!("last_entry_pos: {} equal_entry_pos {}", last_entry_pos, equal_entry_pos);
                // println!("comparing {:?} with {:?}", diff_vec[last_entry_pos - i], diff_vec[equal_entry_pos - i]);
                if diff_vec[last_entry_pos - i] != diff_vec[equal_entry_pos - i] {
                    return None;
                }
            }

            let height_of_pattern = diff_vec.iter().rev().take(offset).map(|(height, _)| *height).sum();
            let rocks_in_pattern = diff_vec.iter().rev().take(offset).map(|(_, rocks)| *rocks).sum();

            println!("repetition detected - height_of_pattern: {height_of_pattern} and rocks_in_pattern: {rocks_in_pattern}");
            Some((height_of_pattern, rocks_in_pattern))
        }
    };
}

fn trim_columns(columns: &mut HashMap<u32, HashSet<u32>>) {
    let max_row = *columns.values()
        .map(|set| set.iter().max().unwrap())
        .max()
        .unwrap() + 4;

    for column in columns.values_mut() {
        for row in column.clone() {
            if row < max_row - 30 {
                column.remove(&row);
            }
        }
    }
}
