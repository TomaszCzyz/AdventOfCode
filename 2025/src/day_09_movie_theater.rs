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

fn area(c1: Coord, c2: Coord) -> i64 {
    let width = (c1.0 - c2.0).abs();
    let height = (c1.1 - c2.1).abs();
    (width + 1) * (height + 1)
}

fn part_2(filename: &str) -> i64 {
    let _ = read_input(filename);

    todo!()
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
