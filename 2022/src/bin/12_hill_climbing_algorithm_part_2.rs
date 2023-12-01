pub use advent_of_code_2022::hill_climbing_algorithm::hill_climbing_algorithm_part_2;

fn main() {
    let answer = hill_climbing_algorithm_part_2("inputs/12_input.txt");

    println!("The shortest path has length of: {}", answer);
}