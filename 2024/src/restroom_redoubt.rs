use itertools::{Itertools, MinMaxResult};
use std::fs;

struct Floor {
    width: i32,
    height: i32,
}

#[derive(Debug)]
struct RobotInfo {
    pos: Position,
    v_x: i32,
    v_y: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Quadrant {
    I,
    II,
    III,
    IV,
}

fn read_input(file_name: &str) -> Vec<RobotInfo> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let coords = line
                .split_whitespace()
                .map(|x| {
                    let coords = x
                        .get(2..)
                        .unwrap()
                        .split(',')
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    (coords[0], coords[1])
                })
                .collect::<Vec<_>>();

            RobotInfo {
                pos: Position {
                    x: coords[0].0,
                    y: coords[0].1,
                },
                v_x: coords[1].0,
                v_y: coords[1].1,
            }
        })
        .collect::<Vec<_>>()
}

fn calc_position(robot_info: &RobotInfo, round_num: i32, floor: &Floor) -> Position {
    let x = (robot_info.pos.x + robot_info.v_x * round_num).rem_euclid(floor.width);
    let y = (robot_info.pos.y + robot_info.v_y * round_num).rem_euclid(floor.height);

    Position { x, y }
}

fn restroom_redoubt_part_1(filename: &str, floor: &Floor) -> i32 {
    let inputs = read_input(filename);

    let total_rounds = 100;
    let half_width = (floor.width - 1) / 2;
    let half_height = (floor.height - 1) / 2;

    let positions = inputs
        .iter()
        .map(|robot_info| calc_position(robot_info, total_rounds, floor))
        .map(|position| {
            if position.x < half_width && position.y < half_height {
                Some(Quadrant::I)
            } else if position.x > half_width && position.y < half_height {
                Some(Quadrant::II)
            } else if position.x < half_width && position.y > half_height {
                Some(Quadrant::III)
            } else if position.x > half_width && position.y > half_height {
                Some(Quadrant::IV)
            } else {
                None
            }
        })
        .filter_map(|quadrant| quadrant)
        .fold((0, 0, 0, 0), |acc, quadrant| match quadrant {
            Quadrant::I => (acc.0 + 1, acc.1, acc.2, acc.3),
            Quadrant::II => (acc.0, acc.1 + 1, acc.2, acc.3),
            Quadrant::III => (acc.0, acc.1, acc.2 + 1, acc.3),
            Quadrant::IV => (acc.0, acc.1, acc.2, acc.3 + 1),
        });

    positions.0 * positions.1 * positions.2 * positions.3
}

fn restroom_redoubt_part_2(filename: &str, floor: &Floor) -> usize {
    let inputs = read_input(filename);

    let mut round_num = 1usize;
    loop {
        let positions = inputs
            .iter()
            .map(|robot_info| calc_position(robot_info, round_num as i32, floor))
            .collect::<Vec<_>>();

        for pos in positions.iter() {
            let pos_to_find = vec![
                // row y + 1
                Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
                Position {
                    x: pos.x - 1,
                    y: pos.y + 1,
                },
                Position {
                    x: pos.x,
                    y: pos.y + 1,
                },
                Position {
                    x: pos.x + 1,
                    y: pos.y + 1,
                },
                // row y + 2
                Position {
                    x: pos.x + 2,
                    y: pos.y + 2,
                },
                Position {
                    x: pos.x + 1,
                    y: pos.y + 2,
                },
                Position {
                    x: pos.x,
                    y: pos.y + 2,
                },
                Position {
                    x: pos.x - 1,
                    y: pos.y + 2,
                },
                Position {
                    x: pos.x - 2,
                    y: pos.y + 2,
                },
            ];

            if pos_to_find.iter().all(|p| positions.contains(p)) {
                print_positions(positions.as_slice(), floor);
                return round_num;
            }
        }

        round_num += 1;
    }
}

