pub use advent_of_code_2022::monkey_map::monkey_map_part_2;

fn main() {
    let answer = monkey_map_part_2("inputs/22_input_example_1.txt");

    println!("Final password is: {}", answer);
}