pub use advent_of_code_2022::beacon_exclusion_zone::beacon_exclusion_zone_part_1;

const EXAMPLE_ROW_NUM: i32 = 10;
const INPUT_ROW_NUM: i32 = 2_000_000;

fn main() {
    let answer = beacon_exclusion_zone_part_1("inputs/15_input.txt", INPUT_ROW_NUM);

    println!("There are {} positions that cannot contain a beacon", answer);
}