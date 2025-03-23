use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
enum ObstacleType {
    Box,
    Wall,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn move_to(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Position {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Position {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Position {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

fn read_directions(file_name: &str) -> Vec<Direction> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .skip_while(|line| !line.is_empty())
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!("invalid character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn read_input(file_name: &str) -> (HashMap<Position, ObstacleType>, Position, Vec<Direction>) {
    let mut obstacles = HashMap::new();
    let mut robot_pos = Position { row: 0, col: 0 };

    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .enumerate()
        .take_while(|(_, line)| !line.is_empty())
        .for_each(|(row_i, line)| {
            line.chars().enumerate().for_each(|(col_i, c)| match c {
                'O' => {
                    obstacles.insert(
                        Position {
                            row: row_i,
                            col: col_i,
                        },
                        ObstacleType::Box,
                    );
                }
                '#' => {
                    obstacles.insert(
                        Position {
                            row: row_i,
                            col: col_i,
                        },
                        ObstacleType::Wall,
                    );
                }
                '@' => {
                    robot_pos = Position {
                        row: row_i,
                        col: col_i,
                    };
                }
                '.' => {}
                _ => panic!("invalid character: {}", c),
            })
        });

    let directions = read_directions(file_name);

    (obstacles, robot_pos, directions)
}

fn read_input_2(file_name: &str) -> (HashMap<Position, usize>, Position, Vec<Direction>) {
    let mut obstacles = HashMap::new();
    let mut robot_pos = Position { row: 0, col: 0 };
    let mut obstacle_num = 1; // 0 is reserved for Walls

    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .enumerate()
        .take_while(|(_, line)| !line.is_empty())
        .for_each(|(row_i, line)| {
            let mut row_obstacles_num = 0;
            line.chars().enumerate().for_each(|(col_i, c)| match c {
                'O' => {
                    obstacles.insert(
                        Position {
                            row: row_i,
                            col: 2 * col_i,
                        },
                        obstacle_num,
                    );
                    obstacles.insert(
                        Position {
                            row: row_i,
                            col: 2 * col_i + 1,
                        },
                        obstacle_num + 1,
                    );
                    obstacle_num += 2;
                    row_obstacles_num += 1;
                }
                '#' => {
                    obstacles.insert(
                        Position {
                            row: row_i,
                            col: 2 * col_i,
                        },
                        0,
                    );
                    obstacles.insert(
                        Position {
                            row: row_i,
                            col: 2 * col_i + 1,
                        },
                        0,
                    );
                }
                '@' => {
                    robot_pos = Position {
                        row: row_i,
                        col: 2 * row_obstacles_num + 1 + col_i,
                    };
                }
                '.' => {}
                _ => panic!("invalid character: {}", c),
            })
        });

    let directions = read_directions(file_name);

    (obstacles, robot_pos, directions)
}

fn warehouse_woes_part_1(filename: &str) -> usize {
    let (mut map, mut robot_pos, directions) = read_input(filename);

    for direction in directions {
        // println!("Move {:}:", direction);
        try_move(&mut map, &mut robot_pos, &direction, true);
    }
    print_map(&map, &robot_pos);

    map.iter()
        .filter(|&(_, o)| *o == ObstacleType::Box)
        .map(|(pos, _)| pos.row * 100 + pos.col)
        .sum()
}

fn get_map_boundaries(map: &HashMap<Position, ObstacleType>) -> (usize, usize) {
    let max_row = map.keys().map(|p| p.row).max().unwrap_or(0);
    let max_col = map.keys().map(|p| p.col).max().unwrap_or(0);
    (max_row, max_col)
}

fn print_map(map: &HashMap<Position, ObstacleType>, robot_pos: &Position) {
    let (max_row, max_col) = get_map_boundaries(map);

    for row in 0..=max_row {
        for col in 0..=max_col {
            let pos = Position { row, col };
            if pos == *robot_pos {
                print!("@");
            } else if let Some(obstacle) = map.get(&pos) {
                match obstacle {
                    ObstacleType::Wall => print!("#"),
                    ObstacleType::Box => print!("O"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn print_map_2(map: &HashMap<Position, usize>, robot_pos: &Position) {
    let max_row = map.keys().map(|p| p.row).max().unwrap_or(0);
    let max_col = map.keys().map(|p| p.col).max().unwrap_or(0);

    for row in 0..=max_row {
        for col in 0..=max_col {
            let pos = Position { row, col };
            if pos == *robot_pos {
                print!("@");
            } else if let Some(obstacle) = map.get(&pos) {
                match obstacle {
                    0 => print!("#"),
                    x => {
                        if x % 2 == 0 {
                            print!("]");
                        } else {
                            print!("[");
                        }
                    }
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn try_move(
    map: &mut HashMap<Position, ObstacleType>,
    current_pos: &mut Position,
    direction: &Direction,
    is_robot: bool,
) -> bool {
    let next_position = current_pos.move_to(direction);

    match map.get(&next_position) {
        None => {
            if is_robot {
                *current_pos = next_position;
            } else {
                map.remove_entry(&current_pos);
                map.insert(next_position, ObstacleType::Box);
            }
            true
        }
        Some(obstacle_type) => {
            let mut obstacle_pos = next_position;
            if *obstacle_type == ObstacleType::Wall {
                false
            } else {
                let can_move = try_move(map, &mut obstacle_pos, direction, false);
                if can_move {
                    map.remove_entry(&obstacle_pos);
                    map.insert(obstacle_pos.move_to(direction), ObstacleType::Box);

                    if is_robot {
                        *current_pos = obstacle_pos;
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn warehouse_woes_part_2(filename: &str) -> u64 {
    let (map, robot_pos, directions) = read_input_2(filename);

    for direction in directions {
        // try_move(&mut map, &mut robot_pos, &direction, true);
        // println!("Move {:}:", direction);
        print_map_2(&map, &robot_pos);
    }

    print_map_2(&map, &robot_pos);
    todo!()
}

fn can_move_robot(
    map: &mut HashMap<Position, usize>,
    current_pos: &Position,
    direction: &Direction,
) -> bool {
    let next_position = current_pos.move_to(direction);

    match map.get(&next_position) {
        None => true,
        Some(obstacle_num) => {
            let mut obstacle_pos = next_position;
            if *obstacle_num == 0 {
                false
            } else {
                can_move_box(map, &mut obstacle_pos, direction)
            }
        }
    }
}

fn can_move_box(
    map: &mut HashMap<Position, usize>,
    current_pos: &Position,
    direction: &Direction,
) -> bool {
    let box_positions = get_box_positions(map, &current_pos);
    
    match direction {
        Direction::Up => {
            
        }
        Direction::Down => {}
        Direction::Left => {}
        Direction::Right => {}
    }
    
    let next_position = current_pos.move_to(direction);

    match map.get(&next_position) {
        None => true,
        Some(obstacle_num) => {
            let mut obstacle_pos = next_position;
            if *obstacle_num == 0 {
                false
            } else {
                let can_move = try_move(map, &mut obstacle_pos, direction, false);
                if can_move {
                    map.remove_entry(&obstacle_pos);
                    map.insert(obstacle_pos.move_to(direction), ObstacleType::Box);

                    if is_robot {
                        *current_pos = obstacle_pos;
                    }
                    true
                } else {
                    false
                }
            }
        }
    }
}

fn get_box_positions(map: &mut HashMap<Position, usize>, current_pos: &Position) -> [Position; 2] {
    let pos_to_left = Position {
        row: current_pos.row,
        col: current_pos.col - 1,
    };

    let pos_to_right = Position {
        row: current_pos.row,
        col: current_pos.col + 1,
    };

    let box_num = map.get(&current_pos).unwrap();
    let box_num_to_left = map.get(&pos_to_left);

    if let Some(box_num_to_left) = box_num_to_left {
        if box_num_to_left == box_num {
            [pos_to_left, *current_pos]
        } else {
            [*current_pos, pos_to_right]
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_test() {
        // let (map, directions) = read_input("inputs/15_input.txt");
        // println!("{:?}", directions.len())
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = warehouse_woes_part_1("inputs/15_input_example_2.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 10092);
    }

    #[test]
    fn part_1_input() {
        let answer = warehouse_woes_part_1("inputs/15_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 1456590);
    }

    #[test]
    fn part_2_input_example() {
        let answer = warehouse_woes_part_2("inputs/15_input_example_3.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 875318608908);
    }

    #[test]
    fn part_2_input() {
        let answer = warehouse_woes_part_2("inputs/15_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 99423413811305);
    }
}