fn print_positions(positions: &[Position], floor: &Floor) {
    let mut grid = vec![vec!['.'; floor.width as usize]; floor.height as usize];
    for pos in positions {
        grid[pos.y as usize][pos.x as usize] = 'O';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn is_possible_christmas_tree_shape_3_with_rescale(
    positions: &Vec<Position>,
    _floor: &Floor,
) -> bool {
    let x_min_max = positions.iter().map(|pos| pos.x).minmax();
    let y_min_max = positions.iter().map(|pos| pos.y).minmax();
    match x_min_max {
        MinMaxResult::MinMax(x_min, x_max) => match y_min_max {
            MinMaxResult::MinMax(y_min, y_max) => {
                let rescaled_positions = positions
                    .iter()
                    .map(|pos| Position {
                        x: pos.x - x_min,
                        y: pos.y - y_min,
                    })
                    .collect::<Vec<_>>();

                is_possible_christmas_tree_shape_3(
                    &rescaled_positions,
                    &Floor {
                        width: x_max - x_min + 1,
                        height: y_max - y_min + 1,
                    },
                )
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn is_possible_christmas_tree_shape_3(positions: &Vec<Position>, floor: &Floor) -> bool {
    for pos in positions.iter() {
        let mirror_pos = Position {
            x: floor.width - 1 - pos.x,
            y: pos.y,
        };

        if !positions.contains(&mirror_pos) {
            return false;
        }
    }

    true
}

fn is_close(pos: &Position, other_pos: &Position) -> bool {
    (pos.y - other_pos.y).abs() <= 1 && (pos.x - other_pos.x).abs() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_FLOOR: Floor = Floor {
        width: 101,
        height: 103,
    };

    const INPUT_EXAMPLE_FLOOR: Floor = Floor {
        width: 11,
        height: 7,
    };

    #[test]
    fn calc_position_test() {
        let answer = calc_position(
            &RobotInfo {
                pos: Position { x: 2, y: 4 },
                v_x: 2,
                v_y: -3,
            },
            5,
            &INPUT_EXAMPLE_FLOOR,
        );

        assert_eq!(answer, Position { x: 1, y: 3 });
    }

    #[test]
    fn is_possible_christmas_tree_shape_3_test() {
        let positions = vec![
            Position { x: 5, y: 0 },
            Position { x: 4, y: 1 },
            Position { x: 6, y: 1 },
            Position { x: 3, y: 2 },
            Position { x: 7, y: 2 },
            Position { x: 2, y: 3 },
            Position { x: 8, y: 3 },
            Position { x: 1, y: 4 },
            Position { x: 9, y: 4 },
            Position { x: 0, y: 5 },
            Position { x: 10, y: 5 },
            Position { x: 5, y: 6 },
            // Position { x: 4, y: 0 },
        ];

        let answer = is_possible_christmas_tree_shape_3(&positions, &INPUT_EXAMPLE_FLOOR);
        assert_eq!(answer, true);
    }

    #[test]
    fn is_possible_christmas_tree_shape_3_with_rescale_test() {
        let positions = vec![
            Position { x: 2, y: 1 },
            Position { x: 1, y: 2 },
            Position { x: 2, y: 2 },
            Position { x: 3, y: 2 },
            Position { x: 2, y: 3 },
            Position { x: 1, y: 4 },
            Position { x: 3, y: 4 },
        ];

        let answer =
            is_possible_christmas_tree_shape_3_with_rescale(&positions, &INPUT_EXAMPLE_FLOOR);
        assert_eq!(answer, true);
    }

    #[test]
    fn part_1_example_input() {
        let answer = restroom_redoubt_part_1("inputs/14_input_example.txt", &INPUT_EXAMPLE_FLOOR);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 12);
    }

    #[test]
    fn part_1_input() {
        let answer = restroom_redoubt_part_1("inputs/14_input.txt", &INPUT_FLOOR);

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 228457125);
    }

    #[test]
    fn part_2_input_example() {
        // example has no answer
        assert_eq!(true, true);
    }

    #[test]
    fn part_2_input() {
        let answer = restroom_redoubt_part_2("inputs/14_input.txt", &INPUT_FLOOR);

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 6493);
    }
}
