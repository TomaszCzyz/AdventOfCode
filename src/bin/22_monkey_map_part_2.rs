use advent_of_code_2022::monkey_map::InputInfo;
pub use advent_of_code_2022::monkey_map::monkey_map_part_2;

fn main() {
    let answer = monkey_map_part_2("inputs/22_input.txt" , InputInfo::input22());

    println!("Final password is: {}", answer);
}