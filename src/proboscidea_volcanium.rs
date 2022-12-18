use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

use itertools::Itertools;

type AdjGraph = HashMap<String, HashSet<String>>;
type MatGraph = Vec<Vec<u32>>;
type ValveRates = HashMap<String, u32>;

fn read_input(file_name: &str) -> (AdjGraph, ValveRates) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut input_data: Vec<(String, u32, Vec<String>)> = Vec::new();

    loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 {
                break;
            }

            let data = buf.trim()
                .replace("Valve ", "")
                .replace(" has flow rate=", "|")
                .replace("; tunnels lead to valves ", "|")
                .replace("; tunnel leads to valve ", "|");

            let data = data
                .split('|')
                .collect::<Vec<_>>();

            let valve_name = data[0].to_string();
            let valve_rate = data[1].parse::<u32>().unwrap();
            let list = data[2].split(',').map(|x| x.trim().to_string()).collect::<Vec<_>>();

            input_data.push((valve_name, valve_rate, list))
        }
    }

    let mut network: AdjGraph = HashMap::new();
    let mut valve_rates: ValveRates = HashMap::new();

    for (valve_name, valve_rate, list) in input_data.into_iter() {
        valve_rates.insert(valve_name.clone(), valve_rate);
        network.insert(valve_name, HashSet::from_iter(list));
    }

    (network, valve_rates)
}

fn dijkstra(graph: &AdjGraph, source_name: &str) -> HashMap<String, u32> {
    let mut dist = HashMap::new();
    let mut prev = HashMap::new();
    let mut queue = HashSet::new();

    for (name, _adj_list) in graph.iter() {
        dist.insert(name.clone(), u32::MAX);
        prev.insert(name.clone(), "".to_string());
        queue.insert(name.clone());
    }
    *dist.get_mut(source_name).unwrap() = 0;

    while !queue.is_empty() {
        let vertex_name = &queue.iter()
            .min_by(|&name1, &name2| (dist[name1]).cmp(&dist[name2]))
            .unwrap()
            .clone();

        queue.remove(vertex_name);

        for neighbor in graph[vertex_name].intersection(&queue) {
            let current_dist = dist[vertex_name] + 1;

            if current_dist < dist[neighbor] {
                *dist.get_mut(neighbor).unwrap() = current_dist;
                prev.insert(neighbor.clone(), vertex_name.clone());
            }
        }
    }

    dist
}

fn make_complete_graph(input_graph: &AdjGraph, rates: &ValveRates) -> (MatGraph, HashMap<String, usize>) {
    let non_zero_vertices_count = input_graph.keys().filter(|&key| rates[key] != 0 || key == "AA").count();
    let mut matrix = vec![vec![0_u32; non_zero_vertices_count]; non_zero_vertices_count];
    let mut name_to_index_mappings = HashMap::new();

    let non_zero_vertices = input_graph.keys()
        .filter(|&key| rates[key] != 0 || key == "AA")
        .collect::<Vec<_>>();

    for (index, vertex) in non_zero_vertices.iter().enumerate() {
        name_to_index_mappings.insert((*vertex).clone(), index);
    }

    for (row_vertex, row_index) in name_to_index_mappings.iter() {
        let shortest_paths = dijkstra(input_graph, row_vertex);

        for (col_vertex, col_index) in name_to_index_mappings.iter() {
            matrix[*row_index][*col_index] = shortest_paths[col_vertex];
        }
    }

    (matrix, name_to_index_mappings)
}

fn bfs(
    graph: &MatGraph,
    starting_node: usize,
    my_minutes_left: u32,
    my_opened_valves: Vec<(usize, u32)>,
    rates: &HashMap<usize, u32>,
) -> Option<u32> {
    let mut can_go_anywhere = false;
    let mut inner_results = Vec::new();
    let (my_current_node, _my_last_minute) = my_opened_valves.last().unwrap();

    for (neighbor_index, dist) in graph[*my_current_node].iter().enumerate() {
        if neighbor_index == starting_node
            || neighbor_index == *my_current_node
            || *dist + 1 > my_minutes_left
            || my_opened_valves.iter().map(|(idx, _)| idx).contains(&neighbor_index) {
            continue;
        }
        can_go_anywhere = true;

        let new_minutes_left = my_minutes_left - dist - 1;
        let new_opened_valves = my_opened_valves.clone()
            .into_iter()
            .chain(iter::once((neighbor_index, 30 - new_minutes_left)))
            .collect::<Vec<_>>();

        let result = bfs(
            graph,
            starting_node,
            new_minutes_left,
            new_opened_valves,
            rates);

        if let Some(pressure) = result {
            inner_results.push(pressure)
        }
    }

    if !can_go_anywhere {
        let mut sum = 0;
        for (valve_index, minute) in my_opened_valves.iter() {
            let valve_rate = rates[valve_index];
            sum += valve_rate * (30 - minute);
        }

        return Some(sum);
    }

    Some(*inner_results.iter().max().unwrap())
}

fn translate(rates: &ValveRates, mappings: &HashMap<String, usize>) -> HashMap<usize, u32> {
    rates.iter()
        .filter(|&(name, _)| mappings.contains_key(name))
        .map(|(name, rate)| (mappings[name], *rate))
        .collect::<HashMap<_, _>>()
}

pub fn proboscidea_volcanium_part_1(file_name: &str) -> u32 {
    let (input_graph, valve_rates) = read_input(file_name);
    let (complete_graph, mappings) = make_complete_graph(&input_graph, &valve_rates);

    print(&complete_graph, &mappings);

    let start = mappings.get("AA").unwrap();
    let rates_translated: HashMap<usize, u32> = translate(&valve_rates, &mappings);

    let r = bfs(
        &complete_graph,
        *start,
        30,
        vec![(*start, 30)],
        &rates_translated);

    r.unwrap()
}

fn print(matrix: &MatGraph, name_mappings: &HashMap<String, usize>) {
    let pairs_in_order = name_mappings.iter()
        .sorted_by(|&(_, i1), &(_, i2)| i1.cmp(i2))
        .collect::<Vec<_>>();

    print!("   ");
    for (name, _index) in pairs_in_order.iter() {
        print!("{} ", *name)
    }
    println!();

    for (row_name, row_index) in pairs_in_order.iter() {
        print!("{} ", row_name);
        for (_col_name, col_index) in pairs_in_order.iter() {
            print!("{:2} ", matrix[**row_index][**col_index]);
        }
        println!()
    }
}