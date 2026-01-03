use itertools::Itertools;
use std::fs;

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
    _ = read_input(filename);

    todo!()
}

fn part_2(filename: &str) -> i64 {
    _ = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/10_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 7);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/10_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4761736832);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/10_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 24);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/10_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 1452422268);
    }
}
