use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

type Sensors = Vec<Point>;
type Beacons = Vec<Point>;

fn parse_point(s: &str) -> Point {
    let (x, y) = s.split(',')
        .map(|ss| ss.trim())
        .map(|ss| ss.trim_start_matches("x="))
        .map(|ss| ss.trim_start_matches("y="))
        .map(|ss| ss.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    Point { x, y }
}

fn read_input(file_name: &str) -> (Sensors, Beacons) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut sensors = Vec::new();
    let mut beacons = Vec::new();

    loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            }

            let (sensor_point, beacon_point) = buf.trim()
                .trim_start_matches("Sensor at ")
                .split(": closest beacon is at ")
                .map(parse_point)
                .collect_tuple()
                .unwrap();

            sensors.push(sensor_point);
            beacons.push(beacon_point)
        }
    }

    (sensors, beacons)
}

fn find_edges_of_circles(circles: &Vec<(Point, usize)>) -> (i32, i32) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;

    for (center, radius) in circles {
        let left_most = center.x - *radius as i32;
        let right_most = center.x + *radius as i32;

        if left_most < x_min {
            x_min = left_most
        }
        if right_most > x_max {
            x_max = right_most
        }
    }

    (x_min, x_max)
}

fn calculate_radiuses(centers: &[Point], edge_points: &[Point]) -> Vec<(Point, usize)> {
    let mut results = Vec::new();

    for (index, center) in centers.iter().enumerate() {
        let radius = center.distance(&edge_points[index]);
        results.push((*center, radius))
    }

    results
}

pub fn beacon_exclusion_zone_part_1(file_name: &str, row_number: i32) -> i32 {
    let (sensors, beacons) = read_input(file_name);

    let circles = calculate_radiuses(&sensors, &beacons);
    println!("{sensors:?}");
    println!("{beacons:?}");

    let (x_min, x_max) = find_edges_of_circles(&circles);
    println!("x_min: {}\t\t x_max: {}", x_min, x_max);

    let mut sum = 0;

    let y = row_number;
    for x in x_min..=x_max {
        let curr_point = Point { x, y };

        let mut is_accessible = true;
        for (index, (center, radius)) in circles.iter().enumerate() {
            if curr_point.distance(&sensors[index]) == 0 || curr_point.distance(&beacons[index]) == 0 {
                break;
            }
            if curr_point.distance(center) <= *radius {
                is_accessible = false;
                break;
            }
        }

        if !is_accessible {
            sum += 1;
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();

    sum
}

pub fn beacon_exclusion_zone_part_2(file_name: &str, square_width: i32) -> u64 {
    let (sensors, beacons) = read_input(file_name);

    let circles = calculate_radiuses(&sensors, &beacons);

    let mut tuning_frequency = 0_u64;
    let mut curr_point = Point { x: 0, y: 0 };

    let total = square_width as u64 * square_width as u64;

    'outer: for _ in 0..=total {
        if curr_point.x > square_width {
            curr_point.x = 0;
            curr_point.y += 1;
        } else {
            curr_point.x += 1;
        }

        for (center, radius) in circles.iter() {
            if curr_point.distance(center) <= *radius {
                // jump to edge of circle in the same row
                curr_point.x = center.x + *radius as i32 - (curr_point.y - center.y).abs();
                continue 'outer;
            }
        }

        println!("not in circle: {:?}", curr_point);

        assert!(curr_point.x >= 0);
        assert!(curr_point.y >= 0);

        tuning_frequency = curr_point.x as u64 * 4000000 + curr_point.y as u64;
        break 'outer;
    }
    println!();

    tuning_frequency
}
