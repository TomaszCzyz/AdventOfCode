use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn move_to(&self, direction: &Direction) -> Pos {
        match direction {
            Direction::Up => Pos {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Pos {
                row: self.row + 1,
                col: self.col,
            },
            Direction::Left => Pos {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Pos {
                row: self.row,
                col: self.col + 1,
            },
        }
    }
}

fn convert_to_tree(map: &Map) -> AdjList {
    let map_len = map.len();
    let map_width = map[0].len();

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

fn print_path(map: &Map, path: &HashSet<Pos>) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, tile) in row.iter().enumerate() {
            if path.contains(&Pos {
                row: row_idx,
                col: col_idx,
            }) {
                print!("*");
                continue;
            }
            match tile {
                Tile::Wall => print!("#"),
                Tile::Road => print!("."),
                Tile::Start => print!("S"),
                Tile::End => print!("E"),
            }
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

fn visit_next(
    map: &Map,
    pos: Pos,
    path: &mut HashSet<Pos>,
    score: usize,
    dir: Direction,
    current_best: &mut usize,
) {
    if map[pos.row][pos.col] == Tile::End {
        println!("Reached end with score {:?}", score);
        print_path(map, path);
        if score < *current_best {
            *current_best = score;
            println!("Found new best score: {:?}", score);
        }
        return;
    }

    path.insert(pos);

    for next_dir in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        // todo: multiple turns score?
        let cost = if next_dir == dir {
            MOVE_SCORE
        } else {
            MOVE_SCORE + TURN_SCORE
        };

        let next_pos = pos.move_to(&next_dir);

        if map[next_pos.row][next_pos.col] != Tile::Wall && !path.contains(&next_pos) {
            visit_next(
                map,
                pos.move_to(&next_dir),
                &mut path.clone(),
                score + cost,
                next_dir,
                current_best,
            );
        }
    }
}

fn print_path_2(path: &HashSet<Pos>) {
    let max_row_value = path.iter().map(|p| p.row).max().unwrap();

    let mut path_clone = path.clone();
    println!("whole path: {:?}", path_clone);

    let mut curr_pos = Pos {
        row: max_row_value,
        col: 1,
    };
    path_clone.remove(&curr_pos);

    print!("path: ");
    print!("{:?}", curr_pos);

    while !path_clone.is_empty() {
        // println!();
        // println!("path_clone: {path_clone:?}");
        let next_pos = match path_clone
            .iter()
            .find(|p| p.row.abs_diff(curr_pos.row) + p.col.abs_diff(curr_pos.col) == 1)
        {
            Some(p) => *p,
            None => {
                println!();
                println!("remaining path: {:?}", path_clone);
                panic!();
            }
        };

        curr_pos = next_pos;
        print!(" -> {:?}", curr_pos);
        let x = path_clone.remove(&curr_pos);
        if !x {
            panic!("NOT REMOVED")
        }
    }
}

fn reindeer_maze_part_1(filename: &str) -> usize {
    let map = read_input(filename);
    // print_map(&map);

    let start_pos = Pos {
        row: map.len() - 2,
        col: 1,
    };

    let mut best_score = usize::MAX;
    visit_next(
        &map,
        start_pos,
        &mut HashSet::new(),
        0,
        Direction::Right,
        &mut best_score,
    );

    best_score
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
        let answer = reindeer_maze_part_1("inputs/16_input_example_2.txt");

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
