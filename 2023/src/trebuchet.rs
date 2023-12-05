#![allow(dead_code)]

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input_part_1(file_name: &str) -> Vec<u32> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut numbers = Vec::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut iter = buf.chars();
        let mut first_digit = u32::MAX;
        let mut last_digit = u32::MAX;

        while let Some(ch) = iter.next() {
            if ch.is_digit(10) {
                first_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        while let Some(ch) = iter.next_back() {
            if ch.is_digit(10) {
                last_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        if last_digit == u32::MAX {
            last_digit = first_digit;
        }

        numbers.push(first_digit * 10 + last_digit);
        buf = String::new();
    }

    numbers
}

const DIGITS_NAMES: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
// const ALPHABET: [&str; 15] = [
//     "z",
//     "e",
//     "r",
//     "o",
//     "n",
//     "t",
//     "w",
//     "h",
//     "f",
//     "u",
//     "i",
//     "v",
//     "s",
//     "x",
//     "g"
// ];

const ALPHABET: &str = "zerontwhfuivsxg";

type NodeIndex = usize;

struct Node<'a> {
    index: NodeIndex,
    value: &'a str,
    children: Vec<NodeIndex>,
    suffix: NodeIndex,
    output: NodeIndex,
}

pub struct Trie<'a> {
    nodes: Vec<Node<'a>>,
}

impl<'a> Trie<'a> {
    fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    fn create_node(&mut self, value: &'a str) -> NodeIndex {
        let index = self.nodes.len();

        self.nodes.push(Node {
            index,
            value,
            children: Vec::new(),
            suffix: usize::MAX,
            output: usize::MAX,
        });

        return index;
    }

    fn get_root(&mut self) -> Option<&mut Node<'a>> {
        self.get_node(0)
    }

    fn get_node(&mut self, index: NodeIndex) -> Option<&mut Node<'a>> {
        self.nodes.get_mut(index)
    }

    fn add_child(&mut self, parent_id: NodeIndex, child_id: NodeIndex) {
        self.nodes[parent_id].children.push(child_id);
    }

    fn get_child_by_value(&self, val: &str) -> Option<NodeIndex> {
        Some(self.nodes.iter().find(|&x| x.value == val)?.index)
    }

    fn has_child_with_value(&self, parent: NodeIndex, val: &str) -> bool {
        if let Some(index) = self.get_child_by_value(val) {
            self.nodes[parent].children.contains(&index)
        } else {
            false
        }
    }
}


struct AhoCorasick<'a> {
    trie: Trie<'a>,
    // next_table: 
}

impl<'a> AhoCorasick<'a> {
    fn new(patterns: &[&'a str]) -> Trie<'a> {
        let mut trie = Self::build_trie(patterns);

        Self::add_suffix_and_output_links(&mut trie);

        trie
    }

    fn add_suffix_and_output_links(trie: &mut Trie) {
        let root_node = trie.get_root().unwrap();
        root_node.suffix = 0;

        let mut queue = VecDeque::new();

        for root_child_index in root_node.children.clone().into_iter() {
            let child_node = trie.get_node(root_child_index).unwrap();
            child_node.suffix = 0;

            queue.push_back(root_child_index);
        }

        while let Some(node_index) = queue.pop_front() {
            let curr_node = trie.get_node(node_index).unwrap();

            for child_index in curr_node.children.clone().into_iter() {
                let child_node = trie.get_node(child_index).unwrap();
                let _child_suffix_index = child_node.suffix;
            } 
        }
    }

    fn build_trie(patterns: &[&'a str]) -> Trie<'a> {
        let mut trie = Trie::new();
        let root_index = trie.create_node("");

        for pattern in patterns {
            let mut curr_node_index = root_index;

            for (i, _) in pattern.char_indices() {
                let letter = &pattern[i..=i];
                if trie.has_child_with_value(curr_node_index, letter) {
                    curr_node_index = trie.get_child_by_value(letter).unwrap();
                } else {
                    let new_node_index = trie.create_node(letter);
                    trie.add_child(curr_node_index, new_node_index);
                    curr_node_index = new_node_index;
                }
            }

            // todo: add "pattern ... ens here
        }

        trie
    }

    // fn next(&mut self, from: &str, letter: &str) -> Option<&str> {
    //     if from == "" {
    //         return Some(letter);
    //     }
    // 
    //     todo!()
    // }
}


pub fn read_input_part_2(file_name: &str) -> Vec<u32> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut numbers = Vec::new();

    AhoCorasick::new(&DIGITS_NAMES);

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut iter = buf.chars();
        let mut first_digit = u32::MAX;
        let mut last_digit = u32::MAX;

        while let Some(ch) = iter.next() {
            if ch.is_digit(10) {
                first_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        while let Some(ch) = iter.next_back() {
            if ch.is_digit(10) {
                last_digit = ch.to_digit(10).unwrap();
                break;
            }
        }

        if last_digit == u32::MAX {
            last_digit = first_digit;
        }

        numbers.push(first_digit * 10 + last_digit);
        buf = String::new();
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example() {
        let input = read_input_part_1("inputs/1_input_example.txt");
        let answer = input.iter().sum::<u32>();

        assert_eq!(answer, 142);
    }

    #[test]
    fn part_1_input() {
        let input = read_input_part_1("inputs/1_input.txt");
        let answer = input.iter().sum::<u32>();

        assert_eq!(answer, 54081);
    }

    #[test]
    fn part_2_input_example() {
        let input = read_input_part_2("inputs/1_input_example.txt");
        let answer = input.iter().sum::<u32>();

        assert_eq!(answer, 142);
    }
}
