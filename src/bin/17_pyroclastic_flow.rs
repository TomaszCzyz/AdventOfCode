pub use advent_of_code_2022::pyroclastic_flow::pyroclastic_flow;

const ROCKS_NUMBER: usize = 1_000_000_000_000;
// const ROCKS_NUMBER: usize = 2_022;

fn main() {
    let answer = pyroclastic_flow("inputs/17_input.txt", ROCKS_NUMBER);

    println!("tower is {} high", answer);
}