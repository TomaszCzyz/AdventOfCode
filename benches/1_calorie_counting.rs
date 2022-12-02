use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2022::{calculate_max_calories, calculate_top_n_max_calories};

fn calorie_counting_part_1(c: &mut Criterion) {
    c.bench_function("task 1 - Calorie Counting - part 1", |b| b.iter(|| calculate_max_calories_default_params()));
}

fn calorie_counting_part_2(c: &mut Criterion) {
    c.bench_function("task 1 - Calorie Counting - part 2", |b| b.iter(|| calculate_top_n_max_calories_default_params()));
}

criterion_group!(benches, calorie_counting_part_1, calorie_counting_part_2);
criterion_main!(benches);

fn calculate_max_calories_default_params() {
    let _ = calculate_max_calories("input");
}

fn calculate_top_n_max_calories_default_params() {
    let _ = calculate_top_n_max_calories("input", 3);
}