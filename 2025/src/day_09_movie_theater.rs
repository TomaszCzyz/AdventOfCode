use itertools::Itertools;
use std::{fs, iter};

type Coord = (i64, i64);

fn read_input(file_name: &str) -> Vec<Coord> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1])
        })
        .collect::<Vec<_>>()
}

fn part_1(filename: &str) -> i64 {
    let coords = read_input(filename);
    let mut current_best = 0;

    for i in 0..coords.len() {
        for jj in (i + 1)..(i + 1 + coords.len()) {
            let j = jj % coords.len();

            let a = area(coords[i], coords[j]);
            if a > current_best {
                current_best = a;
            }
        }
    }

    current_best
}

struct CoordInfo {
    coord: Coord,
    prev_green: Coord,
    next_green: Coord,
}

fn move_one_step(from: Coord, to: Coord) -> Coord {
    if from.0 < to.0 {
        (from.0 + 1, from.1)
    } else if from.0 > to.0 {
        (from.0 - 1, from.1)
    } else if from.1 < to.1 {
        (from.0, from.1 + 1)
    } else if from.1 > to.1 {
        (from.0, from.1 - 1)
    } else {
        panic!("from and to are the same: {:?}", from);
    }
}

fn part_2(filename: &str) -> i64 {
    let coords = read_input(filename);
    println!("coords: {:?}", coords);

    let mut coords_info: Vec<CoordInfo> = Vec::new();

    let coords_wrapped = iter::once(coords[coords.len() - 1])
        .chain(coords.clone())
        .chain(iter::once(coords[0]));

    for (prev, curr, next) in coords_wrapped.tuple_windows::<(_, _, _)>() {
        let prev_green = move_one_step(curr, prev);
        let next_green = move_one_step(curr, next);

        coords_info.push(CoordInfo {
            coord: curr,
            prev_green,
            next_green,
        });
    }

    for info in &coords_info {
        println!(
            "coord: {:?}, prev_green: {:?}, next_green: {:?}",
            info.coord, info.prev_green, info.next_green
        );
    }

    let mut current_best = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            let (ci, cj) = (coords[i], coords[j]);
            let (x_min, x_max) = if ci.0 < cj.0 {
                (ci.0, cj.0)
            } else {
                (cj.0, ci.0)
            };
            let (y_min, y_max) = if ci.1 < cj.1 {
                (ci.1, cj.1)
            } else {
                (cj.1, ci.1)
            };

            let mut is_rectangle_inside = true;
            for k in 0..coords.len() {
                if k == i || k == j {
                    continue;
                }
                let coord_info = &coords_info[k];
                let coord_k = coord_info.coord;
                let prev_green = coord_info.prev_green;
                let next_green = coord_info.next_green;
                if !is_inside_area_edge_inclusive(coord_k, x_min, x_max, y_min, y_max) {
                    continue;
                }

                // check if is on the edge
                if coord_k.0 == ci.0 || coord_k.0 == cj.0 || coord_k.1 == ci.1 || coord_k.1 == cj.1
                {
                    // on the edge required additional check of previous and next green tiles
                    if is_inside_area_edge_exclusive(prev_green, x_min, x_max, y_min, y_max)
                        || is_inside_area_edge_exclusive(next_green, x_min, x_max, y_min, y_max)
                    {
                        is_rectangle_inside = false;
                        break;
                    }
                }
            }

            println!("Checking area between {:?} and {:?}", ci, cj);
            if !is_rectangle_inside {
                continue;
            }

            print_map_with_area(&coords, ci, cj);
            let a = area(ci, cj);
            if a > current_best {
                current_best = a;
            }
        }
    }

    current_best
}

fn is_inside_area_edge_inclusive(p: Coord, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
    (p.0 >= x_min && p.0 <= x_max) && (p.1 >= y_min && p.1 <= y_max)
}

fn is_inside_area_edge_exclusive(p: Coord, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
    p.0 > x_min && p.0 < x_max && p.1 > y_min && p.1 < y_max
}

fn is_path_valid(path: &Vec<Coord>) -> bool {
    if path.len() <= 2 {
        return true;
    }

    let start = path[0];
    let end = path[path.len() - 1];

    let (x_min, x_max) = if start.0 < end.0 {
        (start.0, end.0)
    } else {
        (end.0, start.0)
    };
    let (y_min, y_max) = if start.1 < end.1 {
        (start.1, end.1)
    } else {
        (end.1, start.1)
    };

    path.iter()
        .skip(1)
        .take(path.len() - 2)
        .all(|c| c.0 < x_min && c.0 > x_max && c.1 < y_min && c.1 > y_max)
}

fn print_map_with_area(coords: &Vec<Coord>, c1: Coord, c2: Coord) {
    let x_max = coords.iter().map(|c| c.0).max().unwrap() + 1;
    let y_max = coords.iter().map(|c| c.1).max().unwrap() + 1;
    for i in 1..y_max {
        for j in 1..x_max {
            let (x_min, x_max) = if c1.0 < c2.0 {
                (c1.0, c2.0)
            } else {
                (c2.0, c1.0)
            };
            let (y_min, y_max) = if c1.1 < c2.1 {
                (c1.1, c2.1)
            } else {
                (c2.1, c1.1)
            };

            if (j >= x_min && j <= x_max) && (i >= y_min && i <= y_max) {
                print!("0");
            } else if coords.contains(&(j, i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
    println!()
}

fn area(c1: Coord, c2: Coord) -> i64 {
    let width = (c1.0 - c2.0).abs();
    let height = (c1.1 - c2.1).abs();
    (width + 1) * (height + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/09_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 50);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/09_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4761736832);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/09_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 25272);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/09_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 8759985540);
    }
}
