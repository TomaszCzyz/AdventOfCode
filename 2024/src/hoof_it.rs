use std::collections::{HashMap, HashSet};
use std::fs;

type VertexValues = HashMap<usize, u32>;

type AdjMatrix = Vec<Vec<usize>>;

fn read_input(file_name: &str) -> (AdjMatrix, VertexValues) {
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

    (adj_matrix, vertex_values)
}

fn hoof_it_part_1(filename: &str) -> usize {
    let (adj_matrix, vertex_values) = read_input(filename);

    let start_vertices = vertex_values
        .iter()
        .filter(|&(i, val)| *val == 0)
        .map(|(i, _)| *i)
        .collect::<Vec<usize>>();

    for (vertex, adj_vertices) in adj_matrix.iter().enumerate() {
        println!("{vertex}: {:?}", adj_vertices);
    }
    println!("vertex_values: {:?}", vertex_values);
    println!("start_vertices: {:?}", start_vertices);

    todo!()
}

fn hoof_it_part_2(_filename: &str) -> usize {
    todo!()
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
        assert_eq!(answer, 326);
    }

    #[test]
    fn part_2_input_example() {
        let answer = hoof_it_part_2("inputs/3_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 4);
    }

    #[test]
    fn part_2_input() {
        let answer = hoof_it_part_2("inputs/3_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 381);
    }
}
