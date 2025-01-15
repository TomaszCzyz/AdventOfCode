use fs::read_to_string;
use std::collections::VecDeque;
use std::fs;

type AdjMatrix = Vec<Vec<usize>>;

type MatrixGraph = Vec<Vec<bool>>;

fn read_input(file_name: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut adj_list = Vec::<Vec<usize>>::new();

    let file_content = read_to_string(file_name).unwrap();

    let vertex_values = file_content
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<Vec<_>>();

    let rows_count = file_content.lines().count();
    let cols_count = file_content.lines().next().unwrap().chars().count();
    (0..rows_count).for_each(|row_i| {
        (0..cols_count).for_each(|col_i| {
            let vertex_number = row_i * cols_count + col_i;
            let mut neighbors = Vec::new();
            if row_i != 0 {
                neighbors.push(vertex_number - cols_count);
            }

            if row_i != rows_count - 1 {
                neighbors.push(vertex_number + cols_count);
            }

            if col_i != 0 {
                neighbors.push(vertex_number - 1);
            }

            if col_i != cols_count - 1 {
                neighbors.push(vertex_number + 1);
            }

            adj_list.push(neighbors);
        })
    });

    (adj_list, vertex_values)
}

fn garden_groups_part_1(filename: &str) -> usize {
    let (adj_list, vertex_values) = read_input(filename);

    println!("vertex_values: {vertex_values:?}");
    for (i, v) in adj_list.iter().enumerate() {
        println!(
            "{}: {:?}",
            vertex_values[i],
            v.iter().map(|j| vertex_values[*j]).collect::<Vec<_>>()
        );
    }

    // (area, perimeter)
    let mut vertex_summaries = vec![(0u32, 0u32); adj_list.len()];

    let mut visited = vec![false; adj_list.len()];
    while visited.iter().any(|x| !*x) {
        let unvisited = visited.iter().position(|x| !*x).unwrap();
        let mut queue = VecDeque::from([unvisited]);
        while let Some(vertex) = queue.pop_front() {
            visited[vertex] = true;
            for neighbor in adj_list[vertex] {
                if vertex_values[neighbor] == vertex_values[vertex] {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    todo!()
}

fn garden_groups_part_2(filename: &str, rounds_count: usize) -> usize {
    let numbers = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input_1() {
        let answer = garden_groups_part_1("inputs/12_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 140);
    }

    #[test]
    fn part_1_example_input_2() {
        let answer = garden_groups_part_1("inputs/12_input_2.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 772);
    }

    #[test]
    fn part_1_example_input_3() {
        let answer = garden_groups_part_1("inputs/12_input_3.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 1930);
    }

    #[test]
    fn part_1_input() {
        let answer = garden_groups_part_1("inputs/12_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }
}
