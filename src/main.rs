use advent_of_code_2022::calculate_max_calories;

fn main() {
    if let Ok((calories, elf_num)) = calculate_max_calories("input.txt") {
        println!("elf {} has MAX calories, which is: {}", elf_num, calories);
    }
}