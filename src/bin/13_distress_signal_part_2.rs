pub use advent_of_code_2022::distress_signal::distress_signal_part_2;

fn main() {
    let answer = distress_signal_part_2("inputs/13_input.txt");

    println!("The product of indices of extra packets equals: {}", answer);
}