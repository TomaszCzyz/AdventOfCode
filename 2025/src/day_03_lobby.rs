use itertools::Itertools;
use std::fs;

type BatteryBank = Vec<u64>;

fn read_input(file_name: &str) -> Vec<BatteryBank> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_1(filename: &str) -> u64 {
    let battery_banks = read_input(filename);

    battery_banks.iter().map(analyze_bank).sum()
}

fn part_2(filename: &str) -> u64 {
    let battery_banks = read_input(filename);

    battery_banks
        .iter()
        .map(|b| {
            let result = analyze_bank_2(b);
            println!("{:?}", result);
            println!();
            result
        })
        .sum()
}

fn analyze_bank_2(bank: &BatteryBank) -> u64 {
    let mut current_bests: [u64; 12] = bank[..12].try_into().unwrap();

    for joltage in bank[12..].iter() {
        let min_pos = current_bests
            .windows(2)
            .position(|slice| slice[0] <= *joltage && slice[0] <= slice[1]);

        if let Some(min_pos) = min_pos {
            let min_value = current_bests[min_pos];
            if min_value <= *joltage {
                // shift elements left form specified index
                for i in min_pos..(current_bests.len() - 1) {
                    current_bests[i] = current_bests[i + 1];
                }

                current_bests[current_bests.len() - 1] = *joltage;
            }
        }
    }

    to_number(current_bests)
}

fn to_number(arr: [u64; 12]) -> u64 {
    let mut result = 0;
    for i in (0..12u32).rev() {
        result = result + 10u64.pow(i) * arr[11 - i as usize];
    }
    result
}

fn analyze_bank(bank: &BatteryBank) -> u64 {
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

fn calc_joltage(first: u64, second: u64) -> u64 {
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
    fn to_number_test() {
        let answer = to_number([8, 8, 8, 9, 1, 1, 1, 1, 2, 1, 1, 1]);

        assert_eq!(answer, 888911112111);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/03_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 3121910778619);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/03_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        // 163592593037764 <- too low
        assert_eq!(answer, 0);
    }
}
