use std::fs;

pub fn tuning_trouble_part_1(file_name: &str) -> usize {
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let chars = contents.chars().collect::<Vec<_>>();
    let mut index = 0;

    loop {
        if contains_repetition(&chars[index..index + 4]) {
            index += 1;
            continue;
        }

        return index + 4;
    }
}

fn contains_repetition(chars: &[char]) -> bool {
    for (i, char) in chars.iter().enumerate() {
        for char2 in chars.iter().skip(i + 1) {
            if *char == *char2 {
                return true;
            }
        }
    }

    false
}

pub fn tuning_trouble_part_2(file_name: &str) -> usize {
    let contents = fs::read_to_string(file_name).expect("Should have been able to read the file");

    let chars = contents.chars().collect::<Vec<_>>();
    let mut index = 0;

    loop {
        if contains_repetition(&chars[index..index + 14]) {
            index += 1;
            continue;
        }

        return index + 14;
    }
}