use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn read_input(file_name: &str) -> Vec<Monkey> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut monkeys = Vec::new();

    loop {
        let mut items: Vec<u64> = Vec::new();
        let mut operation: Operation = Operation { type_: OperationType::Add, elem: Some(0) };
        let mut divisor: u64 = 0;
        let mut true_target: usize = 0;
        let mut false_target: usize = 0;

        for _ in 0..6 {
            let mut buf = String::new();
            let _ = reader.read_line(&mut buf).unwrap();
            buf = buf.trim().to_string();

            if buf.starts_with("Starting items:") {
                items = buf.trim_start_matches("Starting items: ")
                    .split(", ")
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
            } else if buf.starts_with("Operation: ") {
                operation = parse_operation(buf.trim_start_matches("Operation: "));
            } else if buf.starts_with("Test: ") {
                divisor = buf.trim_start_matches("Test: divisible by ")
                    .parse::<u64>()
                    .unwrap();
            } else if buf.starts_with("If true: ") {
                true_target = buf.trim_start_matches("If true: throw to monkey ")
                    .parse::<usize>()
                    .unwrap();
            } else if buf.starts_with("If false: ") {
                false_target = buf.trim_start_matches("If false: throw to monkey ")
                    .parse::<usize>()
                    .unwrap();
            }
        }

        monkeys.push(Monkey {
            inspects_counter: 0,
            items,
            operation,
            test: Test {
                divisor,
                true_target,
                false_target,
            },
        });

        if let Ok(n) = reader.read_line(&mut String::new()) {
            if n == 0 {
                break;
            };
        }
    }

    monkeys
}

fn parse_operation(s: &str) -> Operation {
    let ingredients = s.trim_start_matches("new = old ").split(' ').collect::<Vec<_>>();
    let (op, second) = (ingredients[0], ingredients[1]);

    match second.parse::<u64>() {
        Ok(value) => match op {
            "+" => Operation { type_: OperationType::Add, elem: Some(value) },
            "*" => Operation { type_: OperationType::Multiply, elem: Some(value) },
            &_ => panic!()
        },
        Err(_) => match op {
            "+" => Operation { type_: OperationType::Add, elem: None },
            "*" => Operation { type_: OperationType::Multiply, elem: None },
            &_ => panic!()
        }
    }
}

#[derive(Debug)]
enum OperationType {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Operation {
    type_: OperationType,
    elem: Option<u64>,
}

impl Operation {
    fn calculate(&self, old: u64) -> u64 {
        match self.elem {
            None => match self.type_ {
                OperationType::Add => old + old,
                OperationType::Multiply => old * old,
            },
            Some(val) => match self.type_ {
                OperationType::Add => old + val,
                OperationType::Multiply => old * val,
            }
        }
    }
}

#[derive(Debug)]
struct Test {
    divisor: u64,
    true_target: usize,
    false_target: usize,
}

impl Test {
    fn test(&self, dividend: u64) -> usize {
        if dividend % self.divisor == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

#[derive(Debug)]
struct Monkey {
    inspects_counter: usize,
    items: Vec<u64>,
    operation: Operation,
    test: Test,
}

pub enum DecreaseStrategy {
    DivideByThree,
    DivideByDivisorProduct,
}

pub fn monkey_in_the_middle(file_name: &str, round_count: usize, decrease_strategy: DecreaseStrategy) -> usize {
    let mut monkeys = read_input(file_name);
    let divisors_product = monkeys.iter().map(|m| m.test.divisor).reduce(|acc, e| acc * e).unwrap();

    for round in 1..=round_count {
        for id in 0..monkeys.len() {
            let mut items_to_throw = Vec::new();
            let mut monkey = &mut monkeys[id];

            for item in monkey.items.iter() {
                monkey.inspects_counter += 1;
                let new_worry = monkey.operation.calculate(*item);

                let new_worry = match decrease_strategy {
                    DecreaseStrategy::DivideByThree => decrease_worry(new_worry),
                    DecreaseStrategy::DivideByDivisorProduct => decrease_worry_2(new_worry, divisors_product)
                };

                let throw_target = monkey.test.test(new_worry);

                items_to_throw.push((throw_target, new_worry));
            }
            monkey.items = Vec::new();

            for (target_id, item) in items_to_throw {
                monkeys[target_id].items.push(item);
            }
        }

        if [1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 10000].contains(&round) {
            print_round_summarize(round, &monkeys);
        }
    }

    monkeys.iter()
        .map(|monkey| monkey.inspects_counter)
        .sorted_by(|count1, count2| count2.cmp(count1))
        .take(2)
        .reduce(|acc, e| acc * e)
        .unwrap()
}

#[allow(dead_code)]
fn print_round_summarize(round: usize, monkeys: &[Monkey]) {
    println!("After round {round}");
    // for (i, monkey) in monkeys.iter().enumerate() {
    //     println!("Monkey {i}: {:?}", monkey.items)
    // }
    // println!();

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {i} inspected items {} times", monkey.inspects_counter)
    }
    println!();
}

fn decrease_worry(worry: u64) -> u64 {
    worry.div_euclid(3)
}

fn decrease_worry_2(worry: u64, num: u64) -> u64 {
    worry % num
}