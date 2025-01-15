use fs::read_to_string;
use itertools::Itertools;
use std::collections::VecDeque;
use std::fs;

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

struct VertexData {
    perimeter: u32,
    area_number: u32,
}

fn garden_groups_part_1(filename: &str) -> u32 {
    let (adj_list, vertex_values) = read_input(filename);

    let mut area_number = 0;
    let mut vertex_summaries = vec![];
    let mut visited = vec![false; adj_list.len()];

    while visited.iter().any(|x| !*x) {
        let unvisited = visited.iter().position(|x| !*x).unwrap();
        let mut queue = VecDeque::from([unvisited]);
        while let Some(vertex) = queue.pop_front() {
            if visited[vertex] {
                continue;
            } else {
                visited[vertex] = true;
            }

            let mut perimeter = 4;
            for neighbor in adj_list[vertex].iter() {
                if vertex_values[*neighbor] == vertex_values[vertex] {
                    perimeter -= 1;
                    queue.push_back(*neighbor);
                }
            }

            vertex_summaries.push(VertexData {
                area_number,
                perimeter,
            });
        }

        area_number += 1;
    }

    assert_eq!(vertex_summaries.len(), vertex_values.len());

    vertex_summaries
        .iter()
        .chunk_by(|x| x.area_number)
        .into_iter()
        .map(|(_, group)| {
            let vertices_data = group.collect::<Vec<_>>();
            let perimeter = vertices_data.iter().map(|x| x.perimeter).sum::<u32>();
            let area = vertices_data.len() as u32;

            area * perimeter
        })
        .sum::<u32>()
}

fn garden_groups_part_2(filename: &str) -> usize {
    let _ = read_input(filename);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input_1() {
        let answer = garden_groups_part_1("inputs/12_input_example_1.txt");

        println!("part 1 - example 1 - answer: {:?}", answer);
        assert_eq!(answer, 140);
    }

    #[test]
    fn part_1_example_input_2() {
        let answer = garden_groups_part_1("inputs/12_input_example_2.txt");

        println!("part 1 - example 2 - answer: {:?}", answer);
        assert_eq!(answer, 772);
    }

    #[test]
    fn part_1_example_input_3() {
        let answer = garden_groups_part_1("inputs/12_input_example_3.txt");

        println!("part 1 - example 3 - answer: {:?}", answer);
        assert_eq!(answer, 1930);
    }

    #[test]
    fn part_1_input() {
        let answer = garden_groups_part_1("inputs/12_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 1573474);
    }
}
