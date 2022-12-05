use criterion::{Criterion, criterion_group, criterion_main};

use advent_of_code_2022::rucksack_reorganization::{rucksack_reorganization_part_1, rucksack_reorganization_part_2};

fn bench_1(c: &mut Criterion) {
    c.bench_function("task 3 - Calorie Counting - part 1", |b| b.iter(|| rucksack_reorganization_part_1("3_input.txt")));
}

fn bench_2(c: &mut Criterion) {
    c.bench_function("task 3 - Calorie Counting - part 2", |b| b.iter(|| rucksack_reorganization_part_2("3_input.txt")));
}

criterion_group!(benches, bench_1, bench_2);
criterion_main!(benches);

