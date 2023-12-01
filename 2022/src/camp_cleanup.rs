use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

pub fn read_input(file_name: &str) -> SectionsIterator {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    SectionsIterator {
        buf_reader: reader,
    }
}

pub struct SectionsIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for SectionsIterator {
    type Item = ((u32, u32), (u32, u32));

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf_reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                let x: ((u32, u32), (u32, u32)) = buf.trim_end()
                    .split(',')
                    .map(|range| range.split('-')
                        .map(|edge| edge.parse::<u32>().unwrap())
                        .collect::<Vec<_>>())
                    .map(|pair| (pair[0], pair[1]))
                    .collect_tuple()
                    .unwrap();

                Some(x)
            }
            Err(_e) => panic!(),
        }
    }
}

pub fn camp_cleanup_part_1(file_name: &str) -> i32 {
    let mut counter = 0;
    for (first, second) in read_input(file_name) {
        if (first.0 <= second.0 && first.1 >= second.1) || (first.0 >= second.0 && first.1 <= second.1) {
            counter += 1;
        }
    }

    counter
}

pub fn camp_cleanup_part_2(file_name: &str) -> i32 {
    let mut counter = 0;
    for (first, second) in read_input(file_name) {
        // completely overlapping
        if (first.0 <= second.0 && first.1 >= second.1) || (first.0 >= second.0 && first.1 <= second.1) {
            counter += 1;
            continue;
        }

        // partially overlapping
        if (first.0 >= second.0 && first.0 <= second.1)
            || (first.1 >= second.0 && first.1 <= second.1)
            || (second.0 >= first.0 && second.0 <= first.1)
            || (second.1 >= first.0 && second.1 <= first.1
        ) {
            counter += 1;
            continue;
        }
    }

    counter
}