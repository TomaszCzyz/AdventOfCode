use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn read_input(file_name: &str) -> Vec<Vec<u8>> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|result| result.unwrap())
        .map(|line| line.chars()
            .map(|char| if char == 'S' { 0 } else if char == 'E' { 27 } else { char as u8 - 96 })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn find_start_and_end(map: &[Vec<u8>]) -> (Position, Position) {
    let mut start_pos = Position { row: 0, col: 0 };
    let mut end_pos = Position { row: 0, col: 0 };
    for (i, row) in map.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == 0 {
                start_pos = Position { row: i, col: j }
            } else if *elem == 27 {
                end_pos = Position { row: i, col: j }
            }
        }
    }

    (start_pos, end_pos)
}

#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Position {
    fn up(&self) -> Self {
        Self {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Self {
        Self {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn left(&self) -> Self {
        Self {
            row: self.row,
            col: self.col - 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            row: self.row,
            col: self.col + 1,
        }
    }
}

#[derive(Clone, Debug)]
struct MapState<'a> {
    map: &'a Vec<Vec<u8>>,
    pos: Position,
    path: Vec<Position>,
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl MapState<'_> {
    fn can_go(&self, direction: Direction) -> bool {
        let new_pos = match direction {
            Direction::Up => {
                if self.pos.row == 0 {
                    return false;
                }
                self.pos.up()
            }
            Direction::Down => {
                if self.pos.row == self.map.len() - 1 {
                    return false;
                }
                self.pos.down()
            }
            Direction::Left => {
                if self.pos.col == 0 {
                    return false;
                }
                self.pos.left()
            }
            Direction::Right => {
                if self.pos.col == self.map[0].len() - 1 {
                    return false;
                }
                self.pos.right()
            }
        };

        let height_diff = self.map[self.pos.row][self.pos.col] as i32 - self.map[new_pos.row][new_pos.col] as i32;

        height_diff >= -1 && !self.path.contains(&new_pos)
    }

    fn go(&self, direction: Direction) -> Self {
        let new_pos = match direction {
            Direction::Up => Position { row: self.pos.row - 1, col: self.pos.col },
            Direction::Down => Position { row: self.pos.row + 1, col: self.pos.col },
            Direction::Left => Position { row: self.pos.row, col: self.pos.col - 1 },
            Direction::Right => Position { row: self.pos.row, col: self.pos.col + 1 },
        };
        let mut new_path = self.path.clone();

        new_path.push(new_pos);

        Self {
            map: self.map,
            pos: new_pos,
            path: new_path,
        }
    }
}

fn find_paths<'a>(
    map_info: MapState<'a>,
    reached_fields: &mut HashMap<Position, usize>,
    end: Position,
) -> Option<MapState<'a>> {
    let mut result = Option::default();
    if map_info.pos == end {
        result = Some(map_info.clone());
        // print_map_with_path(map_info.map, &map_info.path);
    }

    let current_dist = map_info.path.len();

    match reached_fields.get(&map_info.pos) {
        None => {
            reached_fields.insert(map_info.pos, current_dist);
        }
        Some(prev_distance) => {
            if *prev_distance > current_dist {
                reached_fields.insert(map_info.pos, current_dist);
            } else {
                // this field was reached with shorter distance
                return None;
            }
        }
    }

    for direction in Direction::iter() {
        if !map_info.can_go(direction) {
            continue;
        }

        let new_map_info = map_info.go(direction);

        let inner_state = find_paths(new_map_info, reached_fields, end);

        let res_2 = result.clone();
        if let Some(state) = inner_state {
            if let Some(ref current) = res_2 { // why without ref i got stackoverflow?
                if state.path.len() < current.path.len() {
                    result = Some(state);
                }
            } else {
                result = Some(state.clone())
            }
        }
    }

    result
}

pub fn hill_climbing_algorithm_part_1(file_name: &str) -> usize {
    let map = read_input(file_name);

    let (start, end) = find_start_and_end(&map);
    let map_info = MapState { map: &map, pos: start, path: vec![] };
    let mut reached_fields = HashMap::new();

    let map_state = find_paths(map_info, &mut reached_fields, end).unwrap();

    print_map_with_path(map_state.map, &map_state.path);

    *(reached_fields.get(&end).unwrap())
}

fn find_start_positions(map: &[Vec<u8>]) -> Vec<Position> {
    let mut result = Vec::new();

    for (i, row) in map.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            if *elem == 1 {
                result.push(Position { row: i, col: j })
            }
        }
    }
    result
}

fn check(paths: &[Vec<Position>], pos: &Position) -> i32 {
    for path in paths {
        for (i, position) in path.iter().enumerate() {
            if position == pos {
                return (path.len() - 1 - i) as i32;
            }
        }
    }
    -1
}

pub fn hill_climbing_algorithm_part_2(file_name: &str) -> usize {
    let map = read_input(file_name);

    let starts: Vec<Position> = find_start_positions(&map);
    let (_, end) = find_start_and_end(&map);
    let mut results = Vec::new();
    let mut shortest_paths = Vec::new();

    println!("there are {} starts positions", starts.len());
    for (progress, start) in starts.iter().enumerate() {
        let map_info = MapState { map: &map, pos: *start, path: vec![] };
        let mut reached_fields = HashMap::new();

        // check if there is path cached

        let dist = check(&shortest_paths, start);
        if dist != -1 {
            println!("skipped calculation!!!");
            println!("shortest path starting from {start:?} has length: {dist:?}");
            results.push(dist as usize);

            continue;
        }

        let map_state = find_paths(map_info, &mut reached_fields, end);

        match map_state {
            None => {}
            Some(state) => {
                shortest_paths.push(state.path.clone());
            }
        }

        print!("{progress:4}/{}\t", starts.len());
        match reached_fields.get(&end) {
            None => {
                println!("no result for position {start:?}");
            }
            Some(&shortest) => {
                println!("shortest path starting from {start:?} has length: {shortest:?}");
                results.push(shortest);
            }
        };
    }

    *(results.iter().min().unwrap())
}

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<u8>>) {
    for row in map {
        for elem in row {
            print!("{elem:4}");
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_reached_fields(map: &[Vec<u8>], reached: &HashMap<Position, usize>) {
    for (i, row) in map.iter().enumerate() {
        for (j, _elem) in row.iter().enumerate() {
            let position = Position { row: i, col: j };

            match reached.get(&position) {
                None => print!("   ."),
                Some(dist) => print!("{:4}", dist)
            }
        }
        println!();
    }
}

#[allow(dead_code)]
fn print_map_with_path(map: &[Vec<u8>], path: &[Position]) {
    for (i, row) in map.iter().enumerate() {
        for (j, _elem) in row.iter().enumerate() {
            let position = Position { row: i, col: j };
            if !path.contains(&position) {
                print!("  .");
            } else {
                let result = path.iter().position(|&r| r == position).unwrap();
                print!("{:3}", result);
            }
        }
        println!();
    }
}