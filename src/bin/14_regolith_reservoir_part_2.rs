pub use advent_of_code_2022::regolith_reservoir::regolith_reservoir_part_2;

fn main() {
    let answer = regolith_reservoir_part_2("inputs/14_input.txt");

    println!("Units of sand: {}", answer);
}