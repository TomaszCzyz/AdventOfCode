use std::fs::read;
use std::io::BufRead;

pub fn read_input(file_name: &str) -> Vec<(usize, usize)> {
    let file = std::fs::File::open(file_name).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut buf = String::new();
    _ = reader.read_line(&mut buf).unwrap();

    let times = buf
        .trim_start_matches("Time:")
        .trim()
        .split(' ')
        .filter_map(|num| num.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    buf = String::new();
    _ = reader.read_line(&mut buf).unwrap();

    let distances = buf
        .trim_start_matches("Distance:")
        .trim()
        .split(' ')
        .filter_map(|num| num.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();

    times.into_iter().zip(distances).collect::<Vec<(usize, usize)>>()
}

fn calc_quadratic_solutions(t: f32, s: f32) -> (f32, f32) {
    let square_delta = f32::sqrt(f32::powi(t, 2) - 4. * s);
    let t1 = (t - square_delta) / 2.;
    let t2 = (t + square_delta) / 2.;

    (t1, t2)
}

fn count_integers_in_interval(start: f32, end: f32) -> usize {
    let start_int = start.ceil() as usize;
    let end_int = end.floor() as usize;

    let mut count = if start_int < end_int {
        end_int - start_int + 1
    } else {
        0usize
    };

    if start.fract() == 0.0 {
        count -= 1;
    }

    if end.fract() == 0.0 {
        count -= 1;
    }

    count
}

fn wait_for_it_part_1(filename: &str) -> usize {
    let input = read_input(filename);

    input.iter()
        .map(|(t, s)| calc_quadratic_solutions(*t as f32, *s as f32))
        .map(|(start, end)| count_integers_in_interval(start, end))
        .fold(1, |acc, x| acc * x)
}


fn wait_for_it_part_2(filename: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_test() {
        let input = read_input("inputs/6_input_example.txt");

        println!("seeds: {input:?}");
    }

    #[test]
    fn part_1_input_example() {
        let answer = wait_for_it_part_1("inputs/6_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 288);
    }

    #[test]
    fn part_1_input() {
        let answer = wait_for_it_part_1("inputs/6_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 449550);
    }

    #[test]
    fn part_2_input_example() {
        let answer = wait_for_it_part_2("inputs/6_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 46);
    }

    #[test]
    fn part_2_input() {
        let answer = wait_for_it_part_2("inputs/6_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }
}
