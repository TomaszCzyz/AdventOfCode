use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs;
use std::hash::Hash;
use std::ops::Add;

use itertools::{Itertools, MinMaxResult};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Point {
    row: i32,
    col: i32,
}

impl Point {
    fn dist(&self, other: &Point) -> f32 {
        let v = (self.col - other.col) * (self.col - other.col) + (self.row - other.row) * (self.row - other.row);
        (v as f32).sqrt()
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,

        }
    }
}

fn read_input(file_name: &str) -> HashSet<Point> {
    fs::read_to_string(file_name)
        .unwrap()
        .split("\r\n")
        .enumerate()
        .flat_map(|(row, line)| line.chars()
            .enumerate()
            .filter(|(_col, ch)| *ch == '#')
            .map(move |(col, _ch)| Point { row: row as i32, col: col as i32 }))
        .collect::<HashSet<_>>()
}

pub fn unstable_diffusion_part_1(filename: &str) -> usize {
    let set = unstable_diffusion(filename, 10).1;

    let min_max_col = set.iter().map(|p| p.col).minmax();
    let min_max_row = set.iter().map(|p| p.row).minmax();

    if let (MinMaxResult::MinMax(min_row, max_row), MinMaxResult::MinMax(min_col, max_col)) = (min_max_row, min_max_col) {
        let width = (min_col - max_col).unsigned_abs() as usize + 1;
        let length = (min_row - max_row).unsigned_abs() as usize + 1;

        width * length - set.len()
    } else { panic!() }
}

pub fn unstable_diffusion_part_2(filename: &str) -> usize {
    unstable_diffusion(filename, usize::MAX).0
}

fn unstable_diffusion(filename: &str, rounds: usize) -> (usize, HashSet<Point>) {
    let mut set = read_input(filename);
    // println!("== Initial State ==");
    // print_input(&set);
    // println!("{set:?}");

    let mut direction_points = VecDeque::from(
        [
            [Point { row: -1, col: -1 }, Point { row: -1, col: 0 }, Point { row: -1, col: 1 }], // NW, N, NE
            [Point { row: 1, col: -1 }, Point { row: 1, col: 0 }, Point { row: 1, col: 1 }], // SW, S, SE
            [Point { row: -1, col: -1 }, Point { row: 0, col: -1 }, Point { row: 1, col: -1 }], // NW, W, SW
            [Point { row: -1, col: 1 }, Point { row: 0, col: 1 }, Point { row: 1, col: 1 }] // NE, E, SE
        ]);

    let mut far_points = VecDeque::from(
        [
            Point { row: -2, col: 0 }, // N
            Point { row: 2, col: 0 }, // S
            Point { row: 0, col: -2 }, // W
            Point { row: 0, col: 2 } // E
        ]);

    for round_no in 1..=rounds {
        let mut no_elf_moved = true;
        let set_cloned = set.clone();

        for elf_point in set_cloned.iter() {
            let neighbors = set_cloned.iter()
                .filter(|&p| p.dist(elf_point) <= 2f32)
                .collect::<HashSet<_>>();

            // check if elf can go in any direction
            let mut it = direction_points.iter().zip(far_points.iter());
            // let mut has_adjacent_elf = false;
            while let Some((points, far_point)) = it.next() {
                if neighbors.iter().any(|&&p| p == *elf_point + points[0] || p == *elf_point + points[1] || p == *elf_point + points[2]) {
                    // cannot go in this direction, there is an elf in one of three points
                    continue;
                }

                let has_adjacent_elf = direction_points.iter()
                    .flatten()
                    .unique()
                    .map(|p| *p + *elf_point)
                    .any(|p| neighbors.contains(&p));

                if !has_adjacent_elf {
                    break;
                }

                // check if there is an elf with which the current elf might have a collision
                let collision_elf_point = *elf_point + *far_point;
                let new_tile_point = *elf_point + points[1];

                // there might be collision with an elf
                if neighbors.contains(&collision_elf_point) {
                    let neighbors_of_collision_elf = set_cloned.iter()
                        .filter(|&p| p.dist(&collision_elf_point) <= 2f32)
                        .collect::<HashSet<_>>();

                    let wanted_tile = check_elf_dir(&collision_elf_point, &neighbors_of_collision_elf, &direction_points);
                    let is_collision = match wanted_tile {
                        None => false,
                        Some(p) => p == new_tile_point
                    };

                    if is_collision {
                        break;
                    }
                }

                update_elf_pos(&mut set, elf_point, new_tile_point);
                no_elf_moved = false;
                break;
            }
        }

        move_first_to_last(&mut direction_points, &mut far_points);

        if round_no % 10 == 0 {
            // println!("\n== End of Round {} ==", round_no);
            // print_input(&set);
        }

        if no_elf_moved {
            println!("no elf moved!!");
            // print_input(&set);
            return (round_no, set.clone());
        }
    }

    (rounds, set.clone())
}

fn move_first_to_last(direction_points: &mut VecDeque<[Point; 3]>, far_points: &mut VecDeque<Point>) {
    let t1 = direction_points.pop_front().unwrap();
    direction_points.push_back(t1);
    let t2 = far_points.pop_front().unwrap();
    far_points.push_back(t2);
}

fn update_elf_pos(set: &mut HashSet<Point>, current_point: &Point, new_point: Point) {
    let is_removed = set.remove(current_point);
    if !is_removed {
        panic!()
    }
    let is_inserted = set.insert(new_point);
    if !is_inserted {
        panic!()
    }
}

fn check_elf_dir(elf_point: &Point, set: &HashSet<&Point>, directions_points: &VecDeque<[Point; 3]>) -> Option<Point> {
    let has_adjacent_elf = directions_points.iter()
        .flatten()
        .unique()
        .map(|p| *p + *elf_point)
        .any(|p| set.contains(&p));

    if !has_adjacent_elf {
        return None;
    }

    for points in directions_points {
        if set.iter().any(|&&p| p == *elf_point + points[0] || p == *elf_point + points[1] || p == *elf_point + points[2]) {
            // cannot go in this direction
            continue;
        }

        return Some(*elf_point + points[1]);
    }

    panic!();
}

#[allow(dead_code)]
fn print_input(set: &HashSet<Point>) {
    let min_max_col = set.iter().map(|p| p.col).minmax();
    let min_max_row = set.iter().map(|p| p.row).minmax();

    if let (MinMaxResult::MinMax(min_row, max_row), MinMaxResult::MinMax(min_col, max_col)) = (min_max_row, min_max_col) {
        for row in min_row..=max_row {
            for col in min_col..=max_col {
                print!("{}", if set.contains(&Point { row, col }) { "#" } else { "." });
            }
            println!();
        }
    }
}

#[allow(dead_code)]
fn print_input_padded(set: &HashSet<Point>, min_row: i32, max_row: i32, min_col: i32, max_col: i32) {
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            print!("{}", if set.contains(&Point { row, col }) { "#" } else { "." });
        }
        println!();
    }
}