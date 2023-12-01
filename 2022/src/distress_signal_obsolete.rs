use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::PeekingNext;

#[derive(Copy, Clone)]
struct Info {
    val: i32,
    level: u32,
}

impl Debug for Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.val, self.level)
    }
}

#[derive(Clone)]
struct InputPair {
    left: Vec<Info>,
    right: Vec<Info>,
}

impl Debug for InputPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{:?}\n{:?}", self.left, self.right)
    }
}

fn read_input(file_name: &str) -> (Vec<InputPair>, Vec<(String, String)>) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut pairs = Vec::new();
    let mut raw_pairs = Vec::new();

    loop {
        let mut left = String::new();
        let mut right = String::new();

        let _ = reader.read_line(&mut left).unwrap();
        let _ = reader.read_line(&mut right).unwrap();

        raw_pairs.push((left.clone(), right.clone()));

        // println!("{}", left.trim());
        // println!("{}", right.trim());

        let left: Vec<Info> = parse_packet(left.trim());
        let right: Vec<Info> = parse_packet(right.trim());

        // println!("{left:?}");
        // println!("{right:?}");
        // println!();

        pairs.push(InputPair { left, right });

        if let Ok(n) = reader.read_line(&mut String::new()) {
            if n == 0 {
                break;
            };
        }
    }

    (pairs, raw_pairs)
}

fn parse_packet(line: &str) -> Vec<Info> {
    let mut level = 0_u32;
    let mut infos = Vec::new();

    let mut iterator = line.chars();
    while let Some(char) = iterator.next() {
        match char {
            '[' => {
                level += 1;

                let closing_bracket = iterator.peeking_next(|&x| x == ']');
                match closing_bracket {
                    None => {}
                    Some(_) => {
                        infos.push(Info { val: -1, level });
                        level -= 1;
                    }
                };
            }
            ']' => level -= 1,
            ',' => continue,
            '0'..='9' => {
                let mut number = iterator.clone().take_while(|ch| ch.is_ascii_digit()).collect::<Vec<_>>();
                number.insert(0, char);

                let val = String::from_iter(number).parse::<i32>().unwrap();
                let info = Info { val, level };

                infos.push(info);
            }
            _ => {}
        }
    }
    assert_eq!(level, 0);

    infos
}

pub fn distress_signal_part_1(file_name: &str) -> usize {
    let (input, raw_input) = read_input(file_name);
    let mut indices_sum = 0;

    let mut correct = Vec::new();

    for (index, pair) in input.into_iter().enumerate() {
        let (s1, s2) = &raw_input[index];
        print!("\n\n\n{s1}{s2}");

        if check_order(&pair, index) {
            correct.push((index, pair.clone()));

            indices_sum += index + 1;
        };
    }

    indices_sum
}

fn check_order(pair: &InputPair, counter: usize) -> bool {
    print!("\nchecking {} pair: {:?}", counter, &pair);

    let mut left_iter = pair.left.iter();
    let mut right_iter = pair.right.iter();
    let mut take_from_left = true;
    let mut take_from_right = true;
    let mut elem_l = Info { val: -2, level: 2 };
    let mut elem_r = Info { val: -2, level: 2 };

    loop {
        if take_from_left {
            elem_l = match left_iter.next() {
                None => {
                    println!("CORRECT - Left side ran out of items, so inputs are in the right order");
                    return true;
                }
                Some(t) => {
                    println!("  took {t:?} from left");
                    *t
                }
            };
        }
        if take_from_right {
            elem_r = match right_iter.next() {
                None => {
                    println!("INCORRECT - Right side ran out of items, so inputs are not in the right order");
                    return false;
                }
                Some(t) => {
                    println!("  took {t:?} from right");
                    *t
                }
            };
        }
        assert!(elem_l.val >= -1);
        assert!(elem_r.val >= -1);

        if elem_l.val == -1 && elem_r.val != -1 {
            take_from_left = true;
            take_from_right = false;
            continue;
        }
        if elem_l.val != -1 && elem_r.val == -1 {
            take_from_left = false;
            take_from_right = true;
            continue;
        }
        if elem_l.val == -1 && elem_r.val == -1 {
            match elem_l.level.cmp(&elem_r.level) {
                Ordering::Equal => {
                    take_from_left = true;
                    take_from_right = true;
                    continue;
                }
                Ordering::Less => {
                    take_from_left = true;
                    take_from_right = false;
                    continue;
                }
                Ordering::Greater => {
                    take_from_left = false;
                    take_from_right = true;
                    continue;
                }
            }
        }

        println!("  comparing {:?} with {:?}", elem_l, elem_r);
        println!("------");
        match elem_l.val.cmp(&elem_r.val) {
            Ordering::Equal => {
                take_from_left = true;
                take_from_right = true;
                continue;
            }
            Ordering::Greater => {
                println!("INCORRECT - because left({}) > right({})", elem_l.val, elem_r.val);
                return false;
            }
            Ordering::Less => {
                println!("CORRECT - because left({}) < right({})", elem_l.val, elem_r.val);
                return true;
            }
        }
    }
}
