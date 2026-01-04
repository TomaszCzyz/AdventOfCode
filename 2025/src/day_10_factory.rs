use nalgebra::DMatrix;
use std::collections::VecDeque;
use std::fs;

type ButtonMask = Vec<u8>;

const EPS: f64 = 1e-6;

#[derive(Debug)]
struct Instruction {
    desired_state: Vec<u8>,
    buttons: Vec<ButtonMask>,
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
                    '.' => 0u8,
                    '#' => 1u8,
                    _ => panic!("unexpected char"),
                })
                .collect::<Vec<_>>();

            let joltage_requirements = segments[segments.len() - 1]
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            let buttons = segments[1..segments.len() - 1]
                .iter()
                .map(|&s| {
                    let mut mask = vec![0u8; desired_state.len()];
                    s.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|s| s.parse::<usize>().unwrap())
                        .for_each(|c| mask[c] = 1);

                    mask
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

fn part_1(filename: &str) -> usize {
    let input = read_input(filename);

    input
        .into_iter()
        .skip(2)
        .map(|instruction| dbg!(find_lowest_solution(instruction)))
        .sum()
}

fn find_lowest_solution(instruction: Instruction) -> usize {
    let mut states_queue = VecDeque::from([instruction.desired_state.clone()]);

    while let Some(state) = states_queue.pop_front() {
        match solve_for_state(&instruction.buttons, &state) {
            Some(solution) => return solution,
            None => {
                for i in 0..state.len() {
                    let mut new_state = state.clone();
                    new_state[i] += 2;
                    states_queue.push_back(new_state);
                }
            }
        }
    }

    panic!("no solution found")
}

fn solve_for_state(button_masks: &Vec<ButtonMask>, light_state: &Vec<u8>) -> Option<usize> {
    let m = buttons_to_matrix(button_masks);

    let m_inverse = m
        .svd(true, true)
        .pseudo_inverse(EPS)
        .expect("failed to compute pseudo-inverse");

    let state = DMatrix::from_row_slice(
        light_state.len(),
        1,
        &light_state.iter().map(|&v| v as f64).collect::<Vec<_>>(),
    );
    let solution = m_inverse * state;

    if solution.iter().all(|f| {
        let x = f.fract();
        x < EPS || x > 1f64 - EPS
    }) {
        Some(solution.iter().map(|f| f.round() as usize).sum())
    } else {
        None
    }
}

fn buttons_to_matrix(buttons: &[ButtonMask]) -> DMatrix<f64> {
    let cols = buttons.len();
    let rows = buttons[0].len();

    let data: Vec<f64> = (0..cols)
        .flat_map(|c| (0..rows).map(move |r| buttons[c][r] as f64))
        .collect();

    DMatrix::from_column_slice(rows, cols, &data)
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
