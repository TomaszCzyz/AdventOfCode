#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

const TOTAL_TIME: u32 = 24;

#[derive(Copy, Clone, Debug, Display, EnumIter, Eq, PartialEq, Hash)]
enum Mineral {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type CostInfo = HashMap<Mineral, u32>;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    costs: HashMap<Mineral, CostInfo>,
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
        let ore_robot_costs = HashMap::from([(Mineral::Ore, data[1])]);
        let clay_robot_costs = HashMap::from([(Mineral::Ore, data[2])]);
        let obsidian_robot_costs = HashMap::from([(Mineral::Ore, data[3]), (Mineral::Clay, data[4])]);
        let geode_robot_costs = HashMap::from([(Mineral::Ore, data[5]), (Mineral::Obsidian, data[6])]);

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

fn try_build_next_robots(
    minute: u32,
    costs: &HashMap<Mineral, CostInfo>,
    robots: HashMap<Mineral, u32>,
    minerals: HashMap<Mineral, u32>,
    history: HashMap<Mineral, Vec<u32>>,
    robots_to_build: Vec<Mineral>,
) -> u32 {
    if minute == TOTAL_TIME {
        print_status(&robots, &minerals, &history);

        return minerals[&Mineral::Geode];
    }

    let geode_obsidian_cost = &costs[&Mineral::Geode][&Mineral::Obsidian];
    if minute >= TOTAL_TIME - geode_obsidian_cost && !history.contains_key(&Mineral::Obsidian) {
        // we wont be able to build any geode robots till 24 minute
        return 0;
    }

    let mut answers = Vec::new();

    // try to build requested robots
    for robot_type in robots_to_build.iter() {
        let robot_costs = &costs[robot_type];
        let mut is_affordable = true;

        // check if there is enough minerals
        for (mineral, cost) in robot_costs.iter() {
            if minerals[mineral] < *cost {
                is_affordable = false;
                break;
            }
        }

        if !is_affordable {
            // postpone the same build list to next minute
            let mut updated_minerals = minerals.clone();
            let updated_robots = robots.clone();

            mine_minerals(&updated_robots, &mut updated_minerals);

            let answer = try_build_next_robots(
                minute + 1,
                costs,
                updated_robots,
                updated_minerals,
                history.clone(),
                robots_to_build.clone());

            answers.push(answer);
        } else { // is affordable
            let mut updated_history = history.clone();
            let mut updated_minerals = minerals.clone();
            let mut updated_robots = robots.clone();

            updated_history.entry(*robot_type)
                .and_modify(|minutes| minutes.push(minute))
                .or_insert_with(|| vec![minute]);

            // spend minerals to build a robot
            for (mineral, cost) in robot_costs.iter() {
                updated_minerals.entry(*mineral).and_modify(|qty| *qty -= cost);
            }

            // mine new minerals
            mine_minerals(&updated_robots, &mut updated_minerals);

            // finish construction of a new robot
            updated_robots.entry(*robot_type).and_modify(|robots_number| *robots_number += 1);

            // make new build request
            let new_robots_to_build: Vec<Mineral> = create_build_request(*robot_type, minute, &updated_history);

            let answer = try_build_next_robots(
                minute + 1,
                costs,
                updated_robots,
                updated_minerals,
                updated_history,
                new_robots_to_build);

            answers.push(answer);
        }
    }

    *answers.iter().max().unwrap()
}

fn create_build_request(robot_type: Mineral, _current_minute: u32, _history: &HashMap<Mineral, Vec<u32>>) -> Vec<Mineral> {
    match robot_type {
        Mineral::Ore => vec![Mineral::Ore, Mineral::Clay],
        Mineral::Clay => vec![Mineral::Clay, Mineral::Obsidian],
        Mineral::Obsidian => vec![Mineral::Obsidian, Mineral::Geode],
        Mineral::Geode => vec![Mineral::Geode],
    }
    // match (history.contains_key(&Mineral::Ore), history.contains_key(&Mineral::Clay), history.contains_key(&Mineral::Obsidian), history.contains_key(&Mineral::Geode)) {
    //     (true, true, true, true) => vec![Mineral::Geode],
    //     (true, true, true, false) => {
    //         let most_recent_clay = history[&Mineral::Clay].iter().last().unwrap();
    //         if current_minute < most_recent_clay + 3 {
    //             vec![Mineral::Clay, Mineral::Obsidian, Mineral::Geode]
    //         } else {
    //             vec![Mineral::Obsidian, Mineral::Geode]
    //         }
    //     }
    //     (true, true, false, false) => {
    //         let most_recent_ore = history[&Mineral::Ore].iter().last().unwrap();
    //         if current_minute < most_recent_ore + 3 {
    //             vec![Mineral::Ore, Mineral::Clay, Mineral::Obsidian]
    //         } else {
    //             vec![Mineral::Obsidian, Mineral::Geode]
    //         }
    //     }
    //     (true, false, false, false) => {
    //         vec![Mineral::Ore, Mineral::Clay]
    //     }
    //     _ => {
    //         vec![Mineral::Ore, Mineral::Clay]
    //     }
    // }
}

fn print_status(robots: &HashMap<Mineral, u32>, minerals: &HashMap<Mineral, u32>, history: &HashMap<Mineral, Vec<u32>>) {
    println!("FINISHED");
    for (val, min) in history.iter()
        .flat_map(|(key, values)| {
            values.iter().map(move |value| (key, *value))
        })
        .sorted_by(|(_, min1), (_, min2)| min1.cmp(min2))
        .collect::<Vec<_>>() {
        println!("minute {min} - build {val} robot");
    }
    print_equipment(robots, minerals);
}

/// round phases:
/// 1. spending minerals for robots construction
/// 1. mining minerals
/// 1. finishing robots constructions
fn analyze_next_minute(
    minute: u32,
    robots: HashMap<Mineral, u32>,
    minerals: HashMap<Mineral, u32>,
    costs: &Blueprint,
    history: Vec<(Option<Mineral>, u32)>,
) -> u32 {
    if minute == TOTAL_TIME {
        println!("FINISHED");
        for (mineral, min) in history {
            match mineral {
                None => {
                    // println!("minute {min} - do nothing"),
                }
                Some(val) => println!("minute {min} - build {val} robot"),
            }
        }
        print_equipment(&robots, &minerals);

        return minerals[&Mineral::Geode];
    }

    if minute >= TOTAL_TIME - 7 && !history.iter().any(|(mineral, _)| mineral.is_some() && mineral.unwrap() == Mineral::Obsidian) {
        return 0;
    }

    let mut allowed_minerals: Vec<Mineral> = vec![Mineral::Geode, Mineral::Obsidian, Mineral::Clay, Mineral::Ore];
    let _ = allowed_minerals[0];

    let filtered_history = history.iter()
        .filter(|(miner, _min)| miner.is_some())
        .map(|(miner, min)| (miner.unwrap(), min));

    match filtered_history.clone().last() {
        None => { // no robot was build yet
            allowed_minerals = vec![Mineral::Clay, Mineral::Ore]
        }
        Some((miner, _min)) => {
            allowed_minerals = match miner {
                Mineral::Ore => vec![Mineral::Clay, Mineral::Ore],
                Mineral::Clay => vec![Mineral::Obsidian, Mineral::Clay],
                Mineral::Obsidian => vec![Mineral::Geode, Mineral::Obsidian],
                Mineral::Geode => vec![Mineral::Geode],
            }

            // append "lower" mineral if...
        }
    }

    println!("\tallowed minerals in minute {minute} are: {:?}", allowed_minerals);

    let mut answers = Vec::new();
    let mut was_anything_build = false;

    // choice: try buy a robot
    for mineral in allowed_minerals {
        let robot_costs = &costs.costs[&mineral];
        let mut is_affordable = true;

        // check if there is enough minerals
        for (mineral, cost) in robot_costs.iter() {
            if minerals[mineral] < *cost {
                is_affordable = false;
                break;
            }
        }

        if is_affordable {
            was_anything_build = true;
            let mut updated_history = history.clone();
            let mut updated_minerals = minerals.clone();
            let mut updated_robots = robots.clone();

            updated_history.push((Some(mineral), minute));

            // spend minerals to build a robot
            for (mineral, cost) in robot_costs.iter() {
                updated_minerals.entry(*mineral).and_modify(|qty| *qty -= cost);
            }

            // mine new minerals
            mine_minerals(&updated_robots, &mut updated_minerals);

            // finish construction of a new robot
            updated_robots.entry(mineral).and_modify(|robots_number| *robots_number += 1);

            let answer = analyze_next_minute(minute + 1, updated_robots, updated_minerals, costs, updated_history);
            answers.push(answer);
        }
    }

    if !was_anything_build {
        // choice: do nothing
        let mut updated_minerals = minerals;
        let mut updated_history = history.clone();

        updated_history.push((None, minute));

        mine_minerals(&robots, &mut updated_minerals);

        let answer = analyze_next_minute(minute + 1, robots.clone(), updated_minerals, costs, updated_history);
        answers.push(answer);
    }

    // // choice: do nothing
    // let mut updated_minerals = minerals;
    // let mut updated_history = history.clone();
    //
    // updated_history.push((None, minute));
    //
    //
    // mine_minerals(&robots, &mut updated_minerals);
    //
    // let answer = analyze_next_minute(minute + 1, robots.clone(), updated_minerals, costs, updated_history);
    // answers.push(answer);

    *answers.iter().max().unwrap()
}

pub fn not_enough_minerals_part_1(file_name: &str) -> u32 {
    let costs = read_input(file_name);
    let (robots, minerals) = initialize_equipment();

    println!("{:#?}", costs);

    // let max_geode_qty = analyze_next_minute(1, robots, minerals, &costs[0], Vec::new());
    let max_geode_qty = try_build_next_robots(
        1,
        &costs[0].costs,
        robots,
        minerals,
        HashMap::from([(Mineral::Ore, vec![0])]),
        Vec::from([Mineral::Clay, Mineral::Ore]));

    println!("max geode qty for blueprint {} is {}", costs[0].id, max_geode_qty);
    max_geode_qty
}

fn print_equipment(robots: &HashMap<Mineral, u32>, minerals: &HashMap<Mineral, u32>) {
    for mineral in Mineral::iter() {
        println!("\t{:20} robots:{:3?}  minerals:{:?}", mineral, robots[&mineral], minerals[&mineral]);
    }
    println!();
}
