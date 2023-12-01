pub use advent_of_code_2022::unstable_diffusion::unstable_diffusion_part_1;

fn main() {
    let answer = unstable_diffusion_part_1("inputs/23_input_example.txt");

    println!("Number of free tiles: {}", answer);
}