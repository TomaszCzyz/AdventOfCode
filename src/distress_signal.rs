#![allow(dead_code)]

use std::borrow::Cow;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};

use ptree::{print_tree, Style, TreeItem};

#[derive(Clone, Debug)]
struct Node {
    value: Option<u32>,
    children: Vec<Node>,
}

impl TreeItem for Node {
    type Child = Self;

    fn write_self<W: Write>(&self, f: &mut W, style: &Style) -> io::Result<()> {
        let text = match self.value {
            None => "empty".to_string(),
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

pub fn distress_signal_part_1(file_name: &str) -> usize {
    let (input, raw_input) = read_input(file_name);
    let mut indices_sum = 0;

    for (node1, node2) in input {
        let _ = print_tree(&node1);
        let _ = print_tree(&node2);
        println!();
    }

    indices_sum
}
