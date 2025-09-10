#![allow(dead_code)]

use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use itertools::{EitherOrBoth, Itertools};
use ptree::{print_tree, Style, TreeItem};

#[derive(Clone)]
struct Node {
    value: Option<u32>,
    children: Vec<Node>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self.value {
            None => "E".to_string(),
            Some(val) => val.to_string()
        };

        if self.children.is_empty() {
            write!(f, "{} ", text)
        } else {
            write!(f, "{} {:?}", text, self.children)
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.children == other.children
    }
}

impl TreeItem for Node {
    type Child = Self;

    fn write_self<W: Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        let text = match self.value {
            None => "E".to_string(),
            Some(val) => val.to_string(),
        };
        write!(f, "{}", style.paint(text))
    }
    fn children(&'_ self) -> Cow<'_, [Self::Child]> {
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
            let node = Node { value: Option::from(num), children: vec![] };

            parent.children.push(node);
        }
    }

    parent
}

fn compare(curr_l: &Node, curr_r: &Node) -> Ordering {
    match (curr_l.value, curr_r.value) {
        (Some(val_l), Some(val_r)) => {
            // if both have value, then both have no children
            val_l.cmp(&val_r)
        }
        (None, None) => {
            for pair in curr_l.children.iter().zip_longest(curr_r.children.iter()) {
                let result = match pair {
                    EitherOrBoth::Both(child_l, child_r) => compare(child_l, child_r),
                    EitherOrBoth::Left(_) => Ordering::Greater, // right run out of items
                    EitherOrBoth::Right(_) => Ordering::Less, // left run out of items
                };

                if result == Ordering::Equal {
                    continue;
                } else {
                    return result;
                }
            }
            // empty leaves
            Ordering::Equal
        }
        (None, Some(val_r)) => {
            let node_copy = Node {
                value: Some(val_r),
                children: vec![],
            };

            let extended_child = Node {
                value: None,
                children: vec![node_copy],
            };

            compare(curr_l, &extended_child)
        }
        (Some(val_l), None) => {
            let node_copy = Node {
                value: Some(val_l),
                children: vec![],
            };

            let extended_child = Node {
                value: None,
                children: vec![node_copy],
            };
            compare(&extended_child, curr_r)
        }
    }
}

pub fn distress_signal_part_1(file_name: &str) -> usize {
    let input = read_input(file_name);
    let mut indices_sum = 0;

    for (index, (node_l, node_r)) in input.into_iter().enumerate() {
        let _ = print_tree(&node_l);
        let _ = print_tree(&node_r);

        let is_in_order = compare(&node_l, &node_r);

        if is_in_order == Ordering::Equal || is_in_order == Ordering::Less {
            indices_sum += index + 1
        }
    }

    indices_sum
}

pub fn distress_signal_part_2(file_name: &str) -> usize {
    let mut input = read_input(file_name);
    let mut indices_product = 1;

    let (node_1, node_2) = create_extra_nodes();
    input.push((node_1.clone(), node_2.clone()));

    let sorted = input.into_iter()
        .flat_map(|(node_l, node_r)| [node_l, node_r])
        .sorted_by(compare)
        .collect::<Vec<_>>();

    for (index, node) in sorted.iter().enumerate() {
        if *node == node_1 {
            indices_product *= index + 1;
        }
        if *node == node_2 {
            indices_product *= index + 1;
        }
        println!("{:?}", node);
    }

    indices_product
}

fn create_extra_nodes() -> (Node, Node) {
    (Node {
        value: None,
        children: vec![
            Node {
                value: None,
                children: vec![
                    Node {
                        value: Some(2),
                        children: vec![],
                    }
                ],
            }
        ],
    },
     Node {
         value: None,
         children: vec![
             Node {
                 value: None,
                 children: vec![
                     Node {
                         value: Some(6),
                         children: vec![],
                     }
                 ],
             }
         ],
     })
}
