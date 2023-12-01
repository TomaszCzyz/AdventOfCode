use std::{fs, iter};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};

type Map = Vec<Vec<Tile>>;

#[derive(Eq, PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Tile {
    Ground,
    Blizzard(Direction),
}

impl Debug for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Blizzard(dir) => match dir {
                Direction::Up => write!(f, "^"),
                Direction::Right => write!(f, ">"),
                Direction::Down => write!(f, "v"),
                Direction::Left => write!(f, "<"),
            }
        }
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn up(&self) -> Point {
        Point {
            row: self.row - 1,
            col: self.col,
        }
    }

    fn down(&self) -> Point {
        Point {
            row: self.row + 1,
            col: self.col,
        }
    }

    fn right(&self) -> Point {
        Point {
            row: self.row,
            col: self.col + 1,
        }
    }

    fn left(&self) -> Point {
        Point {
            row: self.row,
            col: self.col - 1,
        }
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

fn read_input(file_name: &str) -> Map {
    fs::read_to_string(file_name)
        .unwrap()
        .split("\r\n")
        .skip(1)
        .take_while(|line| !line.starts_with("##"))
        .map(|line| line.trim_matches('#')
            .chars()
            .map(|ch| if ch == '.' {
                Tile::Ground
            } else {
                Tile::Blizzard(match ch {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => panic!()
                })
            }).collect::<Vec<_>>()
        ).collect::<Vec<_>>()
}

struct TaskData {
    height: usize,
    width: usize,
    blizzards_horizontally: HashMap<Point, HashSet<usize>>,
    blizzards_vertically: HashMap<Point, HashSet<usize>>,
}

pub fn blizzard_basin_part_1(filename: &str) -> usize {
    let input = read_input(filename);
    let (blizzards_horizontally, blizzards_vertically) = calculate_blizzards_distances(&input);
    let (width, height) = (input[0].len(), input.len());

    let task_data = TaskData {
        height,
        width,
        blizzards_horizontally,
        blizzards_vertically,
    };

    let start = Point { row: 0, col: 1 };
    let goal = Point { row: task_data.height + 1, col: task_data.width };
    let start_minute = 0;

    let (result, _came_from) = a_star(start, goal, start_minute, &task_data, manhattan_dist_plus_minute).unwrap();

    // for minute in 0..=12 {
    //     println!("== MINUTE {minute} ==");
    //     for row in 1..=height {
    //         for col in 1..=width {
    //             // '☠'
    //             let ch = if is_clear(&Point { row, col }, minute, &task_data) {
    //                 '▮'
    //             } else {
    //                 '▯'
    //             };
    //             print!("{}", ch);
    //         }
    //         println!()
    //     }
    // }

    result
}

pub fn blizzard_basin_part_2(filename: &str) -> usize {
    let input = read_input(filename);
    let (blizzards_horizontally, blizzards_vertically) = calculate_blizzards_distances(&input);
    let (width, height) = (input[0].len(), input.len());

    let task_data = TaskData {
        height,
        width,
        blizzards_horizontally,
        blizzards_vertically,
    };

    let start = Point { row: 0, col: 1 };
    let goal = Point { row: task_data.height + 1, col: task_data.width };

    let algorithm_data = [
        (start, goal),
        (goal, start),
        (start, goal),
    ];

    let mut sum = 0usize;
    let mut start_minute = 0;
    for (begin, end) in algorithm_data.into_iter() {
        let (minutes, _) = a_star(begin, end, start_minute, &task_data, manhattan_dist_plus_minute).unwrap();
        sum += minutes;
        start_minute = sum;
    }

    sum
}

fn manhattan_dist_plus_minute((p1, min): (&Point, usize), p2: &Point) -> u32 {
    (p1.row.abs_diff(p2.row) + p1.col.abs_diff(p2.col)) as u32 + min as u32
}

fn a_star(
    start: Point,
    goal: Point,
    start_minute: usize,
    task_data: &TaskData,
    h: fn((&Point, usize), &Point) -> u32,
) -> Option<(usize, HashMap<Point, Point>)> {
    let mut open_set = HashSet::from([(start, start_minute)]);
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    g_score.entry((start, start_minute)).or_insert(0);
    f_score.entry((start, start_minute)).or_insert(h((&start, start_minute), &goal));

    while !open_set.is_empty()
    {
        let (current_point, current_minute) = *open_set.iter()
            .min_by(|&elem1, &elem2| f_score[elem1].cmp(&f_score[elem2]))
            .unwrap();

        open_set.remove(&(current_point, current_minute));

        if current_point == goal {
            return Some((current_minute - start_minute, came_from));
        }

        let available_neighbors = neighbors_of(current_point, task_data.height, task_data.width).into_iter()
            .chain(iter::once(current_point))
            .filter(|p| is_clear(p, current_minute + 1, task_data))
            .collect::<Vec<_>>();

        for neighbor in available_neighbors {
            let score_of_current = *g_score.entry((current_point, current_minute)).or_insert(usize::MAX);
            let score_of_neighbor = *g_score.entry((neighbor, current_minute + 1)).or_insert(usize::MAX);

            let tentative_g_score = score_of_current + 1;
            if tentative_g_score <= score_of_neighbor {
                came_from.entry(neighbor)
                    .and_modify(|x| *x = current_point)
                    .or_insert(current_point);

                g_score.entry((neighbor, current_minute + 1))
                    .and_modify(|x| *x = tentative_g_score);

                f_score.entry((neighbor, current_minute + 1))
                    .and_modify(|x| *x = tentative_g_score as u32 + h((&neighbor, current_minute + 1), &goal))
                    .or_insert(tentative_g_score as u32 + h((&neighbor, current_minute + 1), &goal));

                if !open_set.contains(&(neighbor, current_minute + 1)) {
                    open_set.insert((neighbor, current_minute + 1));
                }
            }
        }
    }

    println!("Open set is empty but goal was never reached!");
    None
}

fn neighbors_of(p: Point, height: usize, width: usize) -> Vec<Point> {
    let mut neighbors = Vec::new();

    if p.col != width + 1 {
        neighbors.push(p.right())
    }
    if p.row != height + 1 {
        neighbors.push(p.down())
    }
    if p.row != 0 {
        neighbors.push(p.up())
    }
    if p.col != 0 {
        neighbors.push(p.left())
    }
    neighbors
}

fn is_clear(
    point: &Point,
    minute: usize,
    task_data: &TaskData,
) -> bool {
    let (width, height) = (task_data.width, task_data.height);
    let start = Point { row: 0, col: 1 };
    let end = Point { row: height + 1, col: width };

    if *point == start || *point == end {
        return true;
    }

    // points on edges
    if point.row == 0 || point.row == height + 1 || point.col == 0 || point.col == width + 1 {
        return false;
    }

    for blizzard in task_data.blizzards_horizontally[point].iter() {
        if (width * 1000 + *blizzard - minute) % width == 0 {
            return false;
        }
    }

    for blizzard in task_data.blizzards_vertically[point].iter() {
        if (height * 1000 + *blizzard - minute) % height == 0 {
            return false;
        }
    }

    true
}

/// Creates maps, that represents how far are blizzards for each point at the map.
/// Points of the map's rectangle have coordinates starting from 1 (not zero),
/// to allow adding start point above with non-negative value.
fn calculate_blizzards_distances(input: &Map) -> (HashMap<Point, HashSet<usize>>, HashMap<Point, HashSet<usize>>) {
    let mut blizzards_horizontally = HashMap::new();
    let mut blizzards_vertically = HashMap::new();
    let (width, height) = (input[0].len(), input.len());

    for row in 0..height {
        for col in 0..width {
            let mut horizontal_blizzards_distances = HashSet::new();
            for i in 0..width {
                if let Tile::Blizzard(dir) = &input[row][(col + i) % width] {
                    match dir {
                        Direction::Right => horizontal_blizzards_distances.insert(width - i),
                        Direction::Left => horizontal_blizzards_distances.insert(i),
                        _ => true,
                    };
                }
            }
            blizzards_horizontally.insert(Point { row: row + 1, col: col + 1 }, horizontal_blizzards_distances);

            let mut vertical_blizzards_distances = HashSet::new();
            for i in 0..height {
                if let Tile::Blizzard(dir) = &input[(row + i) % height][col] {
                    match dir {
                        Direction::Down => vertical_blizzards_distances.insert(height - i),
                        Direction::Up => vertical_blizzards_distances.insert(i),
                        _ => true
                    };
                }
            }
            blizzards_vertically.insert(Point { row: row + 1, col: col + 1 }, vertical_blizzards_distances);
        }
    }

    (blizzards_horizontally, blizzards_vertically)
}

fn print_input(input: &Map) {
    for row in input {
        for tile in row {
            print!("{:?}", tile);
        }
        println!()
    }
}

#[allow(dead_code)]
pub fn blizzard_basin_part_1_bfs(filename: &str) -> usize {
    let input = read_input(filename);
    print_input(&input);

    let (blizzards_horizontally, blizzards_vertically) = calculate_blizzards_distances(&input);
    let (width, height) = (input[0].len(), input.len());

    let task_data = TaskData {
        height,
        width,
        blizzards_horizontally,
        blizzards_vertically,
    };

    let mut queue = VecDeque::from([(Point { row: 0, col: 0 }, 1usize)]);

    while !queue.is_empty()
    {
        let (point, minute) = queue.pop_front().unwrap();

        if queue.len() % 100000 == 0 {
            println!("point: {:?}, minute {}\t\t (queue size: {})", point, minute, queue.len());
        }

        if point.row == height - 1 && point.col == width - 1 {
            return minute;
        }

        let available_neighbors = neighbors_of(point, height, width).into_iter()
            .chain(iter::once(point))
            .filter(|p| is_clear(p, minute, &task_data))
            .collect::<Vec<_>>();

        for neighbor in available_neighbors {
            queue.push_back((neighbor, minute + 1));
        }
    }

    println!("queue is empty");
    1
}
