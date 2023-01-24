#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

fn read_input(file_name: &str) -> Vec<Point> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut points = Vec::new();
    let mut buf = String::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let coords = buf.trim()
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let point = Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        };

        points.push(point);
        buf = String::new();
    }

    points
}

pub fn boiling_boulders_part_1(file_name: &str) -> usize {
    let points = read_input(file_name);

    let mut xy_to_z_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut xz_to_y_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut yz_to_x_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    for p in points.iter() {
        xy_to_z_map.entry((p.x, p.y)).or_insert_with(Vec::new).push(p.z);
        xz_to_y_map.entry((p.x, p.z)).or_insert_with(Vec::new).push(p.y);
        yz_to_x_map.entry((p.y, p.z)).or_insert_with(Vec::new).push(p.x);
    }

    let mut visible_sides = 0_usize;
    for map in [xy_to_z_map, xz_to_y_map, yz_to_x_map].iter() {
        let mut visible_sides_per_plane = 0_usize;
        for mut cubes_indexes in map.values().cloned() {
            cubes_indexes.sort();

            let mut visible_counter = 2_usize;

            if cubes_indexes.len() != 1 {
                for consecutive_indexes in cubes_indexes.windows(2) {
                    if consecutive_indexes[1] != consecutive_indexes[0] + 1 {
                        visible_counter += 2;
                    }
                }
            }

            visible_sides_per_plane += visible_counter;
        }

        // println!("{map:?}");
        // println!("visible sides {:?}", visible_sides_per_plane);

        visible_sides += visible_sides_per_plane;
    }

    visible_sides
}

pub fn boiling_boulders_part_2(file_name: &str) -> usize {
    0
}