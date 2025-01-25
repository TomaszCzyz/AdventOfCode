use fs::read_to_string;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug, Copy, Clone)]
struct PerimeterInfo {
    coords: (usize, usize),
    side: Side,
}

#[derive(Debug)]
struct VertexData {
    area_number: u32,
    perimeters: Vec<PerimeterInfo>,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Side {
    L,
    R,
    T,
    B,
}

fn read_input(file_name: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    let file_content = read_to_string(file_name).unwrap();

    let number_to_flower_map = file_content
        .lines()
        .map(|l| l.chars())
        .flatten()
        .collect::<HashSet<_>>()
        .iter()
        .copied()
        .collect::<Vec<_>>();

    let matrix = file_content
        .lines()
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|ch| number_to_flower_map.iter().position(|&x| x == ch).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (matrix, number_to_flower_map)
}

fn traverse_garden(matrix: Vec<Vec<usize>>) -> Vec<VertexData> {
    let rows_count = matrix.len();
    let cols_count = matrix[0].len();

    let mut visited = HashMap::new();
    for row_index in 0..rows_count {
        for col_index in 0..cols_count {
            visited.insert((row_index, col_index), false);
        }
    }

    let mut area_number = 0;
    let mut vertex_summaries = vec![];

    while visited.iter().any(|(_, is_visited)| !is_visited) {
        let (&unvisited, _) = visited.iter().find(|&(_, is_visited)| !is_visited).unwrap();

        let mut queue = VecDeque::from([unvisited]);
        while let Some(vertex) = queue.pop_front() {
            if visited[&vertex] {
                continue;
            } else {
                visited
                    .entry(vertex)
                    .and_modify(|is_visited| *is_visited = true);
            }

            let (row_i, col_i) = vertex;
            let mut perimeters = Vec::<PerimeterInfo>::new();
            let mut neighbors = Vec::new();

            if row_i != 0 {
                neighbors.push(((row_i - 1, col_i), Side::T));
            } else {
                perimeters.push(PerimeterInfo {
                    coords: vertex,
                    side: Side::T,
                });
            }

            if row_i != rows_count - 1 {
                neighbors.push(((row_i + 1, col_i), Side::B));
            } else {
                perimeters.push(PerimeterInfo {
                    coords: vertex,
                    side: Side::B,
                });
            }

            if col_i != 0 {
                neighbors.push(((row_i, col_i - 1), Side::L));
            } else {
                perimeters.push(PerimeterInfo {
                    coords: vertex,
                    side: Side::L,
                });
            }

            if col_i != cols_count - 1 {
                neighbors.push(((row_i, col_i + 1), Side::R));
            } else {
                perimeters.push(PerimeterInfo {
                    coords: vertex,
                    side: Side::R,
                });
            }

            for (neighbor, side) in neighbors.into_iter() {
                let (neighbor_row, neighbor_col) = neighbor;
                if matrix[row_i][col_i] == matrix[neighbor_row][neighbor_col] {
                    queue.push_back((neighbor_row, neighbor_col));
                } else {
                    perimeters.push(PerimeterInfo {
                        coords: vertex,
                        side,
                    });
                }
            }

            vertex_summaries.push(VertexData {
                area_number,
                perimeters,
            });
        }

        area_number += 1;
    }
    vertex_summaries
}

fn garden_groups_part_1(filename: &str) -> u32 {
    let (matrix, _) = read_input(filename);

    let vertex_summaries = traverse_garden(matrix);

    vertex_summaries
        .iter()
        .chunk_by(|x| x.area_number)
        .into_iter()
        .map(|(_, group)| {
            let vertices_data = group.collect::<Vec<_>>();
            let perimeter = vertices_data
                .iter()
                .map(|x| x.perimeters.len())
                .sum::<usize>() as u32;
            let area = vertices_data.len() as u32;

            area * perimeter
        })
        .sum::<u32>()
}

fn garden_groups_part_2(filename: &str) -> u32 {
    let (matrix, _) = read_input(filename);
    
    let vertex_summaries = traverse_garden(matrix);

    vertex_summaries
        .iter()
        .chunk_by(|x| x.area_number)
        .into_iter()
        .map(|(_, group)| {
            let vertices_data = group.collect::<Vec<_>>();

            let perimeter = vertices_data
                .iter()
                .flat_map(|&x| x.perimeters.iter().copied())
                .sorted_by(|p_a, p_b| p_a.side.cmp(&p_b.side))
                .chunk_by(|e| e.side)
                .into_iter()
                .map(|(key, g)| {
                    let mut perimeters_by_side = g.map(|p| p.coords).collect::<Vec<_>>();

                    let direction = if key == Side::L || key == Side::R {
                        perimeters_by_side.sort_by(|(row_a, col_a), (row_b, col_b)| {
                            col_a.cmp(&col_b).then_with(|| row_a.cmp(&row_b))
                        });
                        Direction::Vertical
                    } else {
                        perimeters_by_side.sort_by(|(row_a, col_a), (row_b, col_b)| {
                            row_a.cmp(&row_b).then_with(|| col_a.cmp(&col_b))
                        });
                        Direction::Horizontal
                    };

                    let mut side_perimeter = perimeters_by_side.len();
                    for elements in perimeters_by_side.windows(2) {
                        if is_in_line(elements, direction) {
                            side_perimeter -= 1;
                        }
                    }
                    side_perimeter
                })
                .sum::<usize>() as u32;

            let area = vertices_data.len() as u32;

            area * perimeter
        })
        .sum::<u32>()
}

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Horizontal,
    Vertical,
}

fn is_in_line(elem: &[(usize, usize)], direction: Direction) -> bool {
    let (this_row_i, this_col_i) = elem[0];
    let (next_row_i, next_col_i) = elem[1];

    if direction == Direction::Vertical {
        this_row_i.abs_diff(next_row_i) == 1 && this_col_i == next_col_i
    } else {
        this_col_i.abs_diff(next_col_i) == 1 && this_row_i == next_row_i
    }
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
    fn part_2_example_input_1() {
        let answer = garden_groups_part_2("inputs/12_input_example_1.txt");

        println!("part 1 - example 1 - answer: {:?}", answer);
        assert_eq!(answer, 80);
    }

    #[test]
    fn part_2_example_input_4() {
        let answer = garden_groups_part_2("inputs/12_input_example_4.txt");

        println!("part 1 - example 2 - answer: {:?}", answer);
        assert_eq!(answer, 236);
    }

    #[test]
    fn part_2_example_input_5() {
        let answer = garden_groups_part_2("inputs/12_input_example_5.txt");

        println!("part 1 - example 3 - answer: {:?}", answer);
        assert_eq!(answer, 368);
    }

    #[test]
    fn part_2_input() {
        let answer = garden_groups_part_2("inputs/12_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 966476);
    }
}
