use itertools::Itertools;
use std::fs;

type Range = (usize, usize);
type Id = usize;

fn read_input(file_name: &str) -> (Vec<Range>, Vec<Id>) {
    let file_content = fs::read_to_string(file_name).unwrap();

    let ranges = file_content
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|line| {
            let values = line
                .split("-")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (values[0], values[1])
        })
        .collect::<Vec<_>>();

    let ids = file_content
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (ranges, ids)
}

fn part_1(filename: &str) -> usize {
    let (ranges, ids) = read_input(filename);

    let sorted_ranges = ranges
        .into_iter()
        .sorted_by_key(|x| x.0)
        .collect::<Vec<Range>>();

    let mut merged: Vec<Id> = Vec::new();

    let mut iter = sorted_ranges.into_iter();
    let mut current = iter.next().unwrap();
    while let Some(next_range) = iter.next() {
        if next_range.0 <= current.1 && next_range.1 <= current.1 {
            // contained
            continue;
        } else if next_range.0 <= current.1 && next_range.1 > current.1 {
            // overlap
            current = (current.0, next_range.1);
        } else {
            // disjoint
            merged.push(current.0);
            merged.push(current.1);
            current = next_range;
        }
    }

    merged.push(current.0);
    merged.push(current.1);

    let mut sum: usize = 0;
    for ingredient_id in ids {
        match merged.binary_search(&ingredient_id) {
            Ok(_) => {
                sum += 1;
            }
            Err(pos) => {
                if pos == merged.len() {
                    continue;
                }

                if pos % 2 == 1 {
                    // within a range
                    sum += 1;
                }
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_example_input() {
        let (ranges, ids) = read_input("inputs/05_input_example.txt");

        for r in ranges {
            println!("{:?}", r);
        }

        for id in ids {
            println!("{:?}", id);
        }
    }

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/05_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 3);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = part_1("inputs/05_input_example_2.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 3);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/05_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 623);
    }
}
