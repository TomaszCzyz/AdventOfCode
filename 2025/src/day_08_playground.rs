use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::{Debug, Formatter};
use std::fs;

type Coord = (i64, i64, i64);

#[derive(PartialEq, Debug)]
struct MinNonNan(f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct Connection {
    distance: MinNonNan,
    coord1: Coord,
    coord2: Coord,
}

impl Eq for Connection {}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Conn({:?} <-> {:?} = {:?})",
            self.coord1, self.coord2, self.distance.0
        )
    }
}

fn read_input(file_name: &str) -> Vec<Coord> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<Vec<_>>()
}

fn part_1(filename: &str, laps: usize) -> usize {
    let coords = read_input(filename);
    let mut heap = calc_distances(coords);
    let mut circuits = Vec::<HashSet<Coord>>::new();

    let mut counter = 0;
    while let Some(elem) = heap.pop() {
        if counter >= laps {
            break;
        }

        let (index_1, index_2) = find_in_circuits(&circuits, &elem);

        insert_or_merge_into_circuit(&mut circuits, &elem, index_1, index_2);

        counter += 1;
    }

    circuits.sort_unstable_by_key(|x| 100000 - x.len());
    circuits.iter().map(|c| c.len()).take(3).product()
}

fn part_2(filename: &str) -> i64 {
    let coords = read_input(filename);
    let coords_len = coords.len();
    let mut heap = calc_distances(coords.clone());
    let mut circuits = Vec::<HashSet<Coord>>::new();
    let mut answer = 0;

    while let Some(elem) = heap.pop() {
        let (index_1, index_2) = find_in_circuits(&circuits, &elem);

        insert_or_merge_into_circuit(&mut circuits, &elem, index_1, index_2);

        if circuits.len() == 1 && circuits[0].len() == coords_len {
            answer = elem.coord1.0 * elem.coord2.0;
            break;
        }
    }

    answer
}

fn insert_or_merge_into_circuit(
    circuits: &mut Vec<HashSet<Coord>>,
    elem: &Connection,
    index_1: Option<usize>,
    index_2: Option<usize>,
) {
    match (index_1, index_2) {
        (Some(i1), Some(i2)) if i1 != i2 => {
            // merge sets
            // ensure i_big > i_small to avoid shifting problems
            let (big, small) = if i1 > i2 { (i1, i2) } else { (i2, i1) };

            // take both sets out of the vector; remove higher index first
            let mut big_set = circuits.remove(big);
            let small_set = circuits.remove(small);

            for coord in small_set {
                big_set.insert(coord);
            }

            circuits.push(big_set);
        }
        (Some(i1), None) => {
            circuits[i1].insert(elem.coord2);
        }
        (None, Some(i2)) => {
            circuits[i2].insert(elem.coord1);
        }
        (None, None) => {
            let mut new_circuit = HashSet::<Coord>::new();
            new_circuit.insert(elem.coord1);
            new_circuit.insert(elem.coord2);
            circuits.push(new_circuit);
        }
        (Some(_), Some(_)) => unreachable!(),
    }
}

fn find_in_circuits(
    circuits: &Vec<HashSet<Coord>>,
    elem: &Connection,
) -> (Option<usize>, Option<usize>) {
    let mut index_1: Option<usize> = None;
    let mut index_2: Option<usize> = None;
    for (index, circuit) in circuits.iter().enumerate() {
        if circuit.contains(&elem.coord1) {
            index_1 = Some(index);
            continue;
        }
        if circuit.contains(&elem.coord2) {
            index_2 = Some(index);
            continue;
        }
    }
    (index_1, index_2)
}

fn print_sets(circuits: &Vec<HashSet<Coord>>) {
    for circuit in circuits.iter() {
        println!(
            "len: {}\t\t{:?}",
            circuit.len(),
            circuit.iter().sorted().collect::<Vec<&Coord>>()
        );
    }
}

fn calc_distances(coords: Vec<Coord>) -> BinaryHeap<Connection> {
    let mut heap = BinaryHeap::<Connection>::new();

    for i in 0..coords.len() {
        for j in i..coords.len() {
            if i == j {
                continue;
            }

            let dist = distance(coords[i], coords[j]);
            heap.push(Connection {
                distance: MinNonNan(dist),
                coord1: coords[i],
                coord2: coords[j],
            });
        }
    }
    heap
}

fn distance(c1: Coord, c2: Coord) -> f64 {
    let t = (c1.0 - c2.0).pow(2) as f64 + (c1.1 - c2.1).pow(2) as f64 + (c1.2 - c2.2).pow(2) as f64;
    t.sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/08_input_example_1.txt", 10);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 40);
    }

    #[test]
    fn part_1_input_example_2() {
        let answer = part_1("inputs/08_input_example_2.txt", 5);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/08_input.txt", 1000);

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 103488);
    }

    #[test]
    fn part_2_input_example_1() {
        let answer = part_2("inputs/08_input_example_1.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 25272);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/08_input.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 8759985540);
    }
}
