use itertools::Itertools;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

const TURN_SCORE: usize = 1000;
const MOVE_SCORE: usize = 1;

type Map = Vec<Vec<Tile>>;
type AdjList = HashMap<usize, Vec<usize>>;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

fn move_to(pos: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (pos.0 - 1, pos.1),
        Direction::Down => (pos.0 + 1, pos.1),
        Direction::Left => (pos.0, pos.1 - 1),
        Direction::Right => (pos.0, pos.1 + 1),
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Tile {
    Wall,
    Road,
    Start,
    End,
}

fn read_input(file_name: &str) -> Vec<Vec<Tile>> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Road,
                    'S' => Tile::Start,
                    'E' => Tile::End,
                    _ => panic!("invalid character: {}", c),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn convert_to_tree(map: &Map) -> AdjList {
    let map_len = map.len();
    let map_width = map[0].len();
    let start_x = 1;
    let start_y = map_len - 2;

    // find all vertices that are crossroads
    let mut crossroads_pos = Vec::new();
    for x in 1..map_width - 1 {
        for y in 1..map_len - 2 {
            let vertex = &map[y][x];
            if *vertex != Tile::Road || *vertex == Tile::Start || *vertex == Tile::End {
                continue;
            }

            let neighbors = get_near_roads(map, x, y);
            if neighbors.len() > 2 {
                crossroads_pos.push((x, y));
            } else if neighbors.len() == 2 {
                // add only if they are not on the straight road
                if neighbors[0].0 != neighbors[1].0 && neighbors[0].1 != neighbors[1].1 {
                    crossroads_pos.push((x, y));
                }
            }
        }
    }

    println!("crossroads: {:?}", crossroads_pos);
    print_map_with_crossroads(&map, &crossroads_pos);

    todo!()
}

fn get_near_roads(map: &Map, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if map[y][x - 1] != Tile::Wall {
        neighbors.push((y, x - 1));
    }

    if map[y][x + 1] != Tile::Wall {
        neighbors.push((y, x + 1));
    }

    if map[y - 1][x] != Tile::Wall {
        neighbors.push((y - 1, x));
    }

    if map[y + 1][x] != Tile::Wall {
        neighbors.push((y + 1, x));
    }

    neighbors
}

fn print_map(map: &Map) {
    for row in map {
        for tile in row {
            print!("{:?}", tile);
            // match tile {
            //     Tile::Wall => print!("#"),
            //     Tile::Road => print!("."),
            //     Tile::Start => print!("S"),
            //     Tile::End => print!("E"),
            // }
        }
        println!();
    }
}

fn print_map_with_crossroads(map: &Map, crossroads: &Vec<(usize, usize)>) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if crossroads.contains(&(row_idx, col_idx)) {
                print!("X");
                continue;
            }
            // match tile {
            //     Tile::Wall => print!("#"),
            //     Tile::Road => print!("."),
            //     Tile::Start => print!("S"),
            //     Tile::End => print!("E"),
            // }
            print!("{:?}", tile);
        }
        println!();
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

struct VisitInfo {
    pos: Pos,
    dir: Direction,
}

fn dijkstra(map: &Map, start_col: usize, start_row: usize) {
    let mut unvisited = Vec::<VisitInfo>::new();
    let mut min_dist = HashMap::new();
    let mut queue = Vec::new();

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if map[row_idx][col_idx] == Tile::Wall {
                continue;
            }

            let pos = Pos {
                row: row_idx,
                col: col_idx,
            };

            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                unvisited.push(VisitInfo { pos, dir });
            }

            min_dist.insert(pos, usize::MAX);
            queue.push(pos);
        }
    }

    min_dist.insert(
        Pos {
            row: map.len() - 2,
            col: 1,
        },
        0,
    );

    while !queue.is_empty() {
        let min_dist_vertex = queue
            .iter()
            .min_by(|a, b| min_dist[a].cmp(&min_dist[b]))
            .unwrap();

        let min_dist_vertex =
            queue.remove(queue.iter().position(|x| x == min_dist_vertex).unwrap());

        for
    }
}

fn reindeer_maze_part_1(filename: &str) -> usize {
    let map = read_input(filename);
    print_map(&map);
    // let tree = convert_to_tree(&map);

    let map_len = map.len();

    let start_col = 1;
    let start_row = map_len - 2;

    dijkstra(
        &map,
        start_col,
        start_row,
        &mut Vec::new(),
        0,
        Direction::Right,
    );

    todo!()
}

fn reindeer_maze_part_2(filename: &str) -> usize {
    let _ = read_input(filename);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = reindeer_maze_part_1("inputs/16_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 7036);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = reindeer_maze_part_1("inputs/16_input_example_1.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 11048);
    }

    #[test]
    fn part_1_input() {
        let answer = reindeer_maze_part_1("inputs/16_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 1456590);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = reindeer_maze_part_2("inputs/16_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 9021);
    }

    #[test]
    fn part_2_input() {
        let answer = reindeer_maze_part_2("inputs/16_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 1489116);
    }
}
