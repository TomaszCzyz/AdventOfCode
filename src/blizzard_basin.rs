use std::collections::{HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs;
use std::hash::Hash;
use std::ops::Add;

use itertools::{Itertools, MinMaxResult};

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

pub fn blizzard_basin_part_1(filename: &str) -> usize {
    todo!()
}

pub fn blizzard_basin_part_2(filename: &str) -> usize {
    todo!()
}