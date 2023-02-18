use std::collections::{HashMap, VecDeque};
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
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

impl Debug for OperationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationKind::Addition => write!(f, "+"),
            OperationKind::Subtraction => write!(f, "-"),
            OperationKind::Multiplication => write!(f, "*"),
            OperationKind::Division => write!(f, "/"),
        }
    }
}

#[derive(Debug)]
enum Yell {
    Number(i64),
    Function(String, OperationKind, String),
}

#[derive(Clone, PartialEq)]
enum PostFixElem {
    Variable(String),
    Number(i64),
    Operation(OperationKind),
}

impl Debug for PostFixElem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PostFixElem::Variable(name) => write!(f, "{name}"),
            PostFixElem::Number(num) => if *num != i64::MAX { write!(f, "{num}") } else { write!(f, "ME") },
            PostFixElem::Operation(op) => write!(f, "{op:?}"),
        }
    }
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
            let number = split.next().unwrap().parse::<i64>().unwrap();
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

fn initialize_postfix_notation(input: &HashMap<String, Yell>, postfix_notation: &mut Vec<PostFixElem>, root: &String) {
    match &input[root] {
        Yell::Number(_) => {}
        Yell::Function(name1, op, name2) => {
            postfix_notation.push(PostFixElem::Variable(name1.to_string()));
            postfix_notation.push(PostFixElem::Variable(name2.to_string()));
            postfix_notation.push(PostFixElem::Operation(*op));
        }
    }
}

fn calculate_postfix(postfix_notation: &[PostFixElem]) -> i64 {
    let mut stack: Vec<i64> = Vec::new();

    for elem in postfix_notation {
        match elem {
            PostFixElem::Number(val) => stack.push(*val),
            PostFixElem::Operation(op) => {
                let num1 = stack.pop().unwrap();
                let num2 = stack.pop().unwrap();

                stack.push(
                    match op {
                        OperationKind::Addition => num2 + num1,
                        OperationKind::Subtraction => num2 - num1,
                        OperationKind::Multiplication => num2 * num1,
                        OperationKind::Division => num2 / num1,
                    }
                )
            }
            PostFixElem::Variable(_) => panic!("there should be no variables at this point"),
        }
    }
    stack.pop().unwrap()
}

pub fn monkey_math_part_1(file_name: &str) -> i64 {
    let input = read_input(file_name);

    let mut postfix_notation: Vec<PostFixElem> = Vec::new();

    initialize_postfix_notation(&input, &mut postfix_notation, &"root".to_string());

    'outer: loop {
        for (index, elem) in postfix_notation.clone().iter().enumerate() {
            match elem {
                PostFixElem::Variable(name) => {
                    postfix_notation.remove(index);

                    match &input[name] {
                        Yell::Number(val) => postfix_notation.insert(index, PostFixElem::Number(*val)),
                        Yell::Function(name1, op, name2) => {
                            postfix_notation.insert(index, PostFixElem::Variable(name1.to_string()));
                            postfix_notation.insert(index + 1, PostFixElem::Variable(name2.to_string()));
                            postfix_notation.insert(index + 2, PostFixElem::Operation(*op));
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

pub fn monkey_math_part_2(file_name: &str) -> i64 {
    let input = read_input(file_name);
    let mut lhs_postfix_notation: Vec<PostFixElem> = Vec::new();
    let mut rhs_postfix_notation: Vec<PostFixElem> = Vec::new();

    // println!("{:#?}", input);

    let root = &input["root"];
    let (lhs, rhs) = match root {
        Yell::Number(val) => return *val,
        Yell::Function(name1, _op, name2) => (name1, name2),
    };

    initialize_postfix_notation(&input, &mut lhs_postfix_notation, lhs);
    initialize_postfix_notation(&input, &mut rhs_postfix_notation, rhs);

    let mut contains_me = 0;
    let mut notations = [lhs_postfix_notation, rhs_postfix_notation];

    for (notation_index, notation) in notations.iter_mut().enumerate() {
        'outer: loop {
            for (index, elem) in notation.clone().iter().enumerate() {
                match elem {
                    PostFixElem::Variable(name) => {
                        notation.remove(index);

                        if name == "humn" {
                            contains_me = notation_index;
                            notation.insert(index, PostFixElem::Number(i64::MAX));
                            continue 'outer;
                        }

                        match &input[name] {
                            Yell::Number(val) => notation.insert(index, PostFixElem::Number(*val)),
                            Yell::Function(name1, op, name2) => {
                                notation.insert(index, PostFixElem::Variable(name1.to_string()));
                                notation.insert(index + 1, PostFixElem::Variable(name2.to_string()));
                                notation.insert(index + 2, PostFixElem::Operation(*op));
                            }
                        }
                        continue 'outer;
                    }
                    _ => continue,
                }
            }

            break;
        }
    }

    let notation_with_me = &notations[contains_me];
    let notation_without_me = &notations[(contains_me + 1) % notations.len()];

    let result = calculate_postfix(notation_without_me);

    // manipulate notation to calculate my number
    let new_notation = manipulate_postfix_notation(notation_with_me, result);

    calculate_postfix(&new_notation)
}

fn manipulate_postfix_notation(input_notation: &Vec<PostFixElem>, desired_result: i64) -> Vec<PostFixElem> {
    let mut new_postfix_notation: VecDeque<PostFixElem> = VecDeque::from([PostFixElem::Number(desired_result)]);
    let mut notation_clone: VecDeque<PostFixElem> = VecDeque::from(input_notation.clone());

    // pop last element, which always should be of type 'Operator'
    while let PostFixElem::Operation(op) = notation_clone.pop_back().unwrap() {

        // find the components of the operator
        let mut nesting_counter = 1_usize;
        let mut index = notation_clone.len();
        while nesting_counter != 0 {
            let elem = &notation_clone[index - 1];
            match elem {
                PostFixElem::Number(_) => nesting_counter -= 1,
                PostFixElem::Operation(_) => nesting_counter += 1,
                PostFixElem::Variable(_) => {}
            }
            index -= 1;
        }

        // determine which component contains 'me'
        let mut left_contains_me = false;
        for elem in notation_clone.iter().take(index) {
            if let PostFixElem::Number(num) = elem {
                if *num == i64::MAX {
                    left_contains_me = true;
                    break;
                }
            }
        }

        // split elements into two components
        let right_component = notation_clone.split_off(index);
        let left_component = notation_clone.clone();

        // point to variable with elements to move, and assign the remaining elements back to 'loop variable'
        let elements_to_move = if left_contains_me {
            notation_clone = left_component;
            right_component
        } else {
            notation_clone = right_component;
            left_component
        };

        // move elements to new postfix notation and determine new operator based on component moved
        let new_op = match op {
            OperationKind::Addition => {
                for elem in elements_to_move.into_iter() {
                    new_postfix_notation.push_back(elem);
                }

                OperationKind::Subtraction
            }
            OperationKind::Multiplication => {
                for elem in elements_to_move.into_iter() {
                    new_postfix_notation.push_back(elem);
                }

                OperationKind::Division
            }
            OperationKind::Subtraction => {
                for elem in elements_to_move.into_iter().rev() {
                    new_postfix_notation.push_front(elem);
                }

                if left_contains_me { OperationKind::Addition } else { OperationKind::Subtraction }
            }
            OperationKind::Division => {
                for elem in elements_to_move.into_iter().rev() {
                    new_postfix_notation.push_front(elem);
                }

                if left_contains_me { OperationKind::Multiplication } else { OperationKind::Division }
            }
        };

        new_postfix_notation.push_back(PostFixElem::Operation(new_op))
    }

    new_postfix_notation.into()
}
