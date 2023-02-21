use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

type Map = Vec<Vec<i8>>;
type Coords = (usize, usize);

#[derive(Debug)]
enum MoveInstruction {
    Go(usize),
    TurnLeft,
    TurnRight,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_right(&mut self) {
        *self = match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn turn_left(&mut self) {
        *self = match self {
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
        }
    }
}

fn read_input(file_name: &str) -> (Map, Vec<MoveInstruction>) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut map = Vec::new();

    // parse map
    while reader.read_line(&mut buf).is_ok() {
        if buf == "\r\n" {
            break;
        }

        let line = buf.trim_end_matches("\r\n")
            .chars()
            .map(|ch| match ch {
                ' ' => -1,
                '.' => 0,
                '#' => 1,
                _ => panic!("invalid char: {}", ch)
            })
            .collect::<Vec<_>>();

        map.push(line);
        buf = String::new();
    };

    let _ = reader.read_line(&mut buf).unwrap();

    // parse instructions
    let mut instructions = Vec::new();
    let mut number_buff = Vec::new();
    for ch in buf.trim().chars() {
        if ch.is_ascii_digit() {
            number_buff.push(ch);
            continue;
        }

        let mut sum = 0;
        let mut magnitude = 1;
        while let Some(digit) = number_buff.pop() {
            sum += magnitude * digit.to_digit(10).unwrap() as usize;
            magnitude *= 10;
        }

        instructions.push(MoveInstruction::Go(sum));

        match ch {
            'L' => instructions.push(MoveInstruction::TurnLeft),
            'R' => instructions.push(MoveInstruction::TurnRight),
            _ => {}
        };
    }

    if !buf.is_empty() {
        let mut sum = 0;
        let mut magnitude = 1;
        while let Some(digit) = number_buff.pop() {
            sum += magnitude * digit.to_digit(10).unwrap() as usize;
            magnitude *= 10;
        }

        instructions.push(MoveInstruction::Go(sum));
    }

    // pad rows to have the same length for convenience
    let max_length = map.iter().map(|row| row.len()).max().unwrap();
    for row in map.iter_mut() {
        row.append(&mut iter::repeat(-1).take(max_length - row.len()).collect());
    }

    (map, instructions)
}

fn print_map(map: &Map) {
    for row in map.iter() {
        for tile in row.iter() {
            let sign = match tile {
                -1 => " ",
                0 => ".",
                1 => "#",
                _ => panic!()
            };
            print!("{sign}");
        }
        println!();
    }
}

fn extract_rows_and_cols_info(map: &Map) -> (
    HashMap<usize, (i32, i32)>,
    HashMap<usize, (i32, i32)>,
    HashMap<usize, Vec<i32>>,
    HashMap<usize, Vec<i32>>)
{
    let mut rows = HashMap::new();
    let mut cols = HashMap::new();
    let mut rows_obstacles: HashMap<usize, Vec<i32>> = HashMap::new();
    let mut cols_obstacles: HashMap<usize, Vec<i32>> = HashMap::new();

    // rows
    for i in 0..map.len() {
        let mut obstacles_indexes: Vec<i32> = Vec::new();
        let road_start = map[i].iter().position(|x| *x != -1).unwrap();

        let mut j = road_start;
        while j < map[i].len() && map[i][j] != -1 {
            if map[i][j] == 1 {
                obstacles_indexes.push(j as i32);
            }
            j += 1;
        }
        let road_end = j - 1;

        if !obstacles_indexes.is_empty() {
            let road_len = (road_end - road_start) as i32 + 1;
            let (first, last) = (obstacles_indexes[0], obstacles_indexes[obstacles_indexes.len() - 1]);
            obstacles_indexes.insert(0, last - road_len);
            obstacles_indexes.insert(obstacles_indexes.len(), first + road_len);
        }

        rows.insert(i, (road_start as i32, road_end as i32));
        rows_obstacles.insert(i, obstacles_indexes);
    }

    // cols
    for j in 0..map[0].len() {
        let mut obstacles_indexes = Vec::new();

        let mut i = 0;
        while map[i][j] == -1 { i += 1; }

        let road_start = i;
        while i < map.len() && map[i][j] != -1 {
            if map[i][j] == 1 {
                obstacles_indexes.push(i as i32);
            }
            i += 1;
        }
        let road_end = i - 1;

        if !obstacles_indexes.is_empty() {
            let road_len = (road_end - road_start) as i32 + 1;
            let (first, last) = (obstacles_indexes[0], obstacles_indexes[obstacles_indexes.len() - 1]);
            obstacles_indexes.insert(0, last - road_len);
            obstacles_indexes.insert(obstacles_indexes.len(), first + road_len);
        }

        cols.insert(j, (road_start as i32, road_end as i32));
        cols_obstacles.insert(j, obstacles_indexes);
    }
    (rows, cols, rows_obstacles, cols_obstacles)
}

pub fn monkey_map_part_1(file_name: &str) -> i32 {
    let (map, instructions) = read_input(file_name);

    print_map(&map);
    println!("{:?}", instructions);

    let (rows, cols, rows_obstacles, cols_obstacles) = extract_rows_and_cols_info(&map);
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);

    let mut row = 0_i32;
    let mut col = rows[&0].0;
    let mut dir = Direction::Right;

    for instruction in instructions.into_iter() {
        match instruction {
            MoveInstruction::TurnLeft => dir.turn_left(),
            MoveInstruction::TurnRight => dir.turn_right(),
            MoveInstruction::Go(amount) => {
                println!("\npos: {:?}", (row, col, dir));

                let ((start, end), ob, pos) = match dir {
                    Direction::Right | Direction::Left => (&rows[&(row as usize)], &rows_obstacles[&(row as usize)], &mut col),
                    Direction::Up | Direction::Down => (&cols[&(col as usize)], &cols_obstacles[&(col as usize)], &mut row),
                };
                println!("{ob:?}");

                let idx = ob.partition_point(|&x| x < *pos);
                let (prev_ob, next_ob) = if ob.is_empty() {
                    (i32::MAX, i32::MAX)
                } else {
                    (ob[idx - 1], ob[idx])
                };
                println!("closest obstacles: {:?}", (prev_ob, next_ob));

                let max_forward = next_ob - *pos - 1;
                let max_backward = (prev_ob - *pos + 1).abs();
                println!("move amount: {:3}\tmax forward: {} max backward: {}", amount, max_forward, max_backward);

                match dir {
                    Direction::Left | Direction::Up => *pos -= max_backward.min(amount as i32),
                    Direction::Right | Direction::Down => *pos += max_forward.min(amount as i32),
                };

                let len = end - start + 1;
                println!("edges: {:?}, len: {len}", (start, end));

                if *pos < *start {
                    while *pos < *start { *pos += len; };
                } else if *pos > *end {
                    while *pos > *end { *pos -= len; };
                };
            }
        }
    }

    1000 * (row + 1) + 4 * (col + 1) + match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}
