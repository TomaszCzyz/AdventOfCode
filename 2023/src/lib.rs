use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(file_name: &str) -> Vec<u32> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut numbers = Vec::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut iter = buf.chars();
        let mut first_digit = u32::MAX;
        let mut last_digit = u32::MAX;

        while let Some(ch) = iter.next() {
            if ch.is_digit(10) {
                first_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        while let Some(ch) = iter.next_back() {
            if ch.is_digit(10) {
                last_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        if last_digit == u32::MAX {
            last_digit = first_digit;
        }

        numbers.push(first_digit * 10 + last_digit);
        buf = String::new();
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_example() {
        let input = read_input("inputs/1_input_example.txt");
        let answer = input.iter().sum::<u32>();

        assert_eq!(answer, 142);
    }

    #[test]
    fn input() {
        let input = read_input("inputs/1_input.txt");
        let answer = input.iter().sum::<u32>();

        assert_eq!(answer, 54081);
    }
}
