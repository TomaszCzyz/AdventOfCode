use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

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

fn part_2(filename: &str) -> i64 {
    let coords = read_input(filename);
    println!("coords: {:?}", coords);

    let coords_all = coords
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .flat_map(|(c1, c2)| {
            if c1.0 == c2.0 {
                (c1.1.min(c2.1)..=c1.1.max(c2.1))
                    .map(|a| (c1.0, a))
                    .collect::<Vec<_>>()
            } else {
                (c1.0.min(c2.0)..=c1.0.max(c2.0))
                    .map(|a| (a, c1.1))
                    .collect::<Vec<_>>()
            }
        })
        .collect::<HashSet<_>>();

    let mut current_best = 0;
    for i in 0..coords.len() {
        let ci = coords[i];
        for j in (i + 1)..coords.len() {
            let cj = coords[j];
            let x_min = ci.0.min(cj.0);
            let x_max = ci.0.max(cj.0);
            let y_min = ci.1.min(cj.1);
            let y_max = ci.1.max(cj.1);

            let is_valid = 'rectangle_check: {
                // fast fail path
                for pk in coords.iter() {
                    if is_inside_area_edge_exclusive(*pk, x_min, x_max, y_min, y_max) {
                        break 'rectangle_check false;
                    }
                }

                // accurate checks
                for x in (x_min + 1)..(x_max - 1) {
                    if coords_all.contains(&(x, y_min + 1)) || coords_all.contains(&(x, y_max - 1))
                    {
                        break 'rectangle_check false;
                    }
                }

                for y in (y_min + 1)..(y_max - 1) {
                    if coords_all.contains(&(x_min + 1, y)) || coords_all.contains(&(x_max - 1, y))
                    {
                        break 'rectangle_check false;
                    }
                }

                true
            };

            if !is_valid {
                continue;
            }

            // print_map_with_area(&coords, ci, cj);
            let a = area(ci, cj);
            if a > current_best {
                current_best = a;
            }
        }
    }

    current_best
}

fn is_inside_area_edge_exclusive(p: Coord, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
    p.0 > x_min && p.0 < x_max && p.1 > y_min && p.1 < y_max
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
        assert_eq!(answer, 24);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/09_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 1452422268);
    }
}
