use itertools::Itertools;
use std::fs;

type Range = (usize, usize);
type Id = usize;

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Roll,
}

fn read_input(file_name: &str) -> Vec<Vec<Tile>> {
    let file_content = fs::read_to_string(file_name).unwrap();

    let mut columns = file_content
        .lines()
        .map(|line| {
            let mut row = line
                .chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '@' => Tile::Roll,
                    _ => panic!("invalid tile type"),
                })
                .collect::<Vec<_>>();

            row.insert(0, Tile::Empty);
            row.push(Tile::Empty);
            row
        })
        .collect::<Vec<_>>();

    columns.insert(0, vec![Tile::Empty; columns[0].len()]);
    columns.push(vec![Tile::Empty; columns[0].len()]);
    columns
}

fn part_1(filename: &str) -> usize {
    let map = read_input(filename);

    let width = map[0].len();
    let height = map.len();

    let mut heat_map = vec![vec![0; width]; height];

    for row_i in 1..width - 1 {
        for col_i in 1..height - 1 {
            if map[row_i][col_i] == Tile::Empty {
                continue;
            }

            if map[row_i][col_i + 1] == Tile::Roll {
                heat_map[row_i][col_i] += 1;
                heat_map[row_i][col_i + 1] += 1;
            }

            if map[row_i + 1][col_i] == Tile::Roll {
                heat_map[row_i][col_i] += 1;
                heat_map[row_i + 1][col_i] += 1;
            }

            if map[row_i + 1][col_i + 1] == Tile::Roll {
                heat_map[row_i][col_i] += 1;
                heat_map[row_i + 1][col_i + 1] += 1;
            }

            if map[row_i - 1][col_i + 1] == Tile::Roll {
                heat_map[row_i][col_i] += 1;
                heat_map[row_i - 1][col_i + 1] += 1;
            }
        }
    }

    for row in heat_map.iter().skip(1).take(height - 2) {
        for tile in row.iter().skip(1).take(width - 2) {
            if *tile < 4 {
                print!("x")
            } else {
                print!("{}", tile);
            }
        }
        println!();
    }

    let mut sum = 0;
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, tile) in row.iter().enumerate() {
            if *tile == Tile::Roll && heat_map[row_i][col_i] < 4 {
                sum += 1;
            }
        }
    }

    sum
}

fn part_2(filename: &str) -> usize {
    let _ = read_input(filename);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_example_input() {
        let map = read_input("inputs/04_input_example_1.txt");

        for row in map {
            for tile in row {
                match tile {
                    Tile::Empty => print!("."),
                    Tile::Roll => print!("@"),
                }
            }
            println!();
        }
    }

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/04_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 13);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/04_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 1604);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/04_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 14);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/04_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 353507173555373);
    }
}
