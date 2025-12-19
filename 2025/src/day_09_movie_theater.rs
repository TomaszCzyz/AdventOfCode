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

    let mut current_best = 0;
    for i in 0..coords.len() {
        // for j in (i + 1)..coords.len() {
        for j in (0..coords.len()).filter(|j| *j != i) {
            let (ci, cj) = (coords[i], coords[j]);

            let mut ci_q1 = 0;
            let mut ci_q2 = 0;
            let mut ci_q3 = 0;
            let mut ci_q4 = 0;
            let mut cj_q1 = 0;
            let mut cj_q2 = 0;
            let mut cj_q3 = 0;
            let mut cj_q4 = 0;

            for k in (0..coords.len()).filter(|&k| k != i && k != j) {
                let ck = coords[k];
                if ck.0 > ci.0 && ck.1 > ci.1 {
                    ci_q1 += 1;
                } else if ck.0 < ci.0 && ck.1 > ci.1 {
                    ci_q2 += 1;
                } else if ck.0 < ci.0 && ck.1 < ci.1 {
                    ci_q3 += 1;
                } else if ck.0 > ci.0 && ck.1 < ci.1 {
                    ci_q4 += 1;
                }

                if ck.0 > cj.0 && ck.1 > cj.1 {
                    cj_q1 += 1;
                } else if ck.0 < cj.0 && ck.1 > cj.1 {
                    cj_q2 += 1;
                } else if ck.0 < cj.0 && ck.1 < cj.1 {
                    cj_q3 += 1;
                } else if ck.0 > cj.0 && ck.1 < cj.1 {
                    cj_q4 += 1;
                }
            }

            println!(
                "points in relation to ci: {} {} -> (Q1, Q2, Q3, Q4): {} {} {} {}",
                ci.0, ci.1, ci_q1, ci_q2, ci_q3, ci_q4
            );
            println!(
                "points in relation to cj: {} {} -> (Q1, Q2, Q3, Q4): {} {} {} {}",
                cj.0, cj.1, cj_q1, cj_q2, cj_q3, cj_q4
            );

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

fn is_on_edge(p: Coord, x_min: i64, x_max: i64, y_min: i64, y_max: i64) -> bool {
    p.0 == x_min || p.0 == x_max || p.1 == y_min || p.1 == y_max
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
