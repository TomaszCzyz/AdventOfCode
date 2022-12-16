pub use advent_of_code_2022::beacon_exclusion_zone::beacon_exclusion_zone_part_2;


const EXAMPLE_SQUARE_SIZE: i32 = 20;
const INPUT_SQUARE_SIZE: i32 = 4_000_000;

fn main() {
    let answer = beacon_exclusion_zone_part_2("inputs/15_input.txt", INPUT_SQUARE_SIZE);

    println!("Tuning frequency is: {}", answer);
}