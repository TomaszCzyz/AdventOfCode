use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub enum Choice {
    Left,
    Right,
}

pub struct NodeConnections {
    node: String,
    left: String,
    right: String,
}

pub fn read_input(file_name: &str) -> (Vec<Choice>, Vec<NodeConnections>) {
    let file = std::fs::File::open(file_name).unwrap();
    let mut reader = std::io::BufReader::new(file);

    let mut buf = String::new();
    _ = reader.read_line(&mut buf).unwrap();

    let choices = buf.trim()
        .chars()
        .map(|ch| if ch == 'R' { Choice::Right } else { Choice::Left })
        .collect::<Vec<_>>();

    // skip empty line
    _ = reader.read_line(&mut buf).unwrap();

    let mut nodes = Vec::new();
    let mut buf = String::new();
    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let node = NodeConnections {
            node: buf[..=2].to_string(),
            left: buf[7..=9].to_string(),
            right: buf[12..=14].to_string(),
        };

        nodes.push(node);
        buf = String::new();
    }

    (choices, nodes)
}

type AdjacencyList = Vec<HashSet<usize>>;

fn make_graph(node_connections: &Vec<NodeConnections>) -> AdjacencyList {
    let nodes_map = node_connections.iter()
        .enumerate()
        .map(|(i, node)| (node.node.clone(), i))
        .collect::<HashMap<_, _>>();

    let mut adj_list = (0..nodes_map.len())
        .map(|_| HashSet::new())
        .collect::<Vec<_>>();

    for node in node_connections.iter() {
        let set = &mut adj_list[nodes_map[&node.node]];
        set.insert(nodes_map[&node.left]);
        set.insert(nodes_map[&node.right]);
    }

    adj_list
}

fn make_jump_array(node_connections: Vec<NodeConnections>) -> Vec<[usize; 2]> {
    let nodes_map = node_connections.iter()
        .enumerate()
        .map(|(i, node)| (node.node.clone(), i))
        .collect::<HashMap<_, _>>();

    node_connections.iter()
        .map(|node| [nodes_map[&node.left], nodes_map[&node.right]])
        .collect::<Vec<_>>()
}

fn haunted_wasteland_part_1(filename: &str) -> usize {
    let (choices, node_connections) = read_input(filename);

    let nodes_number = node_connections.len();
    let jump_array = make_jump_array(node_connections);
    let mut pos = 0;
    let mut counter = 0;

    loop {
        if pos == nodes_number - 1 {
            break;
        }

        let choice = match choices[counter % choices.len()] {
            Choice::Left => 0usize,
            Choice::Right => 1usize,
        };

        pos = jump_array[pos][choice];

        counter += 1;
    }

    counter
}


fn haunted_wasteland_part_2(filename: &str) -> usize {
    let _ = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_graph_test() {
        let input = read_input("inputs/8_input_example.txt");
        let graph = make_graph(&input.1);

        println!("{graph:?}");
    }

    #[test]
    fn part_1_input_example() {
        let answer = haunted_wasteland_part_1("inputs/8_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = haunted_wasteland_part_1("inputs/8_input_example_2.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 6);
    }

    #[test]
    fn part_1_input() {
        let answer = haunted_wasteland_part_1("inputs/8_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }

    #[test]
    fn part_2_input_example() {
        let answer = haunted_wasteland_part_2("inputs/8_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }

    #[test]
    fn part_2_input() {
        let answer = haunted_wasteland_part_2("inputs/8_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }
}
