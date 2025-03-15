use crate::plutonian_pebbles::VertexSum::Completed;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;

type VertexValues = HashMap<usize, u32>;

type AdjMatrix = Vec<Vec<usize>>;

#[derive(Clone)]
struct Node {
    value: u64,
    depth: usize,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.value, self.depth)
    }
}

enum Result {
    OneNumber(u64),
    TwoNumbers(u64, u64),
}

fn read_input(file_name: &str) -> Vec<u64> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|val| val.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn apply_rules(number: u64) -> Result {
    if number == 0 {
        Result::OneNumber(1)
    } else {
        let num_digits = number.ilog10() + 1;
        if num_digits % 2 == 0 {
            let divisor = 10u64.pow(num_digits / 2);
            let left_half = number / divisor;
            let right_half = number % divisor;
            Result::TwoNumbers(left_half, right_half)
        } else {
            Result::OneNumber(number * 2024)
        }
    }
}

fn apply_rules_2(number: u64) -> [Option<u64>; 2] {
    if number == 0 {
        [Some(1), None]
    } else {
        let num_digits = number.ilog10() + 1;
        if num_digits % 2 == 0 {
            let divisor = 10u64.pow(num_digits / 2);
            let left_half = number / divisor;
            let right_half = number % divisor;
            [Some(left_half), Some(right_half)]
        } else {
            [Some(number * 2024), None]
        }
    }
}

fn solution(numbers: Vec<u64>, rounds_count: usize) -> usize {
    let mut queue = numbers
        .iter()
        .map(|&n| (n, rounds_count))
        .collect::<Vec<_>>();

    let mut results_count = 0;

    loop {
        let (mut curr_value, rounds_num) = match queue.pop() {
            None => break,
            Some(r) => r,
        };
        for round_i in (0..rounds_num).rev() {
            let result = apply_rules(curr_value);
            curr_value = match result {
                Result::OneNumber(num) => num,
                Result::TwoNumbers(left, right) => {
                    queue.insert(0, (right, round_i));
                    left
                }
            }
        }
        results_count += 1;
    }

    results_count
}

fn plutonian_pebbles_part_1(filename: &str) -> usize {
    let numbers = read_input(filename);

    solution(numbers, 25)
}

enum VertexSum {
    Pending,
    Completed(u32),
}

fn plutonian_pebbles_part_2(filename: &str, rounds_count: usize) -> u32 {
    let numbers = read_input(filename);

    let mut adj_matrix = Vec::<Vec<usize>>::new();
    let mut vertex_values = Vec::<Node>::new();

    for number in numbers {
        vertex_values.push(Node {
            value: number,
            depth: 0,
        });
        adj_matrix.push(Vec::new())
    }

    for round_i in 1..(rounds_count + 1) {
        for (node_i, node) in vertex_values
            .clone()
            .iter()
            .enumerate()
            .filter(|(_, node)| node.depth == round_i - 1)
        {
            let results = apply_rules_2(node.value);
            for result in results.iter().filter_map(|&r| r) {
                match vertex_values.iter().position(|n| n.value == result) {
                    None => {
                        vertex_values.push(Node {
                            value: result,
                            depth: round_i,
                        });
                        adj_matrix.push(Vec::new());
                        adj_matrix[node_i].push(vertex_values.len() - 1);
                    }
                    Some(vertex_pos) => {
                        adj_matrix[node_i].push(vertex_pos);
                    }
                }
            }
        }
    }
    println!("vertex_values: {vertex_values:?}");
    for (i, v) in adj_matrix.iter().enumerate() {
        println!(
            "{}: {:?}",
            vertex_values[i].value,
            v.iter()
                .map(|j| vertex_values[*j].value)
                .collect::<Vec<_>>()
        );
    }

    let mut vertex_sums = (0..vertex_values.len())
        .map(|_| VertexSum::Pending)
        .collect::<Vec<_>>();

    let mut counter = 0;
    for (root_vertex, _) in vertex_values
        .iter()
        .enumerate()
        .filter(|(_, node)| node.depth == 0)
    {
        dfs(
            &adj_matrix,
            &vertex_values,
            &mut vertex_sums,
            rounds_count,
            &mut counter,
            root_vertex,
            0,
        );
    }
    counter
}

fn dfs(
    adj_matrix: &AdjMatrix,
    vertex_values: &Vec<Node>,
    vertex_sums: &mut Vec<VertexSum>,
    max_depth: usize,
    counter: &mut u32,
    vertex: usize,
    current_depth: usize,
) {
    println!("Visiting vertex with value {}", vertex_values[vertex].value);

    if current_depth == max_depth {
        println!(
            "Marking vertex with value {} with Completed(1)",
            vertex_values[vertex].value
        );
        vertex_sums[vertex] = Completed(1);
        *counter += 1;
        return;
    }

    for neighbor in adj_matrix[vertex].iter() {
        match vertex_sums[*neighbor] {
            Completed(val) => {
                println!(
                    "neighbor {} is completed, increasing counter by {val}",
                    vertex_values[*neighbor].value
                );
                *counter = *counter + val;
                continue;
            }
            _ => {}
        }

        dfs(
            adj_matrix,
            vertex_values,
            vertex_sums,
            max_depth,
            counter,
            *neighbor,
            current_depth + 1,
        );
    }

    let are_all_paths_completed = adj_matrix[vertex]
        .iter()
        .all(|neighbor| matches!(vertex_sums[*neighbor], Completed(_)));

    if are_all_paths_completed {
        let sum = adj_matrix[vertex]
            .iter()
            .map(|neighbor| match vertex_sums[*neighbor] {
                Completed(val) => val,
                _ => unreachable!(),
            })
            .sum::<u32>();

        println!(
            "All paths are completed for vertex {}, marking it as completed with sum {sum}",
            vertex_values[vertex].value
        );
        vertex_sums[vertex] = Completed(sum);
        return;
    }
}

fn brute_force(numbers: Vec<u64>, rounds_count: usize) -> usize {
    let mut current_values = numbers.clone();

    for round_i in 0..rounds_count {
        let mut next_values = Vec::new();
        for value in current_values {
            let result = apply_rules(value);
            match result {
                Result::OneNumber(num) => {
                    next_values.push(num);
                }
                Result::TwoNumbers(left, right) => {
                    next_values.push(left);
                    next_values.push(right);
                }
            }
        }
        current_values = next_values;

        println!(
            "round: {:3} ({}): {:?}",
            round_i + 1,
            current_values.len(),
            current_values
        );
    }

    current_values.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_test() {
        let numbers = read_input("inputs/11_input_example.txt");
        let answer = brute_force(numbers, 25);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 55312);
    }

    #[test]
    fn part_1_example_input() {
        let answer = plutonian_pebbles_part_1("inputs/11_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 55312);
    }

    #[test]
    fn part_1_input() {
        let answer = plutonian_pebbles_part_1("inputs/11_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 175006);
    }

    #[test]
    fn part_2_input_example_6_rounds() {
        let answer = plutonian_pebbles_part_2("inputs/11_input_example.txt", 6);

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 22);
    }

    // #[test]
    // fn part_2_input_example_25_rounds() {
    //     let answer = plutonian_pebbles_part_2("inputs/11_input_example.txt", 25);
    //
    //     println!("part 2 - original - answer: {:?}", answer);
    //     assert_eq!(answer, 55312);
    // }
    //
    // #[test]
    // fn part_2_input_75_rounds() {
    //     let answer = plutonian_pebbles_part_2("inputs/11_input.txt", 45);
    //
    //     println!("part 2 - original - answer: {:?}", answer);
    //     assert_eq!(answer, 1192);
    // }
}
