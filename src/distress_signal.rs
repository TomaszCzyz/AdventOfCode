#![allow(dead_code)]

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use itertools::PeekingNext;
use ptree::{print_tree, Style, TreeItem};
use uuid::Uuid;

#[derive(Clone, Debug)]
struct Node {
    id: Uuid,
    value: Option<u32>,
    children: Vec<Node>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.value == other.value && self.children == other.children
    }
}

impl TreeItem for Node {
    type Child = Self;

    fn write_self<W: Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        let text = match self.value {
            None => "E".to_string(),
            Some(val) => val.to_string()
        };
        write!(f, "{}", style.paint(text))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(self.children.clone())
    }
}

fn read_input(file_name: &str) -> (Vec<(Node, Node)>, Vec<(String, String)>) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut pairs = Vec::new();
    let mut raw_view = Vec::new();

    loop {
        let mut left = String::new();
        let mut right = String::new();

        let _ = reader.read_line(&mut left).unwrap();
        let _ = reader.read_line(&mut right).unwrap();

        raw_view.push((left.clone(), right.clone()));

        let left: Node = parse_tree(left.trim());
        let right: Node = parse_tree(right.trim());

        pairs.push((left, right));

        if let Ok(n) = reader.read_line(&mut String::new()) {
            if n == 0 {
                break;
            };
        }
    }

    (pairs, raw_view)
}

fn find_closing_bracket(line: &str) -> usize {
    let mut level = 0;
    let mut closing_bracket_index = 0;
    for (index, char) in line.chars().skip(1).enumerate() {
        if char == ']' {
            if level == 0 {
                closing_bracket_index = index;
                break;
            } else {
                level -= 1;
            }
        }

        if char == '[' {
            level += 1
        }
    }
    assert_ne!(closing_bracket_index, 0);

    closing_bracket_index + 1
}

fn my_split(line: &str) -> Vec<&str> {
    let mut level = 0;
    let mut parts = Vec::new();
    let mut begin = 0;

    for (i, char) in line.chars().enumerate() {
        if char == ',' && level == 0 {
            parts.push(&line[begin..i]);
            begin = i + 1;
        };

        if char == '[' {
            level += 1;
            continue;
        }

        if char == ']' {
            level -= 1;
            continue;
        }
    }
    parts.push(&line[begin..]);

    parts
}

fn parse_tree(line: &str) -> Node {
    let mut parent = Node {
        id: Uuid::new_v4(),
        value: None,
        children: vec![],
    };
    let slice = &line[1..line.len() - 1];

    for part in my_split(slice).into_iter() {
        if part.starts_with('[') {
            let node = parse_tree(part);

            parent.children.push(node);
        } else if !part.is_empty() {
            let num = part.parse::<u32>().unwrap();
            let node = Node { id: Uuid::new_v4(), value: Option::from(num), children: vec![] };

            parent.children.push(node);
        }
    }

    parent
}

fn dfs(root: &Node) -> Vec<i32> {
    let mut ordered = Vec::new();
    let mut stack = VecDeque::new();
    let mut visited = Vec::new();

    stack.push_front(root);

    while !stack.is_empty() {
        let current_node = stack.pop_front().unwrap();

        if !visited.contains(&current_node) {
            if let Some(val) = current_node.value {
                print!("{:?} ", val);
                ordered.push(val as i32);
            } else {
                print!("E ");
                ordered.push(-1);
            }

            visited.push(current_node);

            for child in current_node.children.iter().rev() {
                stack.push_front(child);
            }
        }
    }
    println!();

    ordered
}

struct Answer {
    is_in_order: bool,
    reason: String,
}

fn double_dfs_recursion(
    current_l: &Node,
    current_r: &Node,
    visited_l: &mut Vec<Uuid>,
    visited_r: &mut Vec<Uuid>,
) -> Option<Answer> {
    if visited_l.contains(&current_l.id) || visited_r.contains(&current_r.id) { // ?
        return None;
    }
    visited_l.push(current_l.id);
    visited_r.push(current_r.id);

    if let Some(val_l) = current_l.value {
        if let Some(val_r) = current_r.value {
            match val_l.cmp(&val_r) {
                Ordering::Equal => {
                    println!("left({:?}) == right({:?}), do nothing...", val_l, val_r);
                }
                Ordering::Less => {
                    let string = format!("CORRECT - because left({}) <= right({})", val_l, val_r);
                    println!("{}", string);
                    return Some(Answer { is_in_order: true, reason: string });
                }
                Ordering::Greater => {
                    let string = format!("INCORRECT - because left({}) > right({})", val_l, val_r);
                    println!("{}", string);
                    return Some(Answer { is_in_order: false, reason: string });
                }
            }
        }
    }

    let mut iter_l = current_l.children.iter();
    let mut iter_r = current_r.children.iter();
    loop {
        if let Some(child_l) = iter_l.next() {
            if let Some(child_r) = iter_r.next() {
                let answer = double_dfs_recursion(child_l, child_r, visited_l, visited_r);
                if answer.is_some() {
                    return answer;
                };
            } else {
                let string = "INCORRECT - Right side ran out of items, so inputs are not in the right order".to_string();
                println!("{}", string);
                return Some(Answer { is_in_order: false, reason: string) });
            }
        } else if let Some(_next_node) = iter_r.peeking_next(|_| true) {
            let string = "CORRECT - Left side ran out of items, so inputs are in the right order".to_string();
            println!("{}", string);
            return Some(Answer { is_in_order: true, reason: string });
        } else {
            // println!("same number of children");
            break;
        }
    }
    None
}

pub fn distress_signal_part_1(file_name: &str) -> usize {
    let (input, raw_input) = read_input(file_name);
    let mut indices_sum = 0;

    for (node1, node2) in input {
        let _ = print_tree(&node1);
        let _ = print_tree(&node2);

        let mut visited_l = Vec::new();
        let mut visited_r = Vec::new();

        double_dfs_recursion(&node1, &node2, &mut visited_l, &mut visited_r);
    }

    indices_sum
}
