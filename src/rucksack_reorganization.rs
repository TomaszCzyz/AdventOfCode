use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct InputIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for InputIterator {
    type Item = (Vec<char>, Vec<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf_reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                let (first, second) = buf.trim_end().split_at(buf.len() / 2 - 1);

                let first_part = first.chars().collect::<Vec<_>>();
                let second_part = second.chars().collect::<Vec<_>>();

                Some((first_part, second_part))
            }
            Err(_e) => panic!(),
        }
    }
}

pub fn read_input(file_name: &str) -> InputIterator {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    InputIterator {
        buf_reader: reader,
    }
}

fn get_char_value(char: &char) -> u8 {
    match char {
        'a'..='z' => *char as u8 - 96,
        'A'..='Z' => *char as u8 - 38,
        _ => panic!()
    }
}

fn sum_chars(chars: Vec<&char>) -> i32 {
    chars.iter().map(|char| get_char_value(char)).map(|x| x as i32).sum()
}

pub fn rucksack_reorganization_part_1(file_name: &str) -> i32 {
    let mut total = 0;

    for (first_half, second_half) in read_input(file_name) {
        let first_set: HashSet<char> = first_half.into_iter().collect();
        let second_set: HashSet<char> = second_half.into_iter().collect();

        let intersection: Vec<&char> = first_set.intersection(&second_set).collect();

        total += sum_chars(intersection);
    }

    total
}

pub struct ThreeLineInputIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for ThreeLineInputIterator {
    type Item = (Vec<char>, Vec<char>, Vec<char>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = Vec::new();

        for _i in 1..=3 {
            let mut buf = String::new();
            let single_line = match self.buf_reader.read_line(&mut buf) {
                Ok(0) => return None,
                Ok(_n) => {
                    let line = buf.trim_end().chars().collect::<Vec<_>>();

                    Some(line)
                }
                Err(_e) => panic!(),
            };

            if let Some(t) = single_line {
                result.push(t);
            }
        }

        let final_result = (result[0].clone(), result[1].clone(), result[2].clone());
        Some(final_result)
    }
}

pub fn read_input_2(file_name: &str) -> ThreeLineInputIterator {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    ThreeLineInputIterator {
        buf_reader: reader,
    }
}

pub fn rucksack_reorganization_part_2(file_name: &str) -> i32 {
    let mut total = 0;

    for (first, second, third) in read_input_2(file_name) {
        let first_set: HashSet<char> = first.into_iter().collect();
        let second_set: HashSet<char> = second.into_iter().collect();
        let third_set: HashSet<char> = third.into_iter().collect();

        let intersection: HashSet<char> = first_set.intersection(&second_set).copied().collect();
        let intersection_2: Vec<&char> = third_set.intersection(&intersection).collect();

        total += sum_chars(intersection_2);
    }

    total
}
