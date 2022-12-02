use std::fs::File;
use std::io::{self, BufReader, prelude::*};

pub mod rock_paper_scissors;

pub fn calculate_max_calories(file_name: &str) -> Result<(i32, i32), io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut max_calories = 0;
    let mut max_elf_num = 1;
    let mut elf_counter = 1;
    let mut single_elf_calories = 0;

    for result in reader.lines() {
        let line = result.unwrap();

        if line == "" {
            if single_elf_calories > max_calories {
                (max_elf_num, max_calories) = (elf_counter, single_elf_calories);
            }

            single_elf_calories = 0;
            elf_counter += 1;
            continue;
        }

        let calories: i32 = line.parse().unwrap();
        single_elf_calories += calories;
    }

    Ok((max_calories, max_elf_num))
}

pub fn calculate_top_n_max_calories(file_name: &str, n: usize) -> Result<Vec<i32>, io::Error> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut max_calories = Vec::new();
    let mut single_elf_calories = 0;

    for result in reader.lines() {
        let line = result.unwrap();

        if line == "" {
            if max_calories.len() == 0 {
                max_calories.push(single_elf_calories);
                single_elf_calories = 0;
                continue;
            }

            if let Err(pos) = max_calories.binary_search_by(|probe| single_elf_calories.cmp(probe)) {
                if pos != max_calories.len() {
                    max_calories.insert(pos, single_elf_calories);
                    max_calories.truncate(n);
                }
            }

            single_elf_calories = 0;
            continue;
        }

        let calories: i32 = line.parse().unwrap();
        single_elf_calories += calories;
    }

    Ok(max_calories)
}
