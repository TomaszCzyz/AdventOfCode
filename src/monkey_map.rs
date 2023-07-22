use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Pointer};
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, EnumIter, Ord, PartialOrd)]
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

type PointsMap = Vec<Vec<Point>>;
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

impl Point {
    fn get_adjacent_point(&self, dir: Direction) -> Point {
        let (x, y) = (self.x, self.y);
        match dir {
            Direction::Left => Point { x: x - 1, y },
            Direction::Right => Point { x: x + 1, y },
            Direction::Up => Point { x, y: y - 1 },
            Direction::Down => Point { x, y: y + 1 },
        }
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
    // maps side to its sides that are to the right and to the bottom ("orbits")
    side_complements: HashMap<DiceSide, ([DiceSide; 3], [DiceSide; 3])>,
    /// The relative rotations between two adjacent sides.
    /// Relation represents following:
    /// "to walk from Side1 to Side2, Side2 should be rotated 270 degree"
    jump_rotations: [[Direction; 6]; 6],
    /// Map with information about adjacent edges of the dice
    adjacent_edges: HashMap<(DiceSide, Direction), (DiceSide, Direction)>,
}

impl DiceInfo {
    const COMPLEMENTS: [(DiceSide, ([DiceSide; 3], [DiceSide; 3])); 6] = [
        (DiceSide::Side1, ([DiceSide::Side5, DiceSide::Side6, DiceSide::Side2], [DiceSide::Side4, DiceSide::Side6, DiceSide::Side3])),
        (DiceSide::Side2, ([DiceSide::Side3, DiceSide::Side5, DiceSide::Side4], [DiceSide::Side1, DiceSide::Side5, DiceSide::Side6])),
        (DiceSide::Side3, ([DiceSide::Side5, DiceSide::Side4, DiceSide::Side2], [DiceSide::Side1, DiceSide::Side4, DiceSide::Side6])),
        (DiceSide::Side4, ([DiceSide::Side2, DiceSide::Side3, DiceSide::Side5], [DiceSide::Side1, DiceSide::Side3, DiceSide::Side6])),
        (DiceSide::Side5, ([DiceSide::Side4, DiceSide::Side2, DiceSide::Side3], [DiceSide::Side1, DiceSide::Side2, DiceSide::Side6])),
        (DiceSide::Side6, ([DiceSide::Side2, DiceSide::Side1, DiceSide::Side5], [DiceSide::Side4, DiceSide::Side1, DiceSide::Side3])),
    ];

    const JUMP_ROT: [[Direction; 6]; 6] = [
        [Direction::Up, Direction::Left, Direction::Up, Direction::Down, Direction::Right, Direction::Up],
        [Direction::Right, Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Right],
        [Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Down],
        [Direction::Down, Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Up],
        [Direction::Right, Direction::Up, Direction::Up, Direction::Up, Direction::Up, Direction::Right],
        [Direction::Up, Direction::Left, Direction::Down, Direction::Up, Direction::Right, Direction::Up],
    ];


    fn new() -> Self {
        let adjacent_edges = HashMap::from([
            ((DiceSide::Side1, Direction::Up), (DiceSide::Side3, Direction::Down)),
            ((DiceSide::Side1, Direction::Down), (DiceSide::Side4, Direction::Down)),
            ((DiceSide::Side1, Direction::Left), (DiceSide::Side2, Direction::Down)),
            ((DiceSide::Side1, Direction::Right), (DiceSide::Side5, Direction::Down)),
            ((DiceSide::Side2, Direction::Up), (DiceSide::Side6, Direction::Right)),
            ((DiceSide::Side2, Direction::Down), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side2, Direction::Left), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side2, Direction::Right), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side3, Direction::Up), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side3, Direction::Down), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side3, Direction::Left), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side3, Direction::Right), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side4, Direction::Up), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side4, Direction::Down), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side4, Direction::Left), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side4, Direction::Right), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side5, Direction::Up), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side5, Direction::Down), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side5, Direction::Left), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side5, Direction::Right), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side6, Direction::Up), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side6, Direction::Down), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side6, Direction::Left), (DiceSide::Side1, Direction::Up)),
            ((DiceSide::Side6, Direction::Right), (DiceSide::Side1, Direction::Up)),
        ]);

        Self {
            side_complements: HashMap::from(DiceInfo::COMPLEMENTS),
            jump_rotations: DiceInfo::JUMP_ROT,
            adjacent_edges,
        }
    }
}


