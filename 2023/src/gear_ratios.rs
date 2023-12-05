use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coord {
    row: usize,
    col: usize,
}

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

#[derive(Debug)]
pub struct TaskData {
    numbers: Vec<u32>,
    numbers_coords: Vec<Coord>,
    coord_to_number_index: HashMap<Coord, usize>,
    symbols: Vec<(char, Coord)>,
}

pub fn read_input_part_1(file_name: &str) -> TaskData {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut numbers = Vec::new();
    let mut numbers_coords = Vec::new();
    let mut coord_to_number_index = HashMap::new();

    let mut symbols = Vec::new();
    let mut row = 0usize;

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut iter = buf.chars().enumerate().peekable();

        while let Some((col, ch)) = iter.next() {
            if ch.is_digit(10) {
                let mut digits = String::from(ch);
                let mut coords = Vec::from([Coord { row, col }]);

                while let Some((_, next_char)) = iter.peek() {
                    if next_char.is_digit(10) {
                        let (col, ch) = iter.next().unwrap();
                        digits.push(ch);
                        coords.push(Coord { row, col });
                    } else {
                        break;
                    }
                }

                let num = digits.parse::<u32>().unwrap();

                numbers.push(num);
                for coord in coords {
                    coord_to_number_index.insert(coord, numbers.len() - 1);
                    numbers_coords.push(coord)
                }
            } else if ch == '.' || ch == '\n' || ch == '\r' {
                continue;
            } else {
                symbols.push((ch, Coord { row, col }));
            }
        }

        buf = String::new();
        row += 1;
    }

    TaskData {
        numbers,
        numbers_coords,
        coord_to_number_index,
        symbols,
    }
}

fn get_neighbors(c: &Coord) -> [Coord; 8] {
    [
        Coord { row: c.row - 1, col: c.col - 1 },
        Coord { row: c.row - 1, col: c.col },
        Coord { row: c.row - 1, col: c.col + 1 },
        Coord { row: c.row, col: c.col + 1 },
        Coord { row: c.row + 1, col: c.col + 1 },
        Coord { row: c.row + 1, col: c.col },
        Coord { row: c.row + 1, col: c.col - 1 },
        Coord { row: c.row, col: c.col - 1 },
    ]
}

fn gear_ratios_part_1(filename: &str) -> u32 {
    let task_data = read_input_part_1(filename);
    let mut numbers = Vec::new();
    let mut visited_number_indices = Vec::new();

    let symbols_coords = task_data.symbols.iter()
        .map(|(_, coord)| coord)
        .flat_map(|c| get_neighbors(c));

    for coord in symbols_coords {
        if task_data.numbers_coords.contains(&coord) {
            let number_index = task_data.coord_to_number_index[&coord];
            if visited_number_indices.contains(&number_index) {
                continue;
            } else {
                numbers.push(task_data.numbers[number_index]);
                visited_number_indices.push(number_index);
            }
        }
    }

    numbers.iter().sum()
}

fn gear_ratios_part_2(filename: &str) -> u32 {
    let data = read_input_part_1(filename);
    let mut sum = 0;

    for (ch, coord) in data.symbols.iter() {
        if *ch == '*' {
            let distinct_adjacent_numbers = get_neighbors(coord).iter()
                .filter(|&coord| data.numbers_coords.contains(&coord))
                .map(|coord| data.numbers[data.coord_to_number_index[coord]])
                .collect::<HashSet<u32>>();

            if distinct_adjacent_numbers.len() == 2 {
                sum += distinct_adjacent_numbers.iter().fold(1, |acc, x| acc * x);
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

// #[test]
    // fn read_input() {
    //     let answer = read_input_part_1("inputs/2_input_example.txt");
    // 
    //     println!("answer: {:#?}", answer);
    // }

    #[test]
    fn part_1_example_input() {
        let answer = gear_ratios_part_1("inputs/3_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4361);
    }

    #[test]
    fn part_1_input() {
        let answer = gear_ratios_part_1("inputs/3_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 514969);
    }

    #[test]
    fn part_2_input_example() {
        let answer = gear_ratios_part_2("inputs/3_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 467835);
    }

    #[test]
    fn part_2_input() {
        let answer = gear_ratios_part_2("inputs/3_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 78915902);
    }
}
