pub use advent_of_code_2022::boiling_boulders::boiling_boulders_part_1;

fn main() {
    let answer = boiling_boulders_part_1("inputs/18_input.txt");

    println!("surface area of your scanned lava droplet is {}", answer);
}