use std::fs;

const T_A: f64 = 3.;
const T_B: f64 = 1.;

#[derive(Debug)]
enum BlockType {
    None,
    Robot,
    Box,
    Wall,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_input(file_name: &str) -> (Vec<Vec<BlockType>>, Vec<Direction>) {
    let map = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => BlockType::Wall,
                    '@' => BlockType::Robot,
                    'O' => BlockType::Box,
                    '.' => BlockType::None,
                    _ => panic!("invalid character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let directions = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .skip_while(|line| !line.is_empty())
        .flat_map(|line| {
            line.chars()
                .map(|c| match c {
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    _ => panic!("invalid character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (map, directions)
}

fn warehouse_woes_part_1(filename: &str) -> u64 {
    let inputs = read_input(filename);

    todo!()
}

fn warehouse_woes_part_2(filename: &str) -> u64 {
    let inputs = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_test() {
        let answer = read_input("inputs/15_input_example.txt");
        println!("{answer:?}")
    }

    #[test]
    fn part_1_example_input() {
        let answer = warehouse_woes_part_1("inputs/15_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 480);
    }

    #[test]
    fn part_1_input() {
        let answer = warehouse_woes_part_1("inputs/15_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 29877);
    }

    #[test]
    fn part_2_input_example() {
        let answer = warehouse_woes_part_2("inputs/15_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 875318608908);
    }

    #[test]
    fn part_2_input() {
        let answer = warehouse_woes_part_2("inputs/15_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 99423413811305);
    }
}
