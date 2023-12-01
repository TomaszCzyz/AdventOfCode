use std::borrow::BorrowMut;
use std::f32::consts::SQRT_2;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

pub fn read_input(file_name: &str) -> InstructionsIterator {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    InstructionsIterator {
        buf_reader: reader,
    }
}

pub struct InstructionsIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for InstructionsIterator {
    type Item = (Direction, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf_reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                let (direction, distance) = buf.trim_end().split_at(1);


                Some((parse_direction(direction), parse_distance(distance)))
            }
            Err(_e) => panic!(),
        }
    }
}

fn parse_distance(distance: &str) -> usize {
    distance.trim().parse::<usize>().ok().unwrap()
}

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        &_ => panic!("incorrect direction")
    }
}


fn expand_map(map: &mut Vec<Vec<char>>, direction: Direction) {
    match direction {
        Direction::Left => {
            for row in map {
                row.insert(0, '.');
            }
        }
        Direction::Right => {
            for row in map {
                row.push('.');
            }
        }
        Direction::Up => {
            let row_length = map[0].len();
            map.insert(0, vec!['.'; row_length])
        }
        Direction::Down => {
            let row_length = map[0].len();
            map.push(vec!['.'; row_length])
        }
    };
}

fn extend_maps_if_needed(
    head_map: &mut Vec<Vec<char>>,
    tail_map: &mut Vec<Vec<char>>,
    knots: &mut [Knot],
) {
    if knots[0].row < 0 {
        expand_map(head_map, Direction::Up);
        expand_map(tail_map, Direction::Up);
        for knot in knots.iter_mut() {
            knot.row += 1
        }
    }

    if knots[0].col < 0 {
        expand_map(head_map, Direction::Left);
        expand_map(tail_map, Direction::Left);
        for knot in knots.iter_mut() {
            knot.col += 1
        }
    }

    if knots[0].row >= head_map.len() as i32 {
        expand_map(head_map, Direction::Down);
        expand_map(tail_map, Direction::Down);
    }

    if knots[0].col >= head_map[0].len() as i32 {
        expand_map(head_map, Direction::Right);
        expand_map(tail_map, Direction::Right);
    }
}

#[derive(Clone, Copy)]
struct Knot {
    row: i32,
    col: i32,
}

fn update_tail_cords_new(knot1: Knot, knot2: &mut Knot) {
    let square_dist = (knot2.row - knot1.row).pow(2) + (knot2.col - knot1.col).pow(2);
    let dist = (square_dist as f32).powf(1. / 2.);

    if dist <= SQRT_2 + 0.001 {
        return;
    }

    let vector = (knot2.row - knot1.row, knot2.col - knot1.col);
    let new_pos: (i32, i32) = match vector {
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (-2, 0) => (-1, 0),
        (2, 0) => (1, 0),

        (2, 2) => (1, 1),
        (2, -2) => (1, -1),
        (-2, 2) => (-1, 1),
        (-2, -2) => (-1, -1),

        (2, 1) => (1, 0),
        (2, -1) => (1, 0),
        (-2, 1) => (-1, 0),
        (-2, -1) => (-1, 0),

        (1, 2) => (0, 1),
        (1, -2) => (0, -1),
        (-1, 2) => (0, 1),
        (-1, -2) => (0, -1),
        _ => panic!()
    };

    knot2.row = knot1.row + new_pos.0;
    knot2.col = knot1.col + new_pos.1;
}

fn count_marked(map: &[Vec<char>]) -> i32 {
    map.iter()
        .map(|row| row.iter()
            .filter(|&char| *char == '#')
            .map(|_| 1)
            .sum::<i32>())
        .sum()
}

pub fn rope_bridge_part_1(file_name: &str) -> i32 {
    rope_bridge_part_2(file_name, 2)
}

pub fn rope_bridge_part_2(file_name: &str, knots_num: usize) -> i32 {
    let initial_size = 2;
    let half = (initial_size / 2) as i32;

    let mut head_map = vec![vec!['.'; initial_size]; initial_size];
    let mut tail_map = vec![vec!['.'; initial_size]; initial_size];

    let mut knots = vec![Knot { row: half, col: half }; knots_num];

    for (direction, dist) in read_input(file_name) {
        for _ in 0..dist {
            match direction {
                Direction::Left => knots[0].row -= 1,
                Direction::Right => knots[0].row += 1,
                Direction::Up => knots[0].col += 1,
                Direction::Down => knots[0].col -= 1,
            };

            extend_maps_if_needed(&mut head_map, &mut tail_map, &mut knots);

            for i in 0..knots.len() - 1 {
                let first = knots[i];
                let second = knots[i + 1].borrow_mut();
                update_tail_cords_new(first, second);
            }

            head_map[knots[0].row as usize][knots[0].col as usize] = '$';
            tail_map[knots.last().unwrap().row as usize][knots.last().unwrap().col as usize] = '#';

            // print_knots(&knots, head_map.len(), head_map[0].len());
        }
    }

    print(&head_map);
    print(&tail_map);

    count_marked(&tail_map)
}

#[allow(dead_code)]
fn print_knots(knots: &[Knot], length: usize, width: usize) {
    let mut array = vec![vec!['.'; width]; length];

    for (i, knot) in knots.iter().enumerate() {
        array[knot.row as usize][knot.col as usize] = format!("{i}").parse().unwrap();
    }

    print(&array);
}

#[allow(dead_code)]
fn print(array2d: &[Vec<char>]) {
    for row in array2d.iter() {
        let row_str = row
            .iter()
            .map(|elem| elem.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", row_str);
    }
    println!()

    // for j in (0..array2d.len() - 1).rev() {
    //     for i in 0..array2d[0].len() {
    //         print!("{}", array2d[i][j]);
    //     }
    //     println!();
    // }
    // println!()
}