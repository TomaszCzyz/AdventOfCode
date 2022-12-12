use advent_of_code_2022::monkey_in_the_middle::{DecreaseStrategy, monkey_in_the_middle};

const ROUND_COUNT: usize = 10_000;

fn main() {
    let answer = monkey_in_the_middle("inputs/11_input.txt", ROUND_COUNT, DecreaseStrategy::DivideByDivisorProduct);

    println!("Monkey business after {ROUND_COUNT} rounds is: {}", answer);
}