use std::fs;

#[derive(Debug)]
struct Link {
    round_number: usize,
    index: usize,
}

#[derive(Debug)]
enum Node {
    Value(u64),
    Link(Link),
}

enum Result {
    OneNumber(u64),
    TwoNumbers(u64, u64),
}

fn read_input(file_name: &str) -> Vec<u64> {
    let numbers = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .map(|val| val.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    numbers
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

fn find_value(rounds: &Vec<Vec<Node>>, number: u64) -> Node {
    for (round_i, round_nodes) in rounds.iter().enumerate() {
        let value_pos = round_nodes.iter().position(|node| match node {
            Node::Value(v) => *v == number,
            Node::Link(_) => false,
        });

        if let Some(pos) = value_pos {
            return Node::Link(Link {
                round_number: round_i,
                index: pos,
            });
        }
    }
    Node::Value(number)
}

fn plutonian_pebbles_part_2(filename: &str, rounds_count: usize) -> usize {
    let numbers = read_input(filename);

    let round_zero = numbers
        .iter()
        .map(|num| Node::Value(*num))
        .collect::<Vec<_>>();

    let mut rounds = vec![round_zero];

    for round_i in 1..rounds_count {
        let mut current_round = Vec::<Node>::new();
        for number in rounds[rounds.len() - 1].iter() {
            if let Node::Value(num) = number {
                let result = apply_rules(*num);
                match result {
                    Result::OneNumber(num) => {
                        let node = find_value(&rounds, num);
                        current_round.push(node);
                    }
                    Result::TwoNumbers(left, right) => {
                        let left_node = find_value(&rounds, left);
                        let right_node = find_value(&rounds, right);
                        current_round.push(left_node);
                        current_round.push(right_node);
                    }
                }
            }
        }
        println!(
            "round {round_i:3} ({:3}): {:?}",
            current_round.len(),
            current_round
        );
        rounds.push(current_round);
    }

    todo!()
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

        println!("round: {:3}: {:?}", round_i + 1, current_values);
    }

    current_values.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_test() {
        let numbers = read_input("inputs/11_input.txt");
        let answer = brute_force(numbers, 20);

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
    fn part_2_input() {
        let answer = plutonian_pebbles_part_2("inputs/11_input.txt", 75);

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 1192);
    }
}
