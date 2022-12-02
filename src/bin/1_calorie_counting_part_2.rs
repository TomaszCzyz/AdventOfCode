use advent_of_code_2022::calculate_top_n_max_calories;

fn main() {
    if let Ok(calories) = calculate_top_n_max_calories("input.txt", 5) {
        println!("top amounts of calories: {}", calories.iter().map(|&i| i.to_string()).collect::<Vec<_>>().join(", "));

        println!("total of 3 max calories: {}", calories.into_iter().take(3).sum::<i32>());
    }
}