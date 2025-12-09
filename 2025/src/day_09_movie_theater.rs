use std::fs;

fn read_input(file_name: &str) -> Vec<usize> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<Vec<_>>();

    todo!()
}

fn part_1(filename: &str, laps: usize) -> usize {
    let _ = read_input(filename);

    todo!()
}

fn part_2(filename: &str) -> i64 {
    let _ = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/09_input_example_1.txt", 10);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 40);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = part_1("inputs/09_input_example_2.txt", 5);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/09_input.txt", 1000);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 103488);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/09_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 25272);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/09_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 8759985540);
    }
}