struct InputInfo {
    // length of an edge of a dice's side
    side_length: usize,
    // cube's net of my input; maps side of a dice from an input net to coords (x,y) of upper-left corner
    // and its rotation relative to original rotation in dice
    dice_sides_info: HashMap<DiceSide, DiceSideInfo>,
}

impl InputInfo {
    fn example() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((50, 100), Direction::Up)),
            (DiceSide::Side2, ((0, 1000), Direction::Left)),
            (DiceSide::Side3, ((50, 50), Direction::Up)),
            (DiceSide::Side4, ((0, 150), Direction::Left)),
            (DiceSide::Side5, ((100, 0), Direction::Down)),
            (DiceSide::Side6, ((50, 0), Direction::Left)),
        ]);

        Self {
            side_length: 50,
            dice_sides_info,
        }
    }

    fn example_1() -> Self {
        let dice_sides_info = HashMap::from([
            (DiceSide::Side1, ((2, 4), Direction::Up)),
            (DiceSide::Side2, ((0, 4), Direction::Left)),
            (DiceSide::Side3, ((2, 2), Direction::Up)),
            (DiceSide::Side4, ((0, 6), Direction::Left)),
            (DiceSide::Side5, ((4, 0), Direction::Left)),
            (DiceSide::Side6, ((2, 0), Direction::Down)),
        ]);

        Self {
            side_length: 2,
            dice_sides_info,
        }
    }
}

fn rotate(map: &Vec<Vec<Point>>, rotation: &Direction) -> Vec<Vec<Point>> {
    match rotation {
        Direction::Up => map.clone(),
        Direction::Right => rotate_once(map),
        Direction::Down => rotate_once(&rotate_once(map)),
        Direction::Left => rotate_once(&rotate_once(&rotate_once(map))),
    }
}

fn rotate_once<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    let n = v.len();
    (0..n).map(|i| (0..n).map(|j| v[n - j - 1][i].clone()).collect::<Vec<_>>()).collect()
}

fn get_sides(side_rotations: &HashMap<DiceSide, HashMap<Direction, PointsMap>>) -> HashMap<(DiceSide, Direction), Vec<Point>> {
    let mut edges = HashMap::new();
    for (dice_side, rotation_maps) in side_rotations {
        for (rot, map) in rotation_maps {
            let dir = match rot {
                Direction::Up => Direction::Up,
                Direction::Right => Direction::Left,
                Direction::Down => Direction::Down,
                Direction::Left => Direction::Right,
            };

            edges.insert((*dice_side, dir), map[0].clone());
        }
    }

    edges
}

