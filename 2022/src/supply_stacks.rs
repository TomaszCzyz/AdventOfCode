use std::fs::File;
use std::io::{BufRead, BufReader};

type Stacks = Vec<Vec<char>>;

#[derive(Debug)]
struct Move {
    qty: usize,
    start: usize,
    end: usize,
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
                        qty: info[0] as usize,
                        start: (info[1] - 1) as usize,
                        end: (info[2] - 1) as usize,
                    }
                );
            }
        }
    };

    // println!("{:?}", stacks);
    // println!("{:?}", moves);
    (stacks, moves)
}


pub fn supply_stacks_part_1(file_name: &str) -> String {
    let (mut stacks, moves) = read_input(file_name);

    for move_ in moves.iter() {
        for _ in 0..move_.qty {
            let elem = stacks[move_.start].pop().unwrap();
            stacks[move_.end].push(elem);
        }
    }


    let mut result = Vec::new();
    for stack in stacks.iter() {
        result.push(stack.last().copied().unwrap())
    }

    println!("{:?}", stacks);

    String::from_iter(result)
}

pub fn supply_stacks_part_2(file_name: &str) -> String {
    let (mut stacks, moves) = read_input(file_name);

    for move_ in moves.iter() {
        let start_stack_len = stacks[move_.start].len();
        let elements = stacks[move_.start].split_off(start_stack_len - move_.qty);

        stacks[move_.end].extend(elements);
    }

    let mut result = Vec::new();

    for stack in stacks.iter() {
        result.push(stack.last().copied().unwrap())
    }

    println!("{:?}", stacks);

    String::from_iter(result)
}