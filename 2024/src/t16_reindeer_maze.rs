use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs;

const TURN_SCORE: usize = 1000;
const MOVE_SCORE: usize = 1;

type Map = Vec<Vec<Tile>>;
type AdjList = HashMap<Pos, HashSet<(Pos, usize, Direction)>>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn dir_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn dir_counterclockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
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

fn visit_in_direction(
    map: &Map,
    initial_pos: Pos,
    pos: Pos,
    dir: Direction,
    dist: usize,
) -> (Pos, usize) {
    // println!("\tvisiting pos: {pos:?} in direction: {dir:?}, dist: {dist}");
    loop {
        let next_pos = pos.move_to(&dir);
        let pos_clockwise = pos.move_to(&dir.dir_clockwise());
        let pos_counterclockwise = pos.move_to(&dir.dir_counterclockwise());

        if map[next_pos.row][next_pos.col] == Tile::Wall
            || (map[pos_clockwise.row][pos_clockwise.col] == Tile::Road && pos != initial_pos)
            || (map[pos_counterclockwise.row][pos_counterclockwise.col] == Tile::Road
                && pos != initial_pos)
        {
            return (pos, dist);
        }

        let info = visit_in_direction(map, initial_pos, next_pos, dir, dist + 1);
        // println!(
        //     "\tnext crossroad is on pos: {:?} (dist: {})",
        //     info.0, info.1
        // );
        return info;
    }
}

fn convert_to_adj_list(map: &Map) -> AdjList {
    let mut adj_list = HashMap::new();
    let mut queue = Vec::new();
    let mut visited = HashSet::new();

    let start_pos = Pos {
        row: map.len() - 2,
        col: 1,
    };

    queue.push(start_pos);

    while let Some(current_pos) = queue.pop() {
        // println!("===current pos: {current_pos:?}");
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let (tile_reached, dist) = visit_in_direction(map, current_pos, current_pos, dir, 0);
            // println!("tile reached: {tile_reached:?}, dist: {dist}");
            if dist == 0 {
                continue;
            }

            adj_list
                .entry(current_pos)
                .and_modify(|neighbors: &mut HashSet<(Pos, usize, Direction)>| {
                    neighbors.insert((tile_reached, dist, dir));
                })
                .or_insert(HashSet::from([(tile_reached, dist, dir)]));

            if !visited.contains(&tile_reached) {
                // println!("pushing {tile_reached:?} to queue");
                queue.push(tile_reached);
            }

            visited.insert(tile_reached);
        }
    }

    adj_list
}

fn print_map(map: &Map) {
    for row in map {
        for tile in row {
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

fn dijkstra(adj_list: &AdjList, start_pos: Pos) {
    let mut distances = HashMap::new();
    // let mut previouses = HashMap::new();
    let mut queue = Vec::new();

    for (pos, neighbors) in adj_list.iter() {
        distances.insert(*pos, usize::MAX);
        // previouses.insert(*pos, None);
        queue.push(*pos);
    }

    distances.insert(start_pos, 0);

    while !queue.is_empty() {
        let u = queue.iter().min_by_key(|pos| distances[*pos]).unwrap();

        // queue.retain(|pos| *pos != *u);

        for (neighbor_pos, dist, dir) in adj_list[u].iter() {

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
    let adj_list = convert_to_adj_list(&map);
    for (pos, info) in adj_list.iter() {
        println!("{pos:?}: {info:?}");
    }

    let start_pos = Pos {
        row: map.len() - 2,
        col: 1,
    };

    let mut best_score = usize::MAX;
    // visit_next_adj_list(
    //     &map,
    //     &adj_list,
    //     start_pos,
    //     &mut HashSet::new(),
    //     0,
    //     Direction::Right,
    //     &mut best_score,
    // );

    best_score

    // todo!()
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