fn find_neighbors(
    input_info: &InputInfo,
    sides: &HashMap<(DiceSide, Direction), Vec<Point>>,
) -> HashMap<Point, HashMap<Direction, Point>> {
    let mut neighbors = HashMap::new();

    let dice_info = DiceInfo::new();
    let adjacent_edges: HashMap<(DiceSide, Direction), (DiceSide, Direction)> = dice_info.adjacent_edges;

    for side in DiceSide::iter() {
        // tiles inside side
        for y in 1..(input_info.side_length - 1) {
            for x in 1..(input_info.side_length - 1) {
                let curr_point = Point { x, y };
                let value = Direction::iter()
                    .map(|dir| (dir, curr_point.get_adjacent_point(dir)))
                    .collect::<HashMap<_, _>>();

                neighbors.insert(curr_point, value);
            }
        }

        // tiles on upper edge without corners
        let edge = &sides[&adjacent_edges[&(side, Direction::Up)]];
        for x in 1..(input_info.side_length - 1) {
            let curr_point = Point { x, y: 0 };

            let mut value = [Direction::Left, Direction::Right, Direction::Down].into_iter()
                .map(|dir| (dir, curr_point.get_adjacent_point(dir)))
                .collect::<HashMap<_, _>>();

            value.insert(Direction::Up, edge[x]);
            neighbors.insert(curr_point, value);
        }

        // tiles on bottom edge without corners
        let edge = &sides[&adjacent_edges[&(side, Direction::Down)]];
        for x in 1..(input_info.side_length - 1) {
            let curr_point = Point { x, y: input_info.side_length - 1 };

            let mut value = [Direction::Left, Direction::Right, Direction::Up].into_iter()
                .map(|dir| (dir, curr_point.get_adjacent_point(dir)))
                .collect::<HashMap<_, _>>();

            value.insert(Direction::Down, edge[x]);
            neighbors.insert(curr_point, value);
        }

        // tiles on left edge without corners
        let edge = &sides[&adjacent_edges[&(side, Direction::Left)]];
        for y in 1..(input_info.side_length - 1) {
            let curr_point = Point { x: 0, y };

            let mut value = [Direction::Down, Direction::Right, Direction::Up].into_iter()
                .map(|dir| (dir, curr_point.get_adjacent_point(dir)))
                .collect::<HashMap<_, _>>();

            value.insert(Direction::Left, edge[y]);
            neighbors.insert(curr_point, value);
        }

        // tiles on right edge without corners
        let edge = &sides[&adjacent_edges[&(side, Direction::Left)]];
        for y in 1..(input_info.side_length - 1) {
            let curr_point = Point { x: input_info.side_length - 1, y };

            let mut value = [Direction::Down, Direction::Left, Direction::Up].into_iter()
                .map(|dir| (dir, curr_point.get_adjacent_point(dir)))
                .collect::<HashMap<_, _>>();

            value.insert(Direction::Right, edge[y]);
            neighbors.insert(curr_point, value);
        }

        // Corners
        // upper-left corner
        let corner = Point { x: 0, y: 0 };
        let mut value = [Direction::Right, Direction::Down].into_iter()
            .map(|dir| (dir, corner.get_adjacent_point(dir)))
            .collect::<HashMap<_, _>>();

        value.insert(Direction::Up, sides[&adjacent_edges[&(side, Direction::Up)]][0]);
        value.insert(Direction::Left, sides[&adjacent_edges[&(side, Direction::Left)]][0]);
        neighbors.insert(corner, value);

        // upper-right corner
        let corner = Point { x: input_info.side_length - 1, y: 0 };
        let mut value = [Direction::Left, Direction::Down].into_iter()
            .map(|dir| (dir, corner.get_adjacent_point(dir)))
            .collect::<HashMap<_, _>>();

        value.insert(Direction::Up, sides[&adjacent_edges[&(side, Direction::Up)]][0]);
        value.insert(Direction::Right, *sides[&adjacent_edges[&(side, Direction::Right)]].last().unwrap());
        neighbors.insert(corner, value);

        // bottom-right corner
        let corner = Point { x: input_info.side_length - 1, y: input_info.side_length - 1 };
        let mut value = [Direction::Left, Direction::Up].into_iter()
            .map(|dir| (dir, corner.get_adjacent_point(dir)))
            .collect::<HashMap<_, _>>();

        value.insert(Direction::Down, *sides[&adjacent_edges[&(side, Direction::Down)]].last().unwrap());
        value.insert(Direction::Right, *sides[&adjacent_edges[&(side, Direction::Right)]].last().unwrap());
        neighbors.insert(corner, value);

        // bottom-left corner
        let corner = Point { x: 0, y: input_info.side_length - 1 };
        let mut value = [Direction::Right, Direction::Up].into_iter()
            .map(|dir| (dir, corner.get_adjacent_point(dir)))
            .collect::<HashMap<_, _>>();

        value.insert(Direction::Down, sides[&adjacent_edges[&(side, Direction::Down)]][0]);
        value.insert(Direction::Left, *sides[&adjacent_edges[&(side, Direction::Left)]].last().unwrap());
        neighbors.insert(corner, value);
    }

    neighbors
}

