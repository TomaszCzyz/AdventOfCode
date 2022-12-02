use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2022::calculate_max_calories;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("task 1 - Calorie Counting", |b| b.iter(|| calculate_max_calories_default_name()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

fn calculate_max_calories_default_name() {
    let _ = calculate_max_calories("input");
}