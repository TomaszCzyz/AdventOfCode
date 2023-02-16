use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(file_name: &str) -> Vec<i32> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut numbers: Vec<i32> = Vec::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let i = buf.trim().parse::<i32>().unwrap();

        numbers.push(i);
        buf = String::new();
    };

    numbers
}

/// mapped numbers example:
/// [1,2,1,3,4]
/// [0,1,2,3,4]
///
/// 0->1
/// 1->2
/// 2->1
/// 3->3
/// 4->4
pub fn grove_positioning_system_part_1(file_name: &str) -> i32 {
    let numbers = read_input(file_name);
    let mut mapped_numbers = (0..numbers.len()).collect::<Vec<usize>>();

    println!("{mapped_numbers:?}  ({:?})", mapped_numbers.iter().map(|&num| numbers[num]).collect::<Vec<_>>());

    for number in mapped_numbers.clone().into_iter() {
        let move_amount = numbers[number];

        if move_amount == 0 {
            continue;
        }

        let initial_pos = mapped_numbers.iter().position(|num| *num == number).unwrap();
        let mut new_pos = (initial_pos as i32 + move_amount).rem_euclid(numbers.len() as i32 - 1) as usize;

        // when moving backwards, we jump to the end on a sequence
        if new_pos == 0 && move_amount < 0 {
            new_pos = numbers.len() - 1;
        }

        // move elements between initial position and new position
        if initial_pos < new_pos {
            for i in initial_pos..new_pos {
                mapped_numbers[i] = mapped_numbers[i + 1];
            }
            mapped_numbers[new_pos] = number;
        } else {
            for i in (new_pos..initial_pos).rev() {
                mapped_numbers[i + 1] = mapped_numbers[i];
            }
            mapped_numbers[new_pos] = number;
        };

        // println!("moving number: {} to position: {new_pos}", numbers[number]);
        // println!("{mapped_numbers:?}  ({:?})", mapped_numbers.iter().map(|&num| numbers[num]).collect::<Vec<_>>());
    }

    let unmapped_numbers = mapped_numbers.iter().map(|&num| numbers[num]).collect::<Vec<_>>();
    let zero_index = unmapped_numbers.iter().position(|&num| num == 0).unwrap();

    println!("unmapped numbers: {:?}", unmapped_numbers);

    unmapped_numbers[(zero_index + 1000) % numbers.len() as i32 as usize]
        + unmapped_numbers[(zero_index + 2000) % numbers.len() as i32 as usize]
        + unmapped_numbers[(zero_index + 3000) % numbers.len() as i32 as usize]
}