use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn get_duration(&self) -> usize {
        match self {
            Instruction::Noop => 1,
            Instruction::Addx(_) => 2,
        }
    }
}

pub struct ProgramIterator {
    buf_reader: BufReader<File>,
}

impl Iterator for ProgramIterator {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self.buf_reader.read_line(&mut buf) {
            Ok(0) => None,
            Ok(_n) => {
                let result = buf.trim_end().split(' ').collect::<Vec<_>>();

                if result[0] == "noop" {
                    Some(Instruction::Noop)
                } else {
                    Some(Instruction::Addx(result[1].parse::<i32>().ok().unwrap()))
                }
            }
            Err(_e) => panic!(),
        }
    }
}

pub fn read_input(file_name: &str) -> ProgramIterator {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    ProgramIterator {
        buf_reader: reader,
    }
}

pub fn cathode_ray_tube_part_1(file_name: &str) -> i32 {
    let mut result = Vec::new();
    let mut registry = 1_i32;
    let mut clock = 1_usize;
    let mut curr_instruction: Option<Instruction> = None;
    let mut curr_instruction_progress = 0_usize;
    let mut update_value = 0_i32;

    let mut crt_pos = 0_usize;
    let mut sprite_pos = 1_i32;

    let mut input_iterator = read_input(file_name);

    loop {
        if update_value != 0 {
            registry += update_value;
            sprite_pos += update_value;
        }
        update_value = 0;

        match curr_instruction {
            None => {
                if let Some(ins) = input_iterator.next() {
                    curr_instruction = Some(ins);
                    curr_instruction_progress = ins.get_duration();
                    continue;
                } else {
                    break;
                }
            }
            Some(ins) => {
                curr_instruction_progress -= 1;

                if curr_instruction_progress == 0 {
                    match ins {
                        Instruction::Noop => update_value = 0,
                        Instruction::Addx(value) => update_value = value,
                    }

                    curr_instruction = None
                }
            }
        }

        if (clock + 20) % 40 == 0 {
            result.push(clock as i32 * registry);
        }

        if (sprite_pos - 1..=sprite_pos + 1).contains(&(crt_pos as i32)) {
            print!("#");
        } else {
            print!(".");
        }

        if clock % 40 == 0 {
            println!()
        }

        clock += 1;
        crt_pos = (crt_pos + 1) % 40;
    }
    println!("{:?}", result);

    result.iter().take(6).sum()
}
