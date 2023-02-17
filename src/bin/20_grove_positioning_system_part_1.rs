pub use advent_of_code_2022::grove_positioning_system::grove_positioning_system_part_1;

fn main() {
    let answer = grove_positioning_system_part_1("inputs/20_input_example.txt");

    println!("sum of coordinates is: {}", answer);
}