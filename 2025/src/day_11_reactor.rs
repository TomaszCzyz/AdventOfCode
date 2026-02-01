use std::collections::{HashMap, VecDeque};
use std::fs;

type Neighbors = Vec<usize>;

// aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out
fn read_input(file_name: &str) -> (Vec<Neighbors>, HashMap<usize, String>) {
    let mut rows_parsed = Vec::<(&str, Vec<&str>)>::new();
    let mut name_to_index = HashMap::new();
    let mut index_to_name = HashMap::new();

    let content = fs::read_to_string(file_name).unwrap();

    content.lines().for_each(|line| {
        let line_elements = line.split(": ").collect::<Vec<_>>();
        let name = line_elements[0];
        let neighbors = line_elements[1].split_whitespace().collect::<Vec<_>>();

        rows_parsed.push((name, neighbors.clone()));
        index_to_name.insert(rows_parsed.len() - 1, name.to_string());
        name_to_index.insert(name.to_string(), rows_parsed.len() - 1);
    });

    index_to_name.insert(rows_parsed.len(), "out".to_string());
    name_to_index.insert("out".to_string(), rows_parsed.len());

    let mut rows = vec![vec![]; rows_parsed.len() + 1];

    for (name, neighbors) in rows_parsed.into_iter() {
        let index = name_to_index[name];
        for neighbor in neighbors {
            rows[index].push(name_to_index[neighbor]);
        }
    }

    (rows, index_to_name)
}

fn part_1(filename: &str) -> usize {
    let (input, translations) = read_input(filename);

    let end_vertex = translations.len() - 1;
    let start_vertex = *translations
        .iter()
        .find(|&(_, elem)| elem == "you")
        .unwrap()
        .0;

    let mut paths_count = 0;
    let mut queue = VecDeque::from([start_vertex]);

    while let Some(vertex) = queue.pop_front() {
        if vertex == end_vertex {
            paths_count += 1;
            continue;
        }

        for neighbor in &input[vertex] {
            queue.push_back(*neighbor);
        }
    }

    paths_count
}

fn print_input(input: &Vec<Neighbors>, translations: &HashMap<usize, String>) {
    for (index, row) in input.iter().enumerate() {
        print!("{}: ", translations[&index]);
        for neighbor in row {
            print!("  {}", translations[neighbor]);
        }
        println!();
    }
}

fn part_2(filename: &str) -> i64 {
    _ = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/11_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/11_input.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 640);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/11_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 24);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/11_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 1452422268);
    }
}
