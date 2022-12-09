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

pub fn rope_bridge_part_1(file_name: &str) -> i32 {
    let initial_size = 2;
    let half = (initial_size / 2) as i32;

    let mut head_map = vec![vec!['.'; initial_size]; initial_size];
    let mut tail_map = vec![vec!['.'; initial_size]; initial_size];

    let (mut head_row, mut head_col): (i32, i32) = (half, half);
    let (mut tail_row, mut tail_col): (i32, i32) = (half, half);

    head_map[head_row as usize][head_col as usize] = '#';

    for (direction, dist) in read_input(file_name) {
        for _ in 0..dist {
            match direction {
                Direction::Left => head_row -= 1,
                Direction::Right => head_row += 1,
                Direction::Up => head_col += 1,
                Direction::Down => head_col -= 1,
            };

            if head_row < 0 {
                expand_map(&mut head_map, Direction::Up);
                expand_map(&mut tail_map, Direction::Up);
                head_row += 1;
                tail_row += 1;
            }

            if head_col < 0 {
                expand_map(&mut head_map, Direction::Left);
                expand_map(&mut tail_map, Direction::Left);
                head_col += 1;
                tail_col += 1;
            }

            if head_row >= head_map.len() as i32 {
                expand_map(&mut head_map, Direction::Down);
                expand_map(&mut tail_map, Direction::Down);
            }

            if head_col >= head_map[0].len() as i32 {
                expand_map(&mut head_map, Direction::Right);
                expand_map(&mut tail_map, Direction::Right);
            }


            assert!(head_row >= 0);
            assert!(head_col >= 0);

            update_tail_cords(&head_row, &head_col, &mut tail_row, &mut tail_col);

            head_map[head_row as usize][head_col as usize] = '$';
            tail_map[tail_row as usize][tail_col as usize] = '#';
        }
    }

    // print(&head_map);
    // print(&tail_map);

    let r: i32 = count_marked(&tail_map);

    r
}

fn count_marked(map: &[Vec<char>]) -> i32 {
    map.iter()
        .map(|row| row.iter()
            .filter(|&char| *char == '#')
            .map(|_| 1)
            .sum::<i32>())
        .sum()
}

fn update_tail_cords(head_row: &i32, head_col: &i32, tail_row: &mut i32, tail_col: &mut i32) {
    let square_dist = (*tail_row - *head_row).pow(2) + (*tail_col - *head_col).pow(2);
    let dist = (square_dist as f32).powf(1. / 2.);

    if dist < 1. {
        return;
    }

    // same row
    if *tail_row == *head_row {
        if *tail_col < *head_col {
            *tail_col = *head_col - 1
        } else {
            *tail_col = *head_col + 1
        }

        return;
    }

    // same column
    if *tail_col == *head_col {
        if *tail_row < *head_row {
            *tail_row = *head_row - 1
        } else {
            *tail_row = *head_row + 1
        }

        return;
    }

    if *tail_row - 2 == *head_row {
        *tail_row = *head_row + 1;
        *tail_col = *head_col;
    } else if *tail_row + 2 == *head_row {
        *tail_row = *head_row - 1;
        *tail_col = *head_col;
    } else if *tail_col - 2 == *head_col {
        *tail_row = *head_row;
        *tail_col = *head_col + 1;
    } else if *tail_col + 2 == *head_col {
        *tail_row = *head_row;
        *tail_col = *head_col - 1;
    }
}

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