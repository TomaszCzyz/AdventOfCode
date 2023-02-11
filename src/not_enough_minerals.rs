use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::min;

use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Copy, Clone, Debug, Display, EnumIter, Eq, PartialEq, Hash)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type CostInfo = (Mineral, u32);

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: HashMap<Mineral, Vec<CostInfo>>,
}


fn read_input(file_name: &str) -> Vec<Blueprint> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut blueprints = Vec::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let mut costs = HashMap::new();

        // id 1 / ore robot: 3 ore / clay robot: 3 ore / obsidian robot: 3 ore 19 clay / geode robot 3 ore 17 obsidian
        // 1 3 3 3 19 3 17

        let only_numbers = buf.trim()
            .replace("Blueprint ", "")
            .replace(": Each ore robot costs", "")
            .replace(" ore. Each clay robot costs", "")
            .replace(" ore. Each obsidian robot costs", "")
            .replace(" ore and", "")
            .replace(" clay. Each geode robot costs", "")
            .replace(" ore and", "")
            .replace(" obsidian.", "");

        let data = only_numbers.split(' ')
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        let id = data[0] as usize;
        let ore_robot_costs = vec![(Mineral::Ore, data[1])];
        let clay_robot_costs = vec![(Mineral::Ore, data[2])];
        let obsidian_robot_costs = vec![(Mineral::Ore, data[3]), (Mineral::Clay, data[4])];
        let geode_robot_costs = vec![(Mineral::Ore, data[5]), (Mineral::Obsidian, data[6])];

        costs.insert(Mineral::Ore, ore_robot_costs);
        costs.insert(Mineral::Clay, clay_robot_costs);
        costs.insert(Mineral::Obsidian, obsidian_robot_costs);
        costs.insert(Mineral::Geode, geode_robot_costs);

        blueprints.push(Blueprint { id, costs });

        buf = String::new();
    }

    blueprints
}

fn initialize_equipment() -> (HashMap<Mineral, u32>, HashMap<Mineral, u32>) {
    (
        HashMap::from([
            (Mineral::Ore, 1),
            (Mineral::Clay, 0),
            (Mineral::Obsidian, 0),
            (Mineral::Geode, 0),
        ]),
        HashMap::from([
            (Mineral::Ore, 0),
            (Mineral::Clay, 0),
            (Mineral::Obsidian, 0),
            (Mineral::Geode, 0),
        ])
    )
}


/// returns: u32 - number of robots building
fn build_robots(robot_type: &Mineral, minerals: &mut HashMap<Mineral, u32>, costs: &Blueprint) -> u32 {
    let robot_costs = &costs.costs[robot_type];
    let mut affordable_number: Vec<u32> = Vec::new();

    for (mineral, cost) in robot_costs.iter() {
        if minerals[mineral] < *cost {
            return 0;
        } else {
            // println!("minerals[{}] / cost <=> {} / {} = {}",mineral, minerals[mineral], cost, minerals[mineral] / cost);
            affordable_number.push(minerals[mineral] / cost);
        }
    }

    let number_to_build = *affordable_number.iter().min().unwrap();

    for (mineral, cost) in robot_costs.iter() {
        minerals.entry(*mineral).and_modify(|qty| *qty -= cost * number_to_build);
    }

    println!("build {number_to_build} robots of type {robot_type}\n");

    number_to_build
}

fn mine_minerals(robots: &HashMap<Mineral, u32>, minerals: &mut HashMap<Mineral, u32>) {
    for (mineral, qty) in minerals.iter_mut() {
        *qty += robots[mineral];
    }
}

const TOTAL_TIME: u32 = 24 + 1;

/// round phases:
/// 1. spending minerals for robots construction
/// 1. mining minerals
/// 1. finishing robots constructions
pub fn not_enough_minerals_part_1(file_name: &str) -> usize {
    let costs = read_input(file_name);
    let (mut robots, mut minerals) = initialize_equipment();

    println!("{:#?}", costs);

    let mut mining_time_spans: [(Mineral, u32, u32); 4] = [
        (Mineral::Geode, 18, TOTAL_TIME),
        (Mineral::Obsidian, 11, 16),
        (Mineral::Clay, 1, 13),
        (Mineral::Ore, 0, 1),
    ];

    for minute in 1..TOTAL_TIME {
        println!("\n====MINUTE {minute}====");
        print_equipment(&robots, &minerals);

        let mut built_robots = HashMap::new();

        for (mineral, begin, end) in mining_time_spans.iter() {
            if *begin > minute || minute >= *end {
                continue;
            }

            let built_robots_number = build_robots(mineral, &mut minerals, &costs[0]);
            built_robots.insert(*mineral, built_robots_number);
        }

        mine_minerals(&robots, &mut minerals);

        for (type_, qty) in built_robots.into_iter() {
            robots.entry(type_).and_modify(|qty_| *qty_ += qty);
        }

        print_equipment(&robots, &minerals);
    }

    0
}

fn print_equipment(robots: &HashMap<Mineral, u32>, minerals: &HashMap<Mineral, u32>) {
    for mineral in Mineral::iter() {
        println!("{:20} robots:{:3?}  minerals:{:?}", mineral, robots[&mineral], minerals[&mineral]);
    }
    println!();
}
