use std::fs;

type ButtonLights = Vec<i64>;

#[derive(Debug)]
struct Instruction {
    desired_state: Vec<bool>,
    buttons: Vec<ButtonLights>,
    joltage_requirements: Vec<i64>,
}

fn read_input(file_name: &str) -> Vec<Instruction> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let segments = line.trim().split_whitespace().collect::<Vec<_>>();

            let desired_state = segments[0]
                .trim_matches(|c| c == '[' || c == ']')
                .chars()
                .enumerate()
                .map(|(idx, c)| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("unexpected char"),
                })
                .collect();

            let joltage_requirements = segments[segments.len() - 1]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let buttons = segments[1..segments.len() - 1]
                .iter()
                .map(|&s| {
                    s.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|s| s.parse::<i64>().unwrap())
                        .collect::<ButtonLights>()
                })
                .collect::<Vec<_>>();

            Instruction {
                desired_state,
                buttons,
                joltage_requirements,
            }
        })
        .collect::<Vec<_>>()
}

fn part_1(filename: &str) -> i64 {
    let input = read_input(filename);

    println!("input: {:?}", input);

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
