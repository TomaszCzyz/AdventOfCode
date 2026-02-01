fn read_input(file_name: &str) -> Vec<usize> {
    todo!()
}

fn part_1(filename: &str) -> usize {
    let _ = read_input(filename);

    todo!()
}

fn part_2(filename: &str) -> i64 {
    _ = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/11_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 7);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = part_1("inputs/11_input_example_2.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/11_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 4761736832);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/11_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 24);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/11_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 1452422268);
    }
}
