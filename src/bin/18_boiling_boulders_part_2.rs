pub use advent_of_code_2022::boiling_boulders::boiling_boulders_part_2;

fn main() {
    let answer = boiling_boulders_part_2("inputs/18_input_example_my_7.txt");

    println!("exterior surface area of your scanned lava droplet is {}", answer);
}