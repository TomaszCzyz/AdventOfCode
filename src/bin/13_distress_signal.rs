pub use advent_of_code_2022::distress_signal::distress_signal_part_1;

fn main() {
    let answer = distress_signal_part_1("inputs/13_input_example.txt");

    println!("The sum of indices of pairs in correct order equals: {}", answer);
}