use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs;

type VertexValues = HashMap<usize, u32>;

type AdjMatrix = Vec<Vec<usize>>;

type MatrixGraph = Vec<Vec<bool>>;

fn read_input(file_name: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut flower_types = Vec::<char>::new();
    let mut matrix_graph = Vec::<Vec<usize>>::new();

    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(row_i, line)| {
            let mut row = Vec::new();
            line.chars().enumerate().for_each(|(col_i, ch)| {
                match flower_types.iter().position(|x| *x == ch) {
                    Some(pos) => {
                        row.push(pos);
                    }
                    None => {
                        flower_types.push(ch);
                        row.push(flower_types.len() - 1);
                    }
                }
            });
            matrix_graph.push(row);
        });

    (matrix_graph, flower_types)
}

fn garden_groups_part_1(filename: &str) -> usize {
    let numbers = read_input(filename);

    todo!()
}

fn garden_groups_part_2(filename: &str, rounds_count: usize) -> usize {
    let numbers = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input_1() {
        let answer = garden_groups_part_1("inputs/12_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 140);
    }

    #[test]
    fn part_1_example_input_2() {
        let answer = garden_groups_part_1("inputs/12_input_2.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 772);
    }

    #[test]
    fn part_1_example_input_3() {
        let answer = garden_groups_part_1("inputs/12_input_3.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 1930);
    }

    #[test]
    fn part_1_input() {
        let answer = garden_groups_part_1("inputs/12_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }
}
