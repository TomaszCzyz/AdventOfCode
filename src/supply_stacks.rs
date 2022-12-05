use std::fs::File;
use std::io::{BufRead, BufReader};

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    qty: i32,
    start: i32,
    end: i32,
}


fn read_input(file_name: &str) -> (Stacks, Vec<Move>) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    loop {
        let mut buf = String::new();
        let _ = reader.read_line(&mut buf).unwrap();

        if buf.starts_with(" 1   2") {
            break;
        }

        lines.push(buf.clone());
    };

    let num_of_stacks = (lines.last().unwrap().len() + 1) / 4;
    let mut stacks: Stacks = vec![Vec::new(); num_of_stacks];

    // parse stacks
    for line in lines.iter().rev() {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let segment = &line.chars().nth((i * 4) + 1).unwrap();

            if segment.is_alphabetic() {
                stack.push(*segment);
            }
        }
    }

    // skip one line
    let _ = reader.read_line(&mut String::new());

    // parse moves
    let mut moves: Vec<Move> = Vec::new();
    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Err(_) => panic!(),
            Ok(_n) => {
                let info: Vec<i32> = buf.split(' ')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                moves.push(
                    Move {
                        qty: info[0],
                        start: info[1],
                        end: info[2],
                    }
                );
            }
        }
    };

    (stacks, moves)
}


pub fn supply_stacks_part_1(file_name: &str) -> String {
    read_input(file_name);

    String::new()
}