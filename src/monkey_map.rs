use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::from_boxed_utf8_unchecked;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Map = Vec<Vec<i8>>;

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

/// row_no/col_no -> ((road_start_index, road_end_index), [indexes of obstacles])
fn extract_rows_and_cols_info(map: &Map) -> (HashMap<usize, ((i32, i32), Vec<i32>)>, HashMap<usize, ((i32, i32), Vec<i32>)>) {
    let mut rows: HashMap<usize, ((i32, i32), Vec<i32>)> = HashMap::new();
    let mut cols: HashMap<usize, ((i32, i32), Vec<i32>)> = HashMap::new();

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

        add_wrapped_obstacles_indexes(&mut obstacles_indexes, road_start, road_end);

        rows.insert(i, ((road_start as i32, road_end as i32), obstacles_indexes));
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

        add_wrapped_obstacles_indexes(&mut obstacles_indexes, road_start, road_end);

        cols.insert(j, ((road_start as i32, road_end as i32), obstacles_indexes));
    }

    (rows, cols)
}

fn add_wrapped_obstacles_indexes(obstacles_indexes: &mut Vec<i32>, road_start: usize, road_end: usize) {
    if !obstacles_indexes.is_empty() {
        let road_len = (road_end - road_start) as i32 + 1;
        let (first, last) = (obstacles_indexes[0], obstacles_indexes[obstacles_indexes.len() - 1]);
        obstacles_indexes.insert(0, last - road_len);
        obstacles_indexes.insert(obstacles_indexes.len(), first + road_len);
    } else {
        obstacles_indexes.push(i32::MIN / 2);
        obstacles_indexes.push(i32::MAX / 2);
    }
}

pub fn monkey_map_part_1(file_name: &str) -> i32 {
    let (map, instructions) = read_input(file_name);

    print_map(&map);
    println!("{:?}", instructions);

    let (rows, cols) = extract_rows_and_cols_info(&map);
    println!("rows: {:?}", rows);
    println!("cols: {:?}", cols);

    let mut row = 0_i32;
    let mut col = rows[&0].0.0;
    let mut dir = Direction::Right;

    for instruction in instructions.into_iter() {
        match instruction {
            MoveInstruction::TurnLeft => dir.turn_left(),
            MoveInstruction::TurnRight => dir.turn_right(),
            MoveInstruction::Go(amount) => {
                println!("\npos: {:?}", (row, col, dir));

                let dir_factor = match dir {
                    Direction::Left | Direction::Up => -1,
                    Direction::Right | Direction::Down => 1,
                };
                let (info, pos) = match dir {
                    Direction::Right | Direction::Left => (&rows[&(row as usize)], &mut col),
                    Direction::Up | Direction::Down => (&cols[&(col as usize)], &mut row),
                };
                let ((start, end), obstacles) = info;

                println!("{obstacles:?}");

                let idx = obstacles.partition_point(|&x| x < *pos);
                let next_obstacle = if dir_factor == 1 { obstacles[idx] } else { obstacles[idx - 1] };
                println!("closest obstacles: {:?}", next_obstacle);

                let max_dist = (next_obstacle - *pos - dir_factor).abs();
                println!("move amount: {:3}\tmax dist: {}", amount, max_dist);

                *pos += dir_factor * max_dist.min(amount as i32);

                let len = end - start + 1;
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

// Part 2

// Input net:
//   ■ ■   |   6 5
//   ■     |   3
// ■ ■     | 2 1
// ■       | 4
//
//   x ->
// y 0,0 0,1
// | 0,1 1,1
// v

// type CoordsMap = Vec<Vec<(usize, usize)>>;
type DiceSideInfo = ((usize, usize), Rotation);

#[derive(Debug, Eq, EnumIter, Ord, PartialOrd, PartialEq, Hash)]
enum DiceSide {
    Side1,
    Side2,
    Side3,
    Side4,
    Side5,
    Side6,
}

#[derive(Debug, Eq, EnumIter, Ord, PartialOrd, PartialEq, Hash)]
enum Rotation {
    Rot000,
    Rot090,
    Rot180,
    Rot270,
}

struct InputInfo {
    // length of an edge of a dice's side
    side_length: usize,
    // cube's net of my input; maps side of a dice from an input net to coords (x,y) of upper-left corner
    // and its rotation relative to original rotation in dice
    dice_sides_info: HashMap<DiceSide, DiceSideInfo>,
    // maps side to its sides that are to the right and to the bottom ("orbits")
    side_complements: HashMap<DiceSide, ([DiceSide; 3], [DiceSide; 3])>,
}

impl InputInfo {
    const COMPLEMENTS: [(DiceSide, ([DiceSide; 3], [DiceSide; 3])); 6] = [
        (DiceSide::Side1, ([DiceSide::Side5, DiceSide::Side6, DiceSide::Side2], [DiceSide::Side4, DiceSide::Side6, DiceSide::Side3])),
        (DiceSide::Side2, ([DiceSide::Side3, DiceSide::Side5, DiceSide::Side4], [DiceSide::Side1, DiceSide::Side5, DiceSide::Side6])),
        (DiceSide::Side3, ([DiceSide::Side5, DiceSide::Side4, DiceSide::Side2], [DiceSide::Side1, DiceSide::Side4, DiceSide::Side6])),
        (DiceSide::Side4, ([DiceSide::Side2, DiceSide::Side3, DiceSide::Side5], [DiceSide::Side1, DiceSide::Side3, DiceSide::Side6])),
        (DiceSide::Side5, ([DiceSide::Side4, DiceSide::Side2, DiceSide::Side3], [DiceSide::Side1, DiceSide::Side2, DiceSide::Side6])),
        (DiceSide::Side6, ([DiceSide::Side2, DiceSide::Side1, DiceSide::Side5], [DiceSide::Side4, DiceSide::Side1, DiceSide::Side3])),
    ];

    fn new() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((50, 100), Rotation::Rot000)),
            (DiceSide::Side2, ((0, 1000), Rotation::Rot270)),
            (DiceSide::Side3, ((50, 50), Rotation::Rot000)),
            (DiceSide::Side4, ((0, 150), Rotation::Rot270)),
            (DiceSide::Side5, ((100, 0), Rotation::Rot180)),
            (DiceSide::Side6, ((50, 0), Rotation::Rot270)),
        ]);

        Self {
            side_length: 50,
            dice_sides_info,
            side_complements: HashMap::from(InputInfo::COMPLEMENTS),
        }
    }

    fn example_1() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((2, 4), Rotation::Rot000)),
            (DiceSide::Side2, ((0, 4), Rotation::Rot270)),
            (DiceSide::Side3, ((2, 2), Rotation::Rot000)),
            (DiceSide::Side4, ((0, 6), Rotation::Rot270)),
            (DiceSide::Side5, ((4, 0), Rotation::Rot270)),
            (DiceSide::Side6, ((2, 0), Rotation::Rot180)),
        ]);

        Self {
            side_length: 2,
            dice_sides_info,
            side_complements: HashMap::from(InputInfo::COMPLEMENTS),
        }
    }
}

fn rotate(map: &Map, rotation: &Rotation) -> Map {
    match rotation {
        Rotation::Rot000 => map.clone(),
        Rotation::Rot090 => rotate_once(map),
        Rotation::Rot180 => rotate_once(&rotate_once(map)),
        Rotation::Rot270 => rotate_once(&rotate_once(&rotate_once(map))),
    }
}

fn rotate_once<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    let n = v.len();
    (0..n).map(|i| (0..n).map(|j| v[n - j - 1][i].clone()).collect::<Vec<_>>()).collect()
}

pub fn monkey_map_part_2(file_name: &str) -> i32 {
    let input_info = InputInfo::example_1();

    // map of each rotation of each side, e.g.
    // side_rotations[DiceSide::Side1][Rotation::000] = [[(50,50),(50,51)], [(51,50),(51,51)]]
    let mut side_rotations: HashMap<DiceSide, HashMap<Rotation, Map>> = HashMap::new();

    let (map, instructions) = read_input(file_name);
    print_map(&map);
    println!("{:?}", instructions);

    // inserting rotations from original map and creating missing ones
    for (dice_side, ((x_begin, y_begin), rot)) in input_info.dice_sides_info {
        let y_range = y_begin..(y_begin + input_info.side_length);
        let side_map = y_range.map(|y| {
            let x_range = x_begin..(x_begin + input_info.side_length);
            (x_range).map(|x| map[y][x]).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let four_rotations = match rot {
            Rotation::Rot000 => [
                rotate(&side_map, &Rotation::Rot000),
                rotate(&side_map, &Rotation::Rot090),
                rotate(&side_map, &Rotation::Rot180),
                rotate(&side_map, &Rotation::Rot270),
            ],
            Rotation::Rot090 => [
                rotate(&side_map, &Rotation::Rot270),
                rotate(&side_map, &Rotation::Rot000),
                rotate(&side_map, &Rotation::Rot090),
                rotate(&side_map, &Rotation::Rot180),
            ],
            Rotation::Rot180 => [
                rotate(&side_map, &Rotation::Rot180),
                rotate(&side_map, &Rotation::Rot270),
                rotate(&side_map, &Rotation::Rot000),
                rotate(&side_map, &Rotation::Rot090),
            ],
            Rotation::Rot270 => [
                rotate(&side_map, &Rotation::Rot090),
                rotate(&side_map, &Rotation::Rot180),
                rotate(&side_map, &Rotation::Rot270),
                rotate(&side_map, &Rotation::Rot000),
            ],
        };

        let rotations = Rotation::iter()
            .zip(four_rotations.into_iter())
            .collect::<HashMap<_, _>>();

        side_rotations.insert(dice_side, rotations);
    }

    for (side, rotations) in side_rotations.iter().sorted_by_key(|&(s, _)| s) {
        println!("dice side: {:?}", side);
        for (rotation, map) in rotations.iter().sorted() {
            println!("{:?} ->", rotation);
            print_map(map);
        }
    }

    // let mut row = 0_i32;
    // let mut col = rows[&0].0.0;
    // let mut dir = Direction::Right;
    //
    // for instruction in instructions.into_iter() {
    //     match instruction {
    //         MoveInstruction::TurnLeft => dir.turn_left(),
    //         MoveInstruction::TurnRight => dir.turn_right(),
    //         MoveInstruction::Go(amount) => {
    //             println!("\npos: {:?}", (row, col, dir));
    //
    //             let dir_factor = match dir {
    //                 Direction::Left | Direction::Up => -1,
    //                 Direction::Right | Direction::Down => 1,
    //             };
    //             let (info, pos) = match dir {
    //                 Direction::Right | Direction::Left => (&rows[&(row as usize)], &mut col),
    //                 Direction::Up | Direction::Down => (&cols[&(col as usize)], &mut row),
    //             };
    //             let ((start, end), obstacles) = info;
    //
    //             println!("{obstacles:?}");
    //
    //             let idx = obstacles.partition_point(|&x| x < *pos);
    //             let next_obstacle = if dir_factor == 1 { obstacles[idx] } else { obstacles[idx - 1] };
    //             println!("closest obstacles: {:?}", next_obstacle);
    //
    //             let max_dist = (next_obstacle - *pos - dir_factor).abs();
    //             println!("move amount: {:3}\tmax dist: {}", amount, max_dist);
    //
    //             *pos += dir_factor * max_dist.min(amount as i32);
    //
    //             let len = end - start + 1;
    //             if *pos < *start {
    //                 while *pos < *start { *pos += len; };
    //             } else if *pos > *end {
    //                 while *pos > *end { *pos -= len; };
    //             };
    //         }
    //     }
    // }
    //
    // 1000 * (row + 1) + 4 * (col + 1) + match dir {
    //     Direction::Right => 0,
    //     Direction::Down => 1,
    //     Direction::Left => 2,
    //     Direction::Up => 3,
    // }
    todo!()
}
