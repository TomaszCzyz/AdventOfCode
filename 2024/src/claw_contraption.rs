use std::fs;

#[derive(Debug)]
struct ClawMachine {
    xa: f32,
    ya: f32,
    xb: f32,
    yb: f32,
    x: f32,
    y: f32,
}

const T_A: f32 = 3.;
const T_B: f32 = 1.;

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
                        .parse::<f32>()
                        .unwrap()
                })
                .collect::<Vec<f32>>();

            let (xa, ya) = (a_numbers[0], a_numbers[1]);

            let b_numbers = line2
                .trim_start_matches("Button B:")
                .split(',')
                .map(|x| {
                    x.trim_start_matches(|c: char| !c.is_numeric())
                        .parse::<f32>()
                        .unwrap()
                })
                .collect::<Vec<f32>>();

            let (xb, yb) = (b_numbers[0], b_numbers[1]);

            let prize_numbers = line3
                .trim_start_matches("Prize:")
                .split(',')
                .filter(|x| !x.is_empty())
                .map(|x| {
                    x.trim_start_matches(|c: char| !c.is_numeric())
                        .parse::<f32>()
                        .unwrap()
                })
                .collect::<Vec<f32>>();

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

fn claw_contraption_part_1(filename: &str) -> u32 {
    let inputs = read_input(filename);

    inputs
        .iter()
        .map(|input| analyze_input(input))
        .filter_map(|x| x)
        .sum()
}

fn analyze_input(input: &ClawMachine) -> Option<u32> {
    // let slop_a = input.xa / input.ya;
    // let slop_b = input.xb / input.yb;
    // let slop_prize = input.x / input.y;

    let mut result_starting_from_a = f32::MAX;
    for click_a_num in 1..=100 {
        let cost = calculate_cost_for_clicks_a(input, click_a_num as f32);
        if cost.fract() != 0. {
            continue;
        }

        let click_b_num = (cost - T_A * click_a_num as f32) / T_B;

        if click_b_num.fract() != 0. {
            continue;
        }

        if click_a_num as f32 * input.xa + click_b_num * input.xb != input.x {
            continue;
        }

        if click_b_num > 100. {
            break;
        }

        result_starting_from_a = cost;
        break;
    }

    let mut result_starting_from_b = f32::MAX;
    for click_b_num in 1..=100 {
        let cost = calculate_cost_for_clicks_b(input, click_b_num as f32);
        if cost.fract() != 0. {
            continue;
        }

        let click_a_num = (cost - T_B * click_b_num as f32) / T_A;

        if click_a_num.fract() != 0. {
            continue;
        }

        if click_a_num * input.xa + click_b_num as f32 * input.xb != input.x {
            continue;
        }

        if click_a_num > 100. {
            break;
        }

        result_starting_from_b = cost;
        break;
    }

    let min = result_starting_from_a.min(result_starting_from_b);
    if min != f32::MAX {
        Some(min as u32)
    } else {
        None
    }
}

fn calculate_cost_for_clicks_a(input: &ClawMachine, click_num: f32) -> f32 {
    T_B * ((input.x + input.y) / (input.xb + input.yb))
        + click_num * (T_A - T_B * ((input.xa + input.ya) / (input.xb + input.yb)))
}

fn calculate_cost_for_clicks_b(input: &ClawMachine, click_num: f32) -> f32 {
    T_A * ((input.x + input.y) / (input.xa + input.ya))
        + click_num * (T_B - T_A * ((input.xb + input.yb) / (input.xa + input.ya)))
}

fn claw_contraption_part_2(filename: &str) -> usize {
    let _ = read_input(filename);
    todo!()
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
        assert_eq!(answer, 4);
    }

    #[test]
    fn part_2_input() {
        let answer = claw_contraption_part_2("inputs/13_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 381);
    }
}
