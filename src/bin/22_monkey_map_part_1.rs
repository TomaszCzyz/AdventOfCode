pub use advent_of_code_2022::monkey_map::monkey_map_part_1;

fn main() {
    let answer = monkey_map_part_1("inputs/22_input.txt");

    println!("Final password is: {}", answer);
}