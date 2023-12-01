pub use advent_of_code_2022::grove_positioning_system::grove_positioning_system_part_2;

fn main() {
    let answer = grove_positioning_system_part_2("inputs/20_input.txt");

    println!("sum of coordinates is: {}", answer);
}