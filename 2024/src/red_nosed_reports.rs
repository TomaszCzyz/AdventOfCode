use std::fs;

type Levels = Vec<i32>;

fn read_input(file_name: &str) -> Vec<Levels> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|val| val.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn red_nosed_reports_part_1(filename: &str) -> usize {
    let levels = read_input(filename);

    let mut counter = levels.len();

    for level in levels.into_iter() {
        let mut i_first = 0;
        let mut i_last = level.len() - 1;
        let sign = if level[i_first] < level[i_last] {
            -1
        } else {
            1
        };

        while i_first < i_last {
            let i_first_next = i_first + 1;
            let i_last_prev = i_last - 1;
            let diff_first = (level[i_first] - level[i_first_next]) * sign;
            let diff_last = (level[i_last_prev] - level[i_last]) * sign;

            if (diff_first < 1 || diff_first > 3) || (diff_last < 1 || diff_last > 3) {
                counter -= 1;
                break;
            }

            i_first = i_first_next;
            i_last = i_last_prev;
        }
    }

    counter
}

fn is_row_valid(levels: &Levels) -> bool {
    levels
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .all(|diff| diff >= 1 && diff <= 3)
}

fn red_nosed_reports_part_2(filename: &str) -> usize {
    let levels = read_input(filename);
    let mut counter = 0;

    let levels_increasing = levels
        .iter()
        .map(|levels| {
            let negative_count = levels
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .filter(|diff| *diff < 0)
                .count();

            return if negative_count > 1 {
                levels.iter().rev().cloned().collect::<Vec<i32>>()
            } else {
                levels.clone()
            };
        })
        .collect::<Vec<Levels>>();

    for levels in levels_increasing.into_iter() {
        let mut curr_levels = levels.clone();

        for j in 0..levels.len() + 1 {
            if is_row_valid(&curr_levels) {
                counter += 1;
                break;
            }

            let mut modified = levels.clone();
            modified.remove(j.min(levels.len() - 1));
            curr_levels = modified;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input() {
        let answer = red_nosed_reports_part_1("inputs/2_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part_1_input() {
        let answer = red_nosed_reports_part_1("inputs/2_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 326);
    }

    #[test]
    fn part_2_input_example() {
        let answer = red_nosed_reports_part_2("inputs/2_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part_2_input() {
        let answer = red_nosed_reports_part_2("inputs/2_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 381);
    }
}
