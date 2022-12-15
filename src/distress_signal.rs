#![allow(dead_code)]

use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::ops::Add;

use itertools::{EitherOrBoth, Itertools};
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
            None => "E".to_string().add("\t\t#").add(&self.id.to_string()[..3]),
            Some(val) => val.to_string().add("\t\t#").add(&self.id.to_string()[..3]),
        };
        write!(f, "{}", style.paint(text))
    }
    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(self.children.clone())
    }
}

fn read_input(file_name: &str) -> Vec<(Node, Node)> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);

    let mut pairs = Vec::new();

    loop {
        let mut left = String::new();
        let mut right = String::new();

        let _ = reader.read_line(&mut left).unwrap();
        let _ = reader.read_line(&mut right).unwrap();

        let left: Node = parse_tree(left.trim());
        let right: Node = parse_tree(right.trim());

        pairs.push((left, right));

        if let Ok(n) = reader.read_line(&mut String::new()) {
            if n == 0 {
                break;
            };
        }
    }

    pairs
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

struct Answer {
    is_in_order: bool,
    reason: String,
}

fn double_dfs_recursion(
    current_l: &Node,
    current_r: &Node,
    visited_l: &mut Vec<Uuid>,
    visited_r: &mut Vec<Uuid>,
    siblings_counts_l: &HashMap<Uuid, usize>,
    siblings_counts_r: &HashMap<Uuid, usize>,
) -> Option<Answer> {
    if visited_l.contains(&current_l.id) || visited_r.contains(&current_r.id) { // ?
        return None;
    }
    visited_l.push(current_l.id);
    visited_r.push(current_r.id);

    match (current_l.value, current_r.value) {
        (Some(val_l), Some(val_r)) => {
            print!("\t");
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
        (None, None) => {
            println!("left(None) == right(None), do nothing...");
        }
        (None, Some(val_r)) => {
            print!("comparing: None with {:?}", current_r);
            if siblings_counts_r[&current_r.id] == 0 {
                print!("\t\t{val_r} CAN be converted");

                if current_l.children.iter().filter(|&node| node.value.is_some()).count() == 0 {
                    let string = "CORRECT - Left side ran out of items, so inputs are in the right order".to_string();
                    println!("{}", string);
                    return Some(Answer { is_in_order: true, reason: string });
                }

                for child in current_l.children.iter() {
                    if let Some(val) = child.value {
                        match val.cmp(&val_r) {
                            Ordering::Equal => { break; }
                            Ordering::Less => {
                                let string = format!("CORRECT - because left({}) <= right({})", val, val_r);
                                println!("{}", string);
                                return Some(Answer { is_in_order: true, reason: string });
                            }
                            Ordering::Greater => {
                                let string = format!("INCORRECT - because left({}) > right({})", val, val_r);
                                println!("{}", string);
                                return Some(Answer { is_in_order: false, reason: string });
                            }
                        }
                    } else {
                        continue;
                    }
                }
            }
            println!();
        }
        (Some(val_l), None) => {
            print!("comparing: {:?} with None", current_l);
            if siblings_counts_l[&current_l.id] == 0 {
                print!("\t\t{val_l} CAN be converted");
                if current_l.children.iter().filter(|&node| node.value.is_some()).count() == 0 {
                    let string = "INCORRECT - Right side ran out of items, so inputs are not in the right order".to_string();
                    println!("{}", string);
                    return Some(Answer { is_in_order: false, reason: string });
                }

                for child in current_r.children.iter() {
                    if let Some(val) = child.value {
                        match val_l.cmp(&val) {
                            Ordering::Equal => { break; }
                            Ordering::Less => {
                                let string = format!("CORRECT - because left({}) <= right({})", val_l, val);
                                println!("{}", string);
                                return Some(Answer { is_in_order: true, reason: string });
                            }
                            Ordering::Greater => {
                                let string = format!("INCORRECT - because left({}) > right({})", val_l, val);
                                println!("{}", string);
                                return Some(Answer { is_in_order: false, reason: string });
                            }
                        }
                    } else {
                        continue;
                    }
                }
            }
            println!();
        }
    }

    for item in current_l.children.iter().zip_longest(current_r.children.iter()) {
        match item {
            EitherOrBoth::Both(child_l, child_r) => {
                let answer = double_dfs_recursion(child_l, child_r, visited_l, visited_r, siblings_counts_l, siblings_counts_r);
                if answer.is_some() {
                    return answer;
                };
            }
            EitherOrBoth::Left(_) => {
                let string = "INCORRECT - Right side ran out of items, so inputs are not in the right order".to_string();
                println!("{}", string);
                return Some(Answer { is_in_order: false, reason: string });
            }
            EitherOrBoth::Right(_) => {
                let string = "CORRECT - Left side ran out of items, so inputs are in the right order".to_string();
                println!("{}", string);
                return Some(Answer { is_in_order: true, reason: string });
            }
        }
    }

    None
}

fn dfs_count_siblings(root: &Node) -> HashMap<Uuid, usize> {
    let mut stack = VecDeque::new();
    let mut visited = Vec::new();

    let mut siblings_counts = HashMap::new();

    stack.push_front(root);

    while !stack.is_empty() {
        let current_node = stack.pop_front().unwrap();

        if !visited.contains(&current_node) {
            visited.push(current_node);

            let siblings = current_node.children.iter().filter(|&node| node.value.is_some()).count();

            for child in current_node.children.iter().rev() {
                if child.value.is_some() {
                    siblings_counts.insert(child.id, siblings - 1);
                }
                stack.push_front(child);
            }
        }
    }

    siblings_counts
}

type Depth = usize;
type SiblingsNum = usize;

fn dfs_leafs_in_order(root: &Node) -> Vec<(i32, Depth, SiblingsNum)> {
    let mut stack = VecDeque::new();
    let mut data: Vec<(i32, Depth, SiblingsNum)> = Vec::new();

    stack.push_front((root, 0_usize, root.children.len()));

    while !stack.is_empty() {
        let (current_node, depth, mut num) = stack.pop_front().unwrap();

        for child in current_node.children.iter().rev() {
            stack.push_front((child, depth + 1, current_node.children.len()));
        }

        num = if num > 0 { num - 1 } else { 0 };
        if let Some(val) = current_node.value {
            data.push((val as i32, depth, num));
        } else {
            data.push((-1, depth, num));
        }
    }

    data
}

fn double_dfs_new(root_l: &Node, root_r: &Node) -> bool {
    let mut stack_l = VecDeque::new();
    let mut stack_r = VecDeque::new();

    stack_l.push_front((root_l, 0_usize, root_l.children.len()));
    stack_r.push_front((root_r, 0_usize, root_l.children.len()));

    let mut must_be_comparable = false;
    loop {
        if stack_l.is_empty() {
            println!("left tree - empty");
            return true;
        }
        if stack_r.is_empty() {
            println!("right tree - empty");
            return false;
        }

        let (curr_l, depth_l, sib_l) = stack_l.pop_front().unwrap();
        let (curr_r, depth_r, sib_r) = stack_r.pop_front().unwrap();

        for child in curr_l.children.iter().rev() {
            stack_l.push_front((child, depth_l + 1, curr_l.children.len()));
        }
        for child in curr_r.children.iter().rev() {
            stack_r.push_front((child, depth_r + 1, curr_r.children.len()));
        }

        println!("current left: {:?} (parent: )\t current right: {:?} (parent: )", curr_l.value, curr_r.value);
        match (curr_l.value, curr_r.value) {
            (Some(l), Some(r)) => {
                must_be_comparable = false;

                if depth_l != depth_r {
                    println!("WRONG - different depths");
                    return false;
                }
                match l.cmp(&r) {
                    Ordering::Equal => { continue; }
                    Ordering::Less => {
                        println!("CORRECT - because left({}) <= right({})", l, r);
                        return true;
                    }
                    Ordering::Greater => {
                        println!("WRONG - because left({}) > right({})", l, r);
                        return false;
                    }
                }
            }
            (None, None) => {
                println!("this node_r id: {:?}", curr_r.id);
                continue;
            }
            (Some(_), None) => {
                if must_be_comparable {
                    println!("cannot move second time");
                    return false;
                }

                if depth_l != depth_r {
                    println!("WRONG - different depths");
                    return false;
                }

                println!("move to next turn");
                stack_l.push_front((curr_l, depth_l + 1, 0));
                must_be_comparable = true;
                continue;
            }
            (None, Some(_)) => {
                if must_be_comparable {
                    println!("cannot move second time");
                    return false;
                }
                if depth_l != depth_r {
                    println!("WRONG - different depths");
                    return false;
                }

                println!("move to next turn");
                stack_r.push_front((curr_r, depth_r + 1, 0));
                must_be_comparable = true;
                continue;
            }
        }
    }
}

pub fn distress_signal_part_1(file_name: &str) -> usize {
    let input = read_input(file_name);
    let mut indices_sum = 0;

    for (index, (node_l, node_r)) in input.into_iter().enumerate() {
        let _ = print_tree(&node_l);
        let _ = print_tree(&node_r);

        let is_in_order = double_dfs_new(&node_l, &node_r);
        println!("pair nr {index}");
        if is_in_order {
            indices_sum += index + 1
        }
        // let leafs_in_order_l = dfs_leafs_in_order(&node_l);
        // let leafs_in_order_r = dfs_leafs_in_order(&node_r);
        // println!("{:?}\n", leafs_in_order_l);
        // println!("{:?}\n", leafs_in_order_r);


        // let is_in_oreder: bool = compare(leafs_in_order_l, leafs_in_order_r);

        // let siblings_count_l = dfs_count_siblings(&node_l);
        // let siblings_count_r = dfs_count_siblings(&node_r);
        //
        // for (id, num) in siblings_count_l.iter() {
        //     print!("({}: {}) ", &id.to_string()[..3], num);
        // }
        // println!();
        // for (id, num) in siblings_count_r.iter() {
        //     print!("({}: {}) ", &id.to_string()[..3], num);
        // }
        // println!();
        //
        // let mut visited_l = Vec::new();
        // let mut visited_r = Vec::new();
        //
        // let answer = double_dfs_recursion(&node_l, &node_r, &mut visited_l, &mut visited_r, &siblings_count_l, &siblings_count_r);
        // if answer.unwrap().is_in_order {
        //     indices_sum += index + 1
        // }
    }

    indices_sum
}

fn compare(leafs_l: &[(i32, Depth, SiblingsNum)], leafs_r: &[(i32, Depth, SiblingsNum)]) -> bool {
    for item in leafs_l.iter().zip_longest(leafs_r.iter()) {
        match item {
            EitherOrBoth::Both((val_l, depth_l, sib_l), (val_r, depth_r, sib_r)) => {
                // if *val_l < 0 && *val_r > 0 {
                //     if val_
                // }

                if *depth_l == *depth_r {
                    // compare val_l with val_r
                    continue;
                }

                if depth_l.abs_diff(*depth_r) == 1 {}
            }
            EitherOrBoth::Left(_) => {
                let string = "INCORRECT - Right side ran out of items, so inputs are not in the right order".to_string();
                println!("{}", string);
                // return Some(Answer { is_in_order: false, reason: string });
            }
            EitherOrBoth::Right(_) => {
                let string = "CORRECT - Left side ran out of items, so inputs are in the right order".to_string();
                println!("{}", string);
                // return Some(Answer { is_in_order: true, reason: string });
            }
        }
    }

    todo!()
}
