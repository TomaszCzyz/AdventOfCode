use std::fs;

fn read_input(file_name: &str) -> Vec<Vec<i64>> {
    let _file_content = fs::read_to_string(file_name).unwrap();
    todo!();
}

fn part_1(filename: &str) -> i64 {
    let _ = read_input(filename);

    todo!();
}

fn part_2(filename: &str) -> i64 {
    let _ = read_input(filename);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/08_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 40);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/08_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4309240495780);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/08_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 3263827);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/08_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 9170286552289);
    }
}
