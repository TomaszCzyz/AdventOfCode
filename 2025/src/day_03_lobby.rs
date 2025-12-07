use itertools::Itertools;
use std::fs;

type BatteryBank = Vec<u32>;

fn read_input(file_name: &str) -> Vec<BatteryBank> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_1(filename: &str) -> u32 {
    let battery_banks = read_input(filename);

    battery_banks.iter().map(analyze_bank).sum()
}

fn part_2(filename: &str) -> usize {
    let _ = read_input(filename);

    todo!()
}

fn analyze_bank(bank: &BatteryBank) -> u32 {
    let max_pos = bank.len() - 1 - bank.iter().rev().position_max().unwrap();
    if max_pos != bank.len() - 1 {
        let second_max_pos = bank[(max_pos + 1)..].iter().position_max().unwrap();
        calc_joltage(bank[max_pos], bank[max_pos + 1 + second_max_pos])
    } else {
        let second_max_pos = bank[..max_pos].iter().position_max().unwrap();
        calc_joltage(bank[second_max_pos], bank[max_pos])
    }
}

fn print_bank(bank: &BatteryBank, first_index: usize, second_index: usize, index: usize) {
    for i in 0..bank.len() {
        if i == first_index {
            print!("{:?}", bank[i]);
        } else if i == second_index {
            print!("{:?}", bank[i]);
        } else if i == index {
            print!("{:?}", bank[i]);
        } else {
            print!("_");
        }
    }
    println!();
}

fn calc_joltage(first: u32, second: u32) -> u32 {
    first * 10 + second
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/03_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 357);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = part_1("inputs/03_input_example_2.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 98);
    }

    #[test]
    fn part_1_input_example_3() {
        let answer = part_1("inputs/03_input_example_3.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 77);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/03_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 17324);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/03_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 43);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/03_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 9397);
    }
}
