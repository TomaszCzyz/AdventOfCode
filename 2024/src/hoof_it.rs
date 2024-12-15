use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

type VertexValues = HashMap<usize, u32>;

type AdjMatrix = Vec<Vec<usize>>;

fn read_input(file_name: &str) -> (AdjMatrix, VertexValues, Vec<usize>) {
    let map = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .filter(|val| val.is_digit(10))
                .map(|val| val.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let up = (-1, 0);
    let down = (1, 0);
    let left = (0, -1);
    let right = (0, 1);
    let directions = [up, down, left, right];

    let row_max_len = map.len() as i32 - 1;
    let col_max_len = map[0].len() as i32 - 1;

    let mut adj_matrix = Vec::new();
    let mut vertex_values = VertexValues::new();
    for (row_i, row) in map.iter().enumerate().map(|(i, row)| (i as i32, row)) {
        for (col_i, value) in row.iter().enumerate().map(|(j, col)| (j as i32, col)) {
            let mut edges = HashSet::new();
            for &(row_shift, col_shift) in directions.iter() {
                let ii = (row_i + row_shift).max(0).min(row_max_len);
                let jj = (col_i + col_shift).max(0).min(col_max_len);
                if map[ii as usize][jj as usize].abs_diff(*value) == 1 {
                    edges.insert(ii as usize * row.len() + jj as usize);
                }
            }
            adj_matrix.push(Vec::from_iter(edges));
            vertex_values.insert(row_i as usize * row.len() + col_i as usize, *value);
        }
    }

    let mut start_vertices = vertex_values
        .iter()
        .filter(|&(_, val)| *val == 0)
        .map(|(i, _)| *i)
        .collect::<Vec<usize>>();

    start_vertices.sort();

    (adj_matrix, vertex_values, start_vertices)
}

fn find_accessible_ends(
    adj_matrix: &AdjMatrix,
    vertex_values: &VertexValues,
    start_vertex: usize,
) -> usize {
    let mut visited = HashSet::new();
    let mut reachable_ends = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_vertex);

    while let Some(vertex) = queue.pop_front() {
        visited.insert(vertex);
        let curr_value = vertex_values[&vertex];
        if curr_value == 9 {
            reachable_ends.insert(vertex);
            continue;
        }

        let neighbors = adj_matrix[vertex]
            .iter()
            .filter(|n| !visited.contains(n))
            .filter(|n| vertex_values[&n] == curr_value + 1);

        for neighbor in neighbors {
            queue.push_back(*neighbor);
        }
    }

    reachable_ends.len()
}

fn find_accessible_ends_all_trials(
    adj_matrix: &AdjMatrix,
    vertex_values: &VertexValues,
    start_vertex: usize,
) -> usize {
    let mut visited = HashSet::new();
    let mut reachable_ends = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(start_vertex);

    while let Some(vertex) = queue.pop_front() {
        visited.insert(vertex);
        let curr_value = vertex_values[&vertex];
        if curr_value == 9 {
            reachable_ends.push(vertex);
            continue;
        }

        let neighbors = adj_matrix[vertex]
            .iter()
            .filter(|n| vertex_values[&n] == curr_value + 1);

        for neighbor in neighbors {
            queue.push_back(*neighbor);
        }
    }

    reachable_ends.len()
}

fn hoof_it_part_1(filename: &str) -> usize {
    let (adj_matrix, vertex_values, start_vertices) = read_input(filename);

    start_vertices
        .iter()
        .map(|start| find_accessible_ends(&adj_matrix, &vertex_values, *start))
        .sum()
}

fn hoof_it_part_2(filename: &str) -> usize {
    let (adj_matrix, vertex_values, start_vertices) = read_input(filename);

    start_vertices
        .iter()
        .map(|start| find_accessible_ends_all_trials(&adj_matrix, &vertex_values, *start))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input() {
        let answer = hoof_it_part_1("inputs/3_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 36);
    }

    #[test]
    fn part_1_input() {
        let answer = hoof_it_part_1("inputs/3_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 593);
    }

    #[test]
    fn part_2_input_example() {
        let answer = hoof_it_part_2("inputs/3_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 81);
    }

    #[test]
    fn part_2_input_example_2() {
        let answer = hoof_it_part_2("inputs/3_input_example_2.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 227);
    }

    #[test]
    fn part_2_input_example_3() {
        let answer = hoof_it_part_2("inputs/3_input_example_3.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 3);
    }

    #[test]
    fn part_2_input() {
        let answer = hoof_it_part_2("inputs/3_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 1192);
    }
}
