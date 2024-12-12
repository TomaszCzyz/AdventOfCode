use std::collections::{BinaryHeap, HashMap};
use std::fs;

pub fn read_input(file_name: &str) -> [BinaryHeap<u32>; 2] {
    let mut left_column = BinaryHeap::new();
    let mut right_column = BinaryHeap::new();

    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut parts = line.split("   ");
            left_column.push(parts.next().unwrap().parse::<u32>().unwrap());
            right_column.push(parts.last().unwrap().parse::<u32>().unwrap());
        });

    [left_column, right_column]
}

pub fn read_input_part_2(file_name: &str) -> [HashMap<u32, usize>; 2] {
    let mut left_column = HashMap::new();
    let mut right_column = HashMap::new();

    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut parts = line.split("   ");
            let left = parts.next().unwrap().parse::<u32>().unwrap();
            let right = parts.last().unwrap().parse::<u32>().unwrap();

            left_column
                .entry(left)
                .and_modify(move |x| *x += 1)
                .or_insert(1);

            right_column
                .entry(right)
                .and_modify(move |x| *x += 1)
                .or_insert(1);
        });

    [left_column, right_column]
}

fn historian_hysteria_part_1(filename: &str) -> u32 {
    let [left, right] = read_input(filename);

    left.into_sorted_vec()
        .iter()
        .zip(right.into_sorted_vec().iter())
        .map(|(h, q)| h.abs_diff(*q))
        .sum()
}

fn historian_hysteria_part_2(filename: &str) -> usize {
    let [left, right] = read_input_part_2(filename);

    left.iter()
        .into_iter()
        .map(|(number, count)| (*number as usize * *right.get(&number).unwrap_or(&0)) * count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input() {
        let answer = historian_hysteria_part_1("inputs/1_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 11);
    }

    #[test]
    fn part_1_input() {
        let answer = historian_hysteria_part_1("inputs/1_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 2375403);
    }

    #[test]
    fn part_2_input_example() {
        let answer = historian_hysteria_part_2("inputs/1_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 31);
    }

    #[test]
    fn part_2_input() {
        let answer = historian_hysteria_part_2("inputs/1_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 23082277);
    }
}
