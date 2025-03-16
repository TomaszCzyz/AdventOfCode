use std::fs;

#[derive(Debug)]
struct ClawMachine {
    xa: f64,
    ya: f64,
    xb: f64,
    yb: f64,
    x: f64,
    y: f64,
}

const T_A: f64 = 3.;
const T_B: f64 = 1.;

fn read_input(file_name: &str) -> Vec<ClawMachine> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>()
        .chunks(4)
        .map(|lines| {
            let line1 = lines[0];
            let line2 = lines[1];
            let line3 = lines[2];
            let a_numbers = line1
                .trim_start_matches("Button A:")
                .split(',')
                .map(|x| {
                    x.trim_start_matches(|c: char| !c.is_numeric())
                        .parse::<f64>()
                        .unwrap()
                })
                .collect::<Vec<f64>>();

            let (xa, ya) = (a_numbers[0], a_numbers[1]);

            let b_numbers = line2
                .trim_start_matches("Button B:")
                .split(',')
                .map(|x| {
                    x.trim_start_matches(|c: char| !c.is_numeric())
                        .parse::<f64>()
                        .unwrap()
                })
                .collect::<Vec<f64>>();

            let (xb, yb) = (b_numbers[0], b_numbers[1]);

            let prize_numbers = line3
                .trim_start_matches("Prize:")
                .split(',')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    x.trim_start_matches(|c: char| !c.is_numeric())
                        .parse::<f64>()
                        .unwrap()
                })
                .collect::<Vec<f64>>();

            let (x, y) = (prize_numbers[0], prize_numbers[1]);

            ClawMachine {
                xa,
                ya,
                xb,
                yb,
                x,
                y,
            }
        })
        .collect()
}

fn claw_contraption_part_1(filename: &str) -> u64 {
    let inputs = read_input(filename);

    inputs
        .iter()
        .map(|input| analyze_input_2(input))
        .filter_map(|x| x)
        .sum()
}

fn analyze_input(input: &ClawMachine) -> Option<u64> {
    let mut result_starting_from_a = f64::MAX;
    for click_a_num in 1..=100 {
        let cost = calculate_cost_for_clicks_a(input, click_a_num as f64);
        if cost.fract() != 0. {
            continue;
        }

        let click_b_num = (cost - T_A * click_a_num as f64) / T_B;

        if click_b_num.fract() != 0. {
            continue;
        }

        if click_a_num as f64 * input.xa + click_b_num * input.xb != input.x {
            continue;
        }

        if click_b_num > 100. {
            break;
        }

        result_starting_from_a = cost;
        break;
    }

    let mut result_starting_from_b = f64::MAX;
    for click_b_num in 1..=100 {
        let cost = calculate_cost_for_clicks_b(input, click_b_num as f64);
        if cost.fract() != 0. {
            continue;
        }

        let click_a_num = (cost - T_B * click_b_num as f64) / T_A;

        if click_a_num.fract() != 0. {
            continue;
        }

        if click_a_num * input.xa + click_b_num as f64 * input.xb != input.x {
            continue;
        }

        if click_a_num > 100. {
            break;
        }

        result_starting_from_b = cost;
        break;
    }

    let min = result_starting_from_a.min(result_starting_from_b);
    if min != f64::MAX {
        Some(min as u64)
    } else {
        None
    }
}

fn calculate_cost_for_clicks_a(input: &ClawMachine, click_num: f64) -> f64 {
    T_B * ((input.x + input.y) / (input.xb + input.yb))
        + click_num * (T_A - T_B * ((input.xa + input.ya) / (input.xb + input.yb)))
}

fn calculate_cost_for_clicks_b(input: &ClawMachine, click_num: f64) -> f64 {
    T_A * ((input.x + input.y) / (input.xa + input.ya))
        + click_num * (T_B - T_A * ((input.xb + input.yb) / (input.xa + input.ya)))
}

fn claw_contraption_part_2(filename: &str) -> u64 {
    let inputs = read_input(filename);

    let inputs = inputs
        .iter()
        .map(|input| ClawMachine {
            xa: input.xa,
            ya: input.ya,
            xb: input.xb,
            yb: input.yb,
            x: input.x + 10000000000000f64,
            y: input.y + 10000000000000f64,
        })
        .collect::<Vec<_>>();

    inputs
        .iter()
        .map(|input| analyze_input_2(input))
        .filter_map(|x| x)
        .sum()
}

fn analyze_input_2(input: &ClawMachine) -> Option<u64> {
    let slope_a = input.ya / input.xa;
    let slope_b = input.yb / input.xb;

    let p_x = (input.y - slope_a * input.x) / (slope_b - slope_a);
    let p_y = slope_b * p_x;

    // early return, does not change the result
    if !is_close_to_integer(p_x) || !is_close_to_integer(p_y) {
        return None;
    }

    let p_b_length = (p_x * p_x + p_y * p_y).sqrt();
    let p_a_length = ((input.x - p_x) * (input.x - p_x) + (input.y - p_y) * (input.y - p_y)).sqrt();

    let a_length = (input.xa * input.xa + input.ya * input.ya).sqrt();
    let b_length = (input.xb * input.xb + input.yb * input.yb).sqrt();

    let a_ratio = p_a_length / a_length;
    let b_ratio = p_b_length / b_length;
    if is_close_to_integer(a_ratio) && is_close_to_integer(b_ratio) {
        let cost = a_ratio * T_A + b_ratio * T_B;
        return Some(cost.round() as u64);
    }

    let a_ratio = p_b_length / a_length;
    let b_ratio = p_a_length / b_length;
    if is_close_to_integer(a_ratio) && is_close_to_integer(b_ratio) {
        let cost = a_ratio * T_A + b_ratio * T_B;
        return Some(cost.round() as u64);
    }

    None
}

fn is_close_to_integer(value: f64) -> bool {
    let rounded = value.round();
    (value - rounded).abs() <= 0.01
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input() {
        let answer = claw_contraption_part_1("inputs/13_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 480);
    }

    #[test]
    fn part_1_input() {
        let answer = claw_contraption_part_1("inputs/13_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 29877);
    }

    #[test]
    fn part_2_input_example() {
        let answer = claw_contraption_part_2("inputs/13_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 875318608908);
    }

    #[test]
    fn part_2_input() {
        let answer = claw_contraption_part_2("inputs/13_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 99423413811305);
    }

    #[test]
    fn analyze_2_test() {
        let answer = analyze_input_2(&ClawMachine {
            xa: 4.0,
            ya: 2.0,
            xb: 1.0,
            yb: 3.0,
            x: 11.0,
            y: 13.0,
        });

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, Some(9));
    }

    #[test]
    fn analyze_2_test_2() {
        let answer = analyze_input_2(&ClawMachine {
            xa: 94.0,
            ya: 34.0,
            xb: 22.0,
            yb: 67.0,
            x: 8400.0,
            y: 5400.0,
        });

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, Some(280));
    }
}
