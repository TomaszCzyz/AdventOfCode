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

fn initialize_mappings(points: &[Point]) -> [HashMap<(u32, u32), Vec<u32>>; 3] {
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

fn calculate_sides_and_gaps(mappings: &[HashMap<(u32, u32), Vec<u32>>; 3]) -> (usize, Vec<HashSet<Point>>) {
    let mut visible_sides = 0_usize;
    let mut gaps = Vec::new();
    for (plane_number, map) in mappings.iter().enumerate() {
        let mut visible_sides_per_plane = 0_usize;
        let mut gaps_per_plane = HashSet::new();

        print_plane(map);

        for (coords, cubes_indexes) in map.iter() {
            let mut indexes = cubes_indexes.clone();
            indexes.sort();

            println!("{:?}: {:?}", coords, indexes);

            let mut visible_counter = 2_usize;
            for consecutive_indexes in indexes.windows(2) {
                let (first, second) = (consecutive_indexes[0], consecutive_indexes[1]);

                if second - first == 1 {
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

        println!("gaps_per_plane {:?}", gaps_per_plane);
        println!("sides per plane {:?}", visible_sides_per_plane);

        visible_sides += visible_sides_per_plane;
        gaps.push(gaps_per_plane);
    }
    (visible_sides, gaps)
}

fn print_plane(plane: &HashMap<(u32, u32), Vec<u32>>) {
    let layers_number = *plane.values().flatten().max().unwrap_or(&0);

    let max_i = *plane.keys().map(|(i, _j)| i).max().unwrap_or(&1);
    let max_j = *plane.keys().map(|(_i, j)| j).max().unwrap_or(&1);

    for layer_number in 1..=layers_number {
        for i in 1..=max_i {
            for j in 1..=max_j {
                if plane.contains_key(&(i, j)) && plane.get(&(i, j)).unwrap().contains(&layer_number) {
                    print!("#")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
        println!("{}", "-".repeat(max_j as usize))
    }
}

pub fn boiling_boulders_part_1(file_name: &str) -> usize {
    let points = read_input(file_name);
    let mappings = initialize_mappings(&points);

    let (total_sides, _gaps) = calculate_sides_and_gaps(&mappings);
    total_sides
}

pub fn boiling_boulders_part_2(file_name: &str) -> usize {
    let points = read_input(file_name);
    let mappings = initialize_mappings(&points);

    let (total_sides, gaps) = calculate_sides_and_gaps(&mappings);

    println!("total sides: {total_sides:?}");

    let mut gaps_intersection = gaps.iter()
        .skip(1)
        .fold(HashSet::from_iter(gaps[0].clone()), |acc, set| &acc & set)
        .into_iter()
        .collect::<Vec<_>>();

    println!("gaps_intersection {gaps_intersection:?}");

    remove_false_positive_gaps(&points, &mut gaps_intersection);

    let gaps_mappings = initialize_mappings(&gaps_intersection);
    let (gaps_sides, _) = calculate_sides_and_gaps(&gaps_mappings);

    println!("sides for gaps mappings: {:?}", gaps_sides);

    total_sides - gaps_sides
}


/// Each gap point must be adjacent to lava point or to other gap point
/// # Arguments
/// * `lava_mapping`:
/// * `gaps_mapping`:
/// returns: ()
fn remove_false_positive_gaps(lava_points: &[Point], gaps_points: &mut Vec<Point>) {
    let mut points_to_remove = Vec::new();

    for gap_point in gaps_points.iter() {
        let (x, y, z) = (gap_point.x, gap_point.y, gap_point.z);

        let up = Point { x: gap_point.x + 1, y, z };
        let down = Point { x: gap_point.x - 1, y, z };
        let left = Point { x, y: gap_point.y + 1, z };
        let right = Point { x, y: gap_point.y - 1, z };
        let forward = Point { x, y, z: gap_point.z + 1 };
        let backward = Point { x, y, z: gap_point.z - 1 };

        let neighbor_points = [up, down, left, right, forward, backward];

        for point in neighbor_points {
            if lava_points.contains(&point) || gaps_points.contains(&point) {
                continue;
            }
            points_to_remove.push(gap_point.clone());
            break;
        }
    }

    for p in points_to_remove.into_iter() {
        println!("removing point {:?}", p);
        let index = gaps_points.iter().position(|x| x == &p).unwrap();
        gaps_points.swap_remove(index);
    }
}
