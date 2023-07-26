use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Pointer};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, EnumIter, Ord, PartialOrd)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn turn_right(&mut self) {
        println!("turning right");
        *self = match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }

    fn turn_left(&mut self) {
        println!("turning left");
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
                -1 => "  ".to_string(),
                0 => ". ".to_string(),
                1 => "# ".to_string(),
                _ => format!("{tile:2}")
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
// y (0,0) (0,1)
// | (0,1) (1,1)
// v

type PointsMap = Vec<Vec<Point>>;
type EdgeIdentifier = (DiceSide, Direction);
type DiceSideInfo = ((usize, usize), Direction);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Copy, Clone, Debug, Eq, EnumIter, Ord, PartialOrd, PartialEq, Hash)]
enum DiceSide {
    Side1,
    Side2,
    Side3,
    Side4,
    Side5,
    Side6,
}

// info about relations between dice's sides
// dice net:
//   ■       |   6
//   ■       |   3
// ■ ■ ■     | 2 1 5
//   ■       |   4
struct DiceInfo {
    /// Map with information about adjacent edges of the dice and information if order of points is reversed
    adjacent_edges: HashMap<EdgeIdentifier, (EdgeIdentifier, bool)>,
}

impl DiceInfo {
    fn new() -> Self {
        let adjacent_edges = HashMap::from([
            ((DiceSide::Side1, Direction::Up), ((DiceSide::Side3, Direction::Down), false)),
            ((DiceSide::Side1, Direction::Down), ((DiceSide::Side4, Direction::Down), true)),
            ((DiceSide::Side1, Direction::Left), ((DiceSide::Side2, Direction::Down), true)),
            ((DiceSide::Side1, Direction::Right), ((DiceSide::Side5, Direction::Down), false)),
            ((DiceSide::Side2, Direction::Up), ((DiceSide::Side6, Direction::Right), true)),
            ((DiceSide::Side2, Direction::Down), ((DiceSide::Side1, Direction::Left), true)),
            ((DiceSide::Side2, Direction::Left), ((DiceSide::Side4, Direction::Right), false)),
            ((DiceSide::Side2, Direction::Right), ((DiceSide::Side3, Direction::Left), false)),
            ((DiceSide::Side3, Direction::Up), ((DiceSide::Side6, Direction::Up), true)),
            ((DiceSide::Side3, Direction::Down), ((DiceSide::Side1, Direction::Up), false)),
            ((DiceSide::Side3, Direction::Left), ((DiceSide::Side2, Direction::Right), false)),
            ((DiceSide::Side3, Direction::Right), ((DiceSide::Side5, Direction::Left), false)),
            ((DiceSide::Side4, Direction::Up), ((DiceSide::Side6, Direction::Down), false)),
            ((DiceSide::Side4, Direction::Down), ((DiceSide::Side1, Direction::Down), true)),
            ((DiceSide::Side4, Direction::Left), ((DiceSide::Side5, Direction::Right), false)),
            ((DiceSide::Side4, Direction::Right), ((DiceSide::Side2, Direction::Left), false)),
            ((DiceSide::Side5, Direction::Up), ((DiceSide::Side6, Direction::Left), false)),
            ((DiceSide::Side5, Direction::Down), ((DiceSide::Side1, Direction::Right), false)),
            ((DiceSide::Side5, Direction::Left), ((DiceSide::Side3, Direction::Right), false)),
            ((DiceSide::Side5, Direction::Right), ((DiceSide::Side4, Direction::Left), false)),
            ((DiceSide::Side6, Direction::Up), ((DiceSide::Side3, Direction::Up), true)),
            ((DiceSide::Side6, Direction::Down), ((DiceSide::Side4, Direction::Up), false)),
            ((DiceSide::Side6, Direction::Left), ((DiceSide::Side5, Direction::Up), false)),
            ((DiceSide::Side6, Direction::Right), ((DiceSide::Side2, Direction::Up), true)),
        ]);

        Self {
            adjacent_edges,
        }
    }
}

pub struct InputInfo {
    start_side: DiceSide,
    start_dir: Direction,
    // length of an edge of a dice's side
    side_length: usize,
    // cube's net of my input; maps side of a dice from an input net to coords (x,y) of upper-left corner
    // and its rotation relative to original rotation in dice
    dice_sides_info: HashMap<DiceSide, DiceSideInfo>,
}

impl InputInfo {
    pub fn input22() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((50, 100), Direction::Up)),
            (DiceSide::Side2, ((0, 100), Direction::Left)),
            (DiceSide::Side3, ((50, 50), Direction::Up)),
            (DiceSide::Side4, ((0, 150), Direction::Left)),
            (DiceSide::Side5, ((100, 0), Direction::Left)),
            (DiceSide::Side6, ((50, 0), Direction::Down)),
        ]);

        Self {
            start_side: DiceSide::Side6,
            start_dir: Direction::Left,
            side_length: 50,
            dice_sides_info,
        }
    }

    pub fn input22_example() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((8, 4), Direction::Up)),
            (DiceSide::Side2, ((4, 4), Direction::Left)),
            (DiceSide::Side3, ((8, 0), Direction::Up)),
            (DiceSide::Side4, ((8, 8), Direction::Down)),
            (DiceSide::Side5, ((12, 8), Direction::Down)),
            (DiceSide::Side6, ((0, 4), Direction::Up)),
        ]);

        Self {
            start_side: DiceSide::Side3,
            start_dir: Direction::Right,
            side_length: 4,
            dice_sides_info,
        }
    }

    pub fn input22_example_1() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((2, 4), Direction::Up)),
            (DiceSide::Side2, ((0, 4), Direction::Left)),
            (DiceSide::Side3, ((2, 2), Direction::Up)),
            (DiceSide::Side4, ((0, 6), Direction::Left)),
            (DiceSide::Side5, ((4, 0), Direction::Left)),
            (DiceSide::Side6, ((2, 0), Direction::Down)),
        ]);

        Self {
            start_side: DiceSide::Side6,
            start_dir: Direction::Left,
            side_length: 2,
            dice_sides_info,
        }
    }

    pub fn input22_example_2() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((4, 8), Direction::Up)),
            (DiceSide::Side2, ((0, 8), Direction::Left)),
            (DiceSide::Side3, ((4, 4), Direction::Up)),
            (DiceSide::Side4, ((0, 12), Direction::Left)),
            (DiceSide::Side5, ((8, 0), Direction::Left)),
            (DiceSide::Side6, ((4, 0), Direction::Down)),
        ]);

        Self {
            start_side: DiceSide::Side6,
            start_dir: Direction::Left,
            side_length: 4,
            dice_sides_info,
        }
    }

    pub fn input22_example_3() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((4, 8), Direction::Up)),
            (DiceSide::Side2, ((0, 8), Direction::Left)),
            (DiceSide::Side3, ((4, 4), Direction::Up)),
            (DiceSide::Side4, ((0, 12), Direction::Left)),
            (DiceSide::Side5, ((8, 0), Direction::Left)),
            (DiceSide::Side6, ((4, 0), Direction::Down)),
        ]);

        Self {
            start_side: DiceSide::Side6,
            start_dir: Direction::Left,
            side_length: 4,
            dice_sides_info,
        }
    }
}

// rotates map till top of a side will point to up direction
fn rotate(map: &Vec<Vec<Point>>, initial_rotation: &Direction) -> PointsMap {
    match initial_rotation {
        Direction::Up => map.clone(),
        Direction::Left => rotate_once(map),
        Direction::Down => rotate_once(&rotate_once(map)),
        // Direction::Down => mirror_horizontally(map),
        Direction::Right => rotate_once(&rotate_once(&rotate_once(map))),
    }
}

fn mirror_horizontally<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    let n = v.len();
    (0..n).map(|i| v[n - 1 - i].clone()).collect::<Vec<_>>()
}

fn rotate_once<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    let n = v.len();
    (0..n).map(|i| (0..n).map(|j| v[n - j - 1][i].clone()).collect::<Vec<_>>()).collect()
}

/// 1 2 3
/// 4 5 6
/// 7 8 9
///
/// gives:
/// up    -> [1,2,3]
/// right -> [3,6,9]
/// down  -> [7,8,9]
/// left  -> [1,4,7]
fn get_edges(side_maps: &HashMap<DiceSide, PointsMap>) -> HashMap<EdgeIdentifier, Vec<Point>> {
    let mut edges = HashMap::new();
    for (dice_side, map) in side_maps {
        // top edge
        edges.insert((*dice_side, Direction::Up), map.first().unwrap().clone());

        // bottom edge
        edges.insert((*dice_side, Direction::Down), map.last().unwrap().clone());

        // right edge
        let right_edge = map.iter().map(|row| *row.last().unwrap()).collect::<Vec<_>>();
        edges.insert((*dice_side, Direction::Right), right_edge);

        // left edge
        let left_edge = map.iter().map(|row| *row.first().unwrap()).collect::<Vec<_>>();
        edges.insert((*dice_side, Direction::Left), left_edge);
    }

    edges
}

fn find_neighbors(
    input_info: &InputInfo,
    sides_transformations: &HashMap<DiceSide, PointsMap>,
    edges: &HashMap<EdgeIdentifier, Vec<Point>>,
) -> HashMap<Point, HashMap<Direction, Point>> {
    let mut neighbors = HashMap::new();

    let dice_info = DiceInfo::new();
    let adjacent_edges = dice_info.adjacent_edges;

    for (&side, side_map) in sides_transformations {
        let n = input_info.side_length;
        for row in 0..n {
            for col in 0..n {
                let curr_point = side_map[row][col];
                let mut tiles_around = HashMap::new();

                // cannot go left
                if col == 0 {
                    let (edge_identifier, is_reversed) = &adjacent_edges[&(side, Direction::Left)];
                    let adjacent_edge = &edges[edge_identifier];
                    let index = if !is_reversed { row } else { n - 1 - row };
                    tiles_around.insert(Direction::Left, adjacent_edge[index]);
                }

                // cannot go right
                if col == n - 1 {
                    let (edge_identifier, is_reversed) = &adjacent_edges[&(side, Direction::Right)];
                    let adjacent_edge = &edges[edge_identifier];
                    let index = if !is_reversed { row } else { n - 1 - row };
                    tiles_around.insert(Direction::Right, adjacent_edge[index]);
                };

                // we cannot go up:
                if row == 0 {
                    let (edge_identifier, is_reversed) = &adjacent_edges[&(side, Direction::Up)];
                    let adjacent_edge = &edges[edge_identifier];
                    let index = if !is_reversed { col } else { n - 1 - col };

                    tiles_around.insert(Direction::Up, adjacent_edge[index]);
                }

                // we cannot go down:
                if row == n - 1 {
                    let (edge_identifier, is_reversed) = &adjacent_edges[&(side, Direction::Down)];
                    let adjacent_edge = &edges[edge_identifier];
                    let index = if !is_reversed { col } else { n - 1 - col };

                    tiles_around.insert(Direction::Down, adjacent_edge[index]);
                }

                for dir in Direction::iter() {
                    if tiles_around.contains_key(&dir) {
                        continue;
                    }

                    let tile_point = match dir {
                        Direction::Left => side_map[row][col - 1],
                        Direction::Right => side_map[row][col + 1],
                        Direction::Up => side_map[row - 1][col],
                        Direction::Down => side_map[row + 1][col],
                    };

                    tiles_around.insert(dir, tile_point);
                }

                neighbors.insert(curr_point, tiles_around);
            }
        }
    }

    neighbors
}

fn create_sides_maps(input_info: &InputInfo) -> HashMap<DiceSide, PointsMap> {
    let mut sides_maps = HashMap::new();

    for (&dice_side, ((x_begin, y_begin), top_direction)) in input_info.dice_sides_info.iter() {
        let n = input_info.side_length;
        let points_map = (*y_begin..(*y_begin + n)).map(|y| {
            (*x_begin..(*x_begin + n)).map(|x| Point { x, y }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let normalized_side = rotate(&points_map, top_direction);

        sides_maps.insert(dice_side, normalized_side);
    }

    sides_maps
}

pub fn monkey_map_part_2(file_name: &str, input_info: InputInfo) -> usize {
    let (original_map, instructions) = read_input(file_name);
    print_map(&original_map);
    println!("{:?}", instructions);

    let sides_maps = create_sides_maps(&input_info);
    print_rotations(&sides_maps);

    let edges = get_edges(&sides_maps);
    println!("Edges");
    for (key, vec) in edges.iter().sorted() {
        println!("{key:?} -> {vec:?}");
    }

    let neighbors: HashMap<Point, HashMap<Direction, Point>> = find_neighbors(&input_info, &sides_maps, &edges);
    // print neighbors
    for (center, around) in neighbors.iter().sorted_by_key(|&(p, _)| p) {
        let up = around[&Direction::Up];
        let right = around[&Direction::Right];
        let down = around[&Direction::Down];
        let left = around[&Direction::Left];

        println!("{center:?} -> up:{up:?} right:{right:?} down:{down:?} left:{left:?}")
    }


    let mut start_x = 0;
    while original_map[0][start_x] != 0 { start_x += 1; };

    // let mut curr_point = Point { x: 5, y: 1 };
    // let mut curr_side = DiceSide::Side6;
    // let mut dir = Direction::Up;
    let mut curr_point = Point { x: start_x, y: 0 };
    let mut curr_side = input_info.start_side;
    let mut dir = input_info.start_dir;
    let mut walk_history = Vec::new();
    let mut walk_ins_counter = 0;

    for instruction in instructions.into_iter() {
        match instruction {
            MoveInstruction::TurnLeft => {
                walk_ins_counter -= 1;
                dir.turn_left();
            }
            MoveInstruction::TurnRight => {
                walk_ins_counter -= 1;
                dir.turn_right();
            }
            MoveInstruction::Go(amount) => {
                println!("\npos: {:?}", (curr_point, dir));

                let mut counter = 0usize;
                loop {
                    walk_ins_counter += 1;
                    match walk_history.iter().map(|(p, _)| p).position(|&p| p == curr_point) {
                        Some(pos) => {
                            walk_history[pos] = (curr_point, walk_ins_counter);
                        }
                        None => {
                            walk_history.push((curr_point, walk_ins_counter));
                        }
                    }

                    // for (row_y, row) in original_map.iter().enumerate() {
                    //     for (tile_x, tile) in row.iter().enumerate() {
                    //         let p = Point { x: tile_x, y: row_y };
                    //
                    //         let sign = match walk_history.iter().find(|&(pp, _)| *pp == p) {
                    //             Some((_, index)) => format!("{:3}", index % 1000),
                    //             None => match tile {
                    //                 -1 => "   ".to_string(),
                    //                 0 => "  .".to_string(),
                    //                 1 => "  #".to_string(),
                    //                 _ => panic!()
                    //             }
                    //         };
                    //
                    //         print!("{sign}");
                    //     }
                    //     println!();
                    // }

                    // when jumping to other side we need to adjust direction
                    for (dice_side, side_map) in sides_maps.iter() {
                        if side_map.iter().flatten().contains(&curr_point) && curr_side != *dice_side {
                            let new_dir = match (curr_side, *dice_side) {
                                (DiceSide::Side1, DiceSide::Side2) => Direction::Up,
                                (DiceSide::Side1, DiceSide::Side3) => Direction::Up,
                                (DiceSide::Side1, DiceSide::Side4) => Direction::Up,
                                (DiceSide::Side1, DiceSide::Side5) => Direction::Up,

                                (DiceSide::Side2, DiceSide::Side1) => Direction::Right,
                                (DiceSide::Side2, DiceSide::Side4) => Direction::Left,
                                (DiceSide::Side2, DiceSide::Side3) => Direction::Right,
                                (DiceSide::Side2, DiceSide::Side6) => Direction::Left,

                                (DiceSide::Side3, DiceSide::Side1) => Direction::Down,
                                (DiceSide::Side3, DiceSide::Side2) => Direction::Left,
                                (DiceSide::Side3, DiceSide::Side5) => Direction::Right,
                                (DiceSide::Side3, DiceSide::Side6) => Direction::Down,

                                (DiceSide::Side4, DiceSide::Side1) => Direction::Up,
                                (DiceSide::Side4, DiceSide::Side2) => Direction::Right,
                                (DiceSide::Side4, DiceSide::Side5) => Direction::Left,
                                (DiceSide::Side4, DiceSide::Side6) => Direction::Up,

                                (DiceSide::Side5, DiceSide::Side1) => Direction::Left,
                                (DiceSide::Side5, DiceSide::Side3) => Direction::Left,
                                (DiceSide::Side5, DiceSide::Side4) => Direction::Right,
                                (DiceSide::Side5, DiceSide::Side6) => Direction::Right,

                                (DiceSide::Side6, DiceSide::Side2) => Direction::Down,
                                (DiceSide::Side6, DiceSide::Side3) => Direction::Down,
                                (DiceSide::Side6, DiceSide::Side4) => Direction::Down,
                                (DiceSide::Side6, DiceSide::Side5) => Direction::Down,
                                _ => dir,
                            };

                            println!("jumped for {curr_side:?} to {dice_side:?} and change dir from {dir:?} to {new_dir:?}");
                            dir = new_dir;
                            curr_side = *dice_side;
                        }
                    }

                    if counter == amount {
                        println!("moved exactly {amount} of tiles");
                        break;
                    }

                    let next_point = neighbors[&curr_point][&dir];

                    if original_map[next_point.y][next_point.x] == 1 {
                        println!("obstacle encountered on {:?}", (next_point.x, next_point.y));
                        break;
                    }

                    // println!("neighbors of next current point ({:?}):", curr_point);
                    // for (neighbor_dir, neighbor_point) in &neighbors[&curr_point] {
                    //     println!("\tdir: {:?} -> {:?}", neighbor_dir, neighbor_point);
                    // }

                    curr_point = next_point;
                    counter += 1;
                }
            }
        }
    }

    for (row_y, row) in original_map.iter().enumerate() {
        for (tile_x, tile) in row.iter().enumerate() {
            let p = Point { x: tile_x, y: row_y };

            let sign = match walk_history.iter().find(|&(pp, _)| *pp == p) {
                Some((_, index)) => format!("{:4}", index % 1000),
                None => match tile {
                    -1 => "    ".to_string(),
                    0 => "   .".to_string(),
                    1 => "   #".to_string(),
                    _ => panic!()
                }
            };

            print!("{sign}");
        }
        println!();
    }

    println!("final point: {curr_point:?} with dir: {dir:?}");

    let (_, original_dir) = input_info.dice_sides_info[&curr_side];
    if original_dir != Direction::Up {
        match original_dir {
            Direction::Up => {}
            Direction::Left => dir.turn_left(),
            Direction::Right => dir.turn_right(),
            Direction::Down => {
                dir.turn_right();
                dir.turn_right();
            }
        };
    }

    1000 * (curr_point.y + 1) + 4 * (curr_point.x + 1) + match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

fn print_rotations(side_rotations: &HashMap<DiceSide, PointsMap>) {
    for (side, map) in side_rotations.iter().sorted_by_key(|&(s, _)| s) {
        println!("dice side: {:?}", side);
        for row in map {
            for point in row {
                print!("{:?}", point);
            }
            println!();
        }
    }
}