pub fn monkey_map_part_2(file_name: &str) -> usize {
    let input_info = InputInfo::example_1();

    let (original_map, instructions) = read_input(file_name);
    print_map(&original_map);
    println!("{:?}", instructions);

    let side_rotations = read_and_transform_dice_sides(&input_info);
    print_rotations(&side_rotations);

    let sides = get_sides(&side_rotations);
    for (key, vec) in &sides {
        println!("{key:?} -> {vec:?}");
    }

    let neighbors: HashMap<Point, HashMap<Direction, Point>> = find_neighbors(&input_info, &sides);
    // print neighbors
    for (center, around) in &neighbors {
        let up = around[&Direction::Up];
        let right = around[&Direction::Right];
        let down = around[&Direction::Down];
        let left = around[&Direction::Left];

        println!("{center:?} -> up:{up:?} right:{right:?} down:{down:?} left:{left:?}")
    }


    let mut curr_point = Point { x: 0, y: 0 };
    let mut dir = Direction::Right;
    let max_dist = 4 * input_info.side_length;

    for instruction in instructions.into_iter() {
        match instruction {
            MoveInstruction::TurnLeft => dir.turn_left(),
            MoveInstruction::TurnRight => dir.turn_right(),
            MoveInstruction::Go(amount) => {
                println!("\npos: {:?}", (curr_point, dir));

                let mut walk_history = Vec::new();
                let mut counter = 0usize;
                loop {
                    if counter == amount {
                        break;
                    }
                    if counter == max_dist {
                        panic!("select correct point from history");
                        // break;
                    }

                    let next_point = neighbors[&curr_point][&dir];

                    if original_map[next_point.y][next_point.x] == 1 {
                        break;
                    }

                    walk_history.push(curr_point);
                    curr_point = next_point;
                    counter += 1;
                }
            }
        }
    }

    1000 * (curr_point.y + 1) + 4 * (curr_point.x + 1) + match dir {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    }
}

fn print_rotations(side_rotations: &HashMap<DiceSide, HashMap<Direction, PointsMap>>) {
    for (side, directions) in side_rotations.iter().sorted_by_key(|&(s, _)| s) {
        println!("dice side: {:?}", side);
        for (rotation, map) in directions.iter().sorted() {
            println!("{:?} ->", rotation);

            for row in map {
                for point in row {
                    print!("{:?}", point);
                }
                println!();
            }
        }
    }
}

// inserting rotations from original map and creating missing ones
// Returns map of each rotation of each side, e.g.
// side_rotations[DiceSide::Side1][Direction::Up] = [[(50,50),(50,51)], [(51,50),(51,51)]]
fn read_and_transform_dice_sides(input_info: &InputInfo) -> HashMap<DiceSide, HashMap<Direction, PointsMap>> {
    let mut side_rotations = HashMap::new();

    for (&dice_side, ((x_begin, y_begin), rot)) in input_info.dice_sides_info.iter() {
        let n = input_info.side_length;

        let points_map = (*y_begin..(*y_begin + n)).map(|y| {
            (*x_begin..(*x_begin + n)).map(|x| Point { x, y }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        let four_rotations = match rot {
            Direction::Up => [
                rotate(&points_map, &Direction::Up),
                rotate(&points_map, &Direction::Right),
                rotate(&points_map, &Direction::Down),
                rotate(&points_map, &Direction::Left),
            ],
            Direction::Right => [
                rotate(&points_map, &Direction::Left),
                rotate(&points_map, &Direction::Up),
                rotate(&points_map, &Direction::Right),
                rotate(&points_map, &Direction::Down),
            ],
            Direction::Down => [
                rotate(&points_map, &Direction::Down),
                rotate(&points_map, &Direction::Left),
                rotate(&points_map, &Direction::Up),
                rotate(&points_map, &Direction::Right),
            ],
            Direction::Left => [
                rotate(&points_map, &Direction::Right),
                rotate(&points_map, &Direction::Down),
                rotate(&points_map, &Direction::Left),
                rotate(&points_map, &Direction::Up),
            ],
        };

        let rotations = Direction::iter()
            .zip(four_rotations.into_iter())
            .collect::<HashMap<_, _>>();

        side_rotations.insert(dice_side, rotations);
    }

    side_rotations
}
