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
    let mut current_best = 0;
    // println!("coords: {:?}", coords);
    println!("coords len: {:?}", coords.len());

    let mut count = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            count += 1;

            let x_min = coords[i].0.min(coords[j].0);
            let x_max = coords[i].0.max(coords[j].0);
            let y_min = coords[i].1.min(coords[j].1);
            let y_max = coords[i].1.max(coords[j].1);

            let mut is_valid = true;
            for k in 0..coords.len() {
                if k == i || k == j {
                    continue;
                }

                let (x, y) = coords[k];
                if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
                    if x == x_min
                        && (!point_in_polygon((x_min + 1, y + 1), &coords)
                            || !point_in_polygon((x_min + 1, y - 1), &coords))
                    {
                        is_valid = false;
                        break;
                    }

                    if x == x_max
                        && (!point_in_polygon((x_max - 1, y + 1), &coords)
                            || !point_in_polygon((x_max - 1, y - 1), &coords))
                    {
                        is_valid = false;
                        break;
                    }

                    if y == y_min
                        && (!point_in_polygon((x - 1, y_min + 1), &coords)
                            || !point_in_polygon((x + 1, y_min + 1), &coords))
                    {
                        is_valid = false;
                        break;
                    }

                    if y == y_max
                        && (!point_in_polygon((x - 1, y_max - 1), &coords)
                            || !point_in_polygon((x + 1, y_max - 1), &coords))
                    {
                        is_valid = false;
                        break;
                    }

                    if !(x == x_min || x == x_max || y == y_min || y == y_max) {
                        // inside polygon
                        is_valid = false;
                        break;
                    }
                }
            }

            if !is_valid {
                continue;
            }
            print_map_with_area(&coords, coords[i], coords[j]);
            let area = area(coords[i], coords[j]);
            if area > current_best {
                current_best = area;
            }
        }
    }

    current_best
}

/// Returns true if point `p` lies strictly inside `polygon`.
/// Polygon must be simple (non-self-intersecting).
pub fn point_in_polygon(p: Coord, polygon: &[Coord]) -> bool {
    let (px, py) = p;
    let mut winding: i32 = 0;
    let n = polygon.len();

    for i in 0..n {
        let (x1, y1) = polygon[i];
        let (x2, y2) = polygon[(i + 1) % n];

        if on_segment(p, polygon[i], polygon[(i + 1) % n]) {
            return true;
        }

        // Translate vertices relative to p
        let a = (x1 - px, y1 - py);
        let b = (x2 - px, y2 - py);

        let q1 = quadrant(a.0, a.1);
        let q2 = quadrant(b.0, b.1);
        let dq = q2 - q1;

        match dq {
            1 | -3 => winding += 1,
            -1 | 3 => winding -= 1,
            2 | -2 => {
                let c = cross(a, b);
                if c > 0 {
                    winding += 2;
                } else if c < 0 {
                    winding -= 2;
                }
            }
            _ => {}
        }
    }

    winding != 0
}

fn cross(a: Coord, b: Coord) -> i64 {
    a.0 * b.1 - a.1 * b.0
}

fn on_segment(p: Coord, a: Coord, b: Coord) -> bool {
    let (px, py) = p;
    let (ax, ay) = a;
    let (bx, by) = b;

    let cross = (bx - ax) as i128 * (py - ay) as i128 - (by - ay) as i128 * (px - ax) as i128;
    if cross != 0 {
        return false;
    }

    let dot = (px - ax) as i128 * (px - bx) as i128 + (py - ay) as i128 * (py - by) as i128;
    dot <= 0
}

fn quadrant(x: i64, y: i64) -> i32 {
    if x >= 0 {
        if y >= 0 {
            0
        } else {
            3
        }
    } else {
        if y >= 0 {
            1
        } else {
            2
        }
    }
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
        assert_eq!(answer, 8759985540);
    }
}
