use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq)]
enum OperationKind {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl FromStr for OperationKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "+" => OperationKind::Addition,
            "-" => OperationKind::Subtraction,
            "*" => OperationKind::Multiplication,
            "/" => OperationKind::Division,
            _ => panic!()
        };

        Ok(op)
    }
}


#[derive(Debug)]
enum Yell {
    Number(u32),
    Function(String, OperationKind, String),
}

#[derive(Clone, Debug, PartialEq)]
enum PostFixNotationElem {
    Variable(String),
    Number(u32),
    Operation(OperationKind),
}

fn read_input(file_name: &str) -> HashMap<String, Yell> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut instructions: HashMap<String, Yell> = HashMap::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }
        // buf.trim();

        let (monkey_name, yell) = buf.split_once(':').unwrap();

        let mut split = yell.trim().split(' ');
        let yell = if split.clone().count() == 1 {
            let number = split.next().unwrap().parse::<u32>().unwrap();
            Yell::Number(number)
        } else {
            let name_1 = split.next().unwrap().to_string();
            let op = split.next().unwrap().parse::<OperationKind>().unwrap();
            let name_2 = split.next().unwrap().to_string();
            Yell::Function(name_1, op, name_2)
        };

        instructions.insert(monkey_name.to_string(), yell);

        buf = String::new();
    };

    instructions
}

pub fn monkey_math_part_1(file_name: &str) -> i64 {
    let input = read_input(file_name);

    let mut postfix_notation: Vec<PostFixNotationElem> = Vec::new();

    // println!("{:#?}", input);

    let root = &input["root"];
    match root {
        Yell::Number(val) => return *val as i64,
        Yell::Function(name1, op, name2) => {
            postfix_notation.push(PostFixNotationElem::Variable(name1.to_string()));
            postfix_notation.push(PostFixNotationElem::Variable(name2.to_string()));
            postfix_notation.push(PostFixNotationElem::Operation(*op));
        }
    }

    'outer: loop {
        for (index, elem) in postfix_notation.clone().iter().enumerate() {
            match elem {
                PostFixNotationElem::Variable(name) => {
                    postfix_notation.remove(index);

                    match &input[name] {
                        Yell::Number(val) => postfix_notation.insert(index, PostFixNotationElem::Number(*val)),
                        Yell::Function(name1, op, name2) => {
                            postfix_notation.insert(index, PostFixNotationElem::Variable(name1.to_string()));
                            postfix_notation.insert(index + 1, PostFixNotationElem::Variable(name2.to_string()));
                            postfix_notation.insert(index + 2, PostFixNotationElem::Operation(*op));
                        }
                    }
                    continue 'outer;
                }
                _ => continue,
            }
        }

        break;
    }

    // println!("{:?}", postfix_notation);

    calculate_postfix(&postfix_notation)
}

fn calculate_postfix(postfix_notation: &[PostFixNotationElem]) -> i64 {
    let mut stack: Vec<i64> = Vec::new();
    for elem in postfix_notation {
        match elem {
            PostFixNotationElem::Number(val) => stack.push(*val as i64),
            PostFixNotationElem::Operation(op) => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();

                match op {
                    OperationKind::Addition => stack.push(num2 + num1),
                    OperationKind::Subtraction => stack.push(num2 - num1),
                    OperationKind::Multiplication => stack.push(num2 * num1),
                    OperationKind::Division => stack.push(num2 / num1),
                }
            }
            PostFixNotationElem::Variable(_) => panic!("there should be no variables at this point"),
        }
    }
    stack.pop().unwrap()
}