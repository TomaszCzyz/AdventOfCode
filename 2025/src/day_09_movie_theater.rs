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
        for j in 0..coords.len() {
            if i == j {
                continue;
            }

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
    let mut xs: Vec<i64> = coords.iter().map(|c| c.0).collect();
    let mut ys: Vec<i64> = coords.iter().map(|c| c.1).collect();
    xs.sort_unstable();
    ys.sort_unstable();

    let mut current_best = 0;
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if i == j {
                continue;
            }

            let c1 = coords[i];
            let c2 = coords[j];
            let (min_x, max_x) = if c1.0 < c2.0 {
                (c1.0, c2.0)
            } else {
                (c2.0, c1.0)
            };
            let (min_y, max_y) = if c1.1 < c2.1 {
                (c1.1, c2.1)
            } else {
                (c2.1, c1.1)
            };

            // check if area within does not contain any other coords
            match coords
                .iter()
                .position(|c| c.0 > min_x && c.0 < max_x && c.1 > min_y && c.1 < max_y)
            {
                Some(_) => continue,
                None => {
                    // check if inside polygon, i.e. there is coord further (or equal) in each direction for tile
                    let c3 = (c1.1, c2.0);
                    let c4 = (c2.1, c1.0);

                    let _ = coords
                        .iter()
                        .position(|c| c.0 > min_x && c.0 < max_x && c.1 > min_y && c.1 < max_y);
                }
            }

            let a = area(c1, c2);
            if a > current_best {
                current_best = a;
            }
        }
    }

    current_best
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
