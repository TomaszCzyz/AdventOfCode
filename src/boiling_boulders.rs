#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

fn initialize_mappings(points: Vec<Point>) -> [HashMap<(u32, u32), Vec<u32>>; 3] {
    let mut xy_to_z_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut xz_to_y_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();
    let mut yz_to_x_map: HashMap<(u32, u32), Vec<u32>> = HashMap::new();

    for p in points.iter() {
        xy_to_z_map.entry((p.x, p.y)).or_insert_with(Vec::new).push(p.z);
        xz_to_y_map.entry((p.x, p.z)).or_insert_with(Vec::new).push(p.y);
        yz_to_x_map.entry((p.y, p.z)).or_insert_with(Vec::new).push(p.x);
    }
    [xy_to_z_map, xz_to_y_map, yz_to_x_map]
}

pub fn boiling_boulders_part_1(file_name: &str) -> usize {
    let points = read_input(file_name);
    let mappings = initialize_mappings(points);

    let mut visible_sides = 0_usize;
    for map in mappings.iter() {
        let mut visible_sides_per_plane = 0_usize;
        for (coords, cubes_indexes) in map.iter() {
            let mut indexes = cubes_indexes.clone();
            indexes.sort();

            println!("{:?}: {:?}", coords, indexes);

            let mut visible_counter = 2_usize;

            if indexes.len() != 1 {
                for consecutive_indexes in indexes.windows(2) {
                    if consecutive_indexes[1] != consecutive_indexes[0] + 1 {
                        visible_counter += 2;
                    }
                }
            }

            visible_sides_per_plane += visible_counter;
        }

        println!("visible sides {:?}", visible_sides_per_plane);
        visible_sides += visible_sides_per_plane;
    }

    visible_sides
}

pub fn boiling_boulders_part_2(file_name: &str) -> usize {
    let points = read_input(file_name);
    let mappings = initialize_mappings(points);

    let mut visible_sides = 0_usize;
    let mut gaps = Vec::new();
    for (plane_number, map) in mappings.iter().enumerate() {
        let mut visible_sides_per_plane = 0_usize;
        let mut gaps_per_plane = HashSet::new();

        for (coords, cubes_indexes) in map.iter() {
            let mut indexes = cubes_indexes.clone();
            indexes.sort();

            println!("{:?}: {:?}", coords, indexes);

            let mut visible_counter = 2_usize;
            for consecutive_indexes in indexes.windows(2) {
                let first = consecutive_indexes[0];
                let second = consecutive_indexes[1];

                if second == first + 1 {
                    continue;
                }

                visible_counter += 2;

                for i in first + 1..second {
                    gaps_per_plane.insert(match plane_number {
                        0 => Point { x: coords.0, y: coords.1, z: i },
                        1 => Point { x: coords.0, y: i, z: coords.1 },
                        2 => Point { x: i, y: coords.0, z: coords.1 },
                        _ => panic!()
                    });
                }
            }

            visible_sides_per_plane += visible_counter;
        }

        println!("gaps {:?}", gaps_per_plane);
        println!("visible sides {:?}", visible_sides_per_plane);

        visible_sides += visible_sides_per_plane;
        gaps.push(gaps_per_plane);
    }

    let gaps_intersection = gaps.iter()
        .skip(1)
        .fold(HashSet::from_iter(gaps[0].clone()), |acc, set| &acc & set);

    println!("gaps_intersection {gaps_intersection:?}");

    // find all distinct droplets
    // do part_1 of the solution for each
    // subtract results from initial sum

    visible_sides
}