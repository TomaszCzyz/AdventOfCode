use std::fs;

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

fn read_input(file_name: &str) -> (Vec<Vec<i64>>, Vec<Operation>) {
    let file_content = fs::read_to_string(file_name).unwrap();

    let mut lines = Vec::new();
    for line in file_content.lines() {
        let first_chat = line.chars().next().unwrap();
        if first_chat.is_digit(10) || first_chat.is_whitespace() {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            lines.push(numbers);
        } else {
            let operations = line
                .split_whitespace()
                .map(|s| match s {
                    "+" => Operation::Add,
                    "*" => Operation::Multiply,
                    _ => panic!("unknown operation: {}", s),
                })
                .collect::<Vec<_>>();

            return (lines, operations);
        }
    }

    unreachable!();
}

fn part_1(filename: &str) -> i64 {
    let (numbers_lines, operations) = read_input(filename);
    let numbers_lines = transpose(numbers_lines);

    let mut total = 0 as i64;
    for (i, op) in operations.iter().enumerate() {
        total += match op {
            Operation::Add => numbers_lines[i].iter().sum::<i64>(),
            Operation::Multiply => numbers_lines[i].iter().product(),
        }
    }

    total
}

fn part_2(filename: &str) -> usize {
    let _ = read_input(filename);

    todo!()
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_example_input() {
        let (numbers_lines, operations) = read_input("inputs/06_input_example_1.txt");

        println!("numbers_lines: {:?}", transpose(numbers_lines));
        println!("operations: {:?}", operations);
    }

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/06_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4277556);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/06_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4309240495780);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/06_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 14);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/06_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 353507173555373);
    }
}
