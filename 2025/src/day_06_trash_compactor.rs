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
            let operations = parse_operations(line);
            return (lines, operations);
        }
    }

    unreachable!();
}

fn read_input_2(file_name: &str) -> (Vec<Vec<u8>>, Vec<Operation>) {
    let file_content = fs::read_to_string(file_name).unwrap();

    let mut lines = Vec::new();
    for line in file_content.lines() {
        let first_char = line.chars().next().unwrap();
        if first_char.is_digit(10) || first_char.is_whitespace() {
            lines.push(line.as_bytes().to_vec());
        } else {
            let operations = parse_operations(line);
            return (lines, operations);
        }
    }

    unreachable!();
}

fn parse_operations(line: &str) -> Vec<Operation> {
    line.split_whitespace()
        .map(|s| match s {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("unknown operation: {}", s),
        })
        .collect::<Vec<_>>()
}

fn part_1(filename: &str) -> i64 {
    let (numbers_lines, operations) = read_input(filename);
    let numbers_lines = transpose(numbers_lines);

    let mut total = 0i64;
    for (i, op) in operations.iter().enumerate() {
        total += match op {
            Operation::Add => numbers_lines[i].iter().sum::<i64>(),
            Operation::Multiply => numbers_lines[i].iter().product(),
        }
    }

    total
}

fn part_2(filename: &str) -> i64 {
    let (mut rows, operations) = read_input_2(filename);

    let max_width = rows.iter().map(|a| a.len()).max().unwrap();
    for rows in rows.iter_mut() {
        rows.resize(max_width + 1, b'0');
    }

    let mut is_digit_in_column = true;
    let mut index = 0;
    let mut column_index = 0;
    let mut total = 0;

    while index < max_width {
        // do calculation for all numbers in the column (set of number separated by whitespace column)
        let mut column_numbers = Vec::new();

        while is_digit_in_column {
            is_digit_in_column = false;
            if index == max_width {
                break;
            }

            // create a number from the current column index
            let mut digits = Vec::new();
            for line in rows.iter() {
                let char = line[index];
                if char.is_ascii_digit() {
                    is_digit_in_column = true;
                    digits.push(char - b'0');
                }
            }

            if is_digit_in_column {
                let number = digits
                    .iter()
                    .fold(0i64, |acc, &digit| acc * 10 + digit as i64);

                column_numbers.push(number);
            }

            index += 1;
        }

        is_digit_in_column = true;
        let column_total = match operations[column_index] {
            Operation::Add => column_numbers.iter().sum::<i64>(),
            Operation::Multiply => column_numbers.iter().product(),
        };
        total += column_total;
        column_index += 1;
    }

    total
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
        let answer = part_2("inputs/06_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 3263827);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/06_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 9170286552289);
    }
}
