use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Debug)]
pub struct Mapping {
    source: usize,
    destination: usize,
    len: usize,
}

impl Mapping {
    fn map(&self, value: usize) -> Option<usize> {
        let source = self.source;
        if source <= value && value < source + self.len {
            let mapped_value = self.destination + value - source;

            return Some(mapped_value);
        }

        None
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum ResourceType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location,
}

type ResourcePair = (ResourceType, ResourceType);

const RESOURCE_CHAIN: [ResourcePair; 7] = [
    (ResourceType::Seed, ResourceType::Soil),
    (ResourceType::Soil, ResourceType::Fertilizer),
    (ResourceType::Fertilizer, ResourceType::Water),
    (ResourceType::Water, ResourceType::Light),
    (ResourceType::Light, ResourceType::Temp),
    (ResourceType::Temp, ResourceType::Humidity),
    (ResourceType::Humidity, ResourceType::Location),
];

type Seeds = Vec<usize>;

pub fn read_input(file_name: &str) -> (Seeds, HashMap<ResourcePair, Vec<Mapping>>) {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    // read seeds
    _ = reader.read_line(&mut buf).unwrap();
    let seeds = buf.trim()
        .trim_start_matches("seeds: ")
        .split(' ')
        .filter_map(|num| num.trim().parse::<usize>().ok())
        .collect::<Vec<_>>();
    buf = String::new();
    _ = reader.read_line(&mut String::new()).unwrap();

    let mut resource_mappings = HashMap::new();

    for resource_pair in RESOURCE_CHAIN.into_iter() {
        let mut mappings = Vec::new();

        // skip an empty line and the line with description
        _ = reader.read_line(&mut String::new()).unwrap();
        while let Ok(n) = reader.read_line(&mut buf) {
            if n == 0 || buf == "\r\n" {
                break;
            }

            let mapping_data = &mut buf.trim()
                .split(' ')
                .filter_map(|num| num.trim().parse::<usize>().ok())
                .collect::<Vec<_>>();

            mappings.push(Mapping {
                source: mapping_data[1],
                destination: mapping_data[0],
                len: mapping_data[2],
            });

            buf = String::new();
        }

        resource_mappings.insert(resource_pair, mappings);
    }

    (seeds, resource_mappings)
}

/// find location value for given seed
fn calc_location(mut value: usize, resource_mappings: &HashMap<ResourcePair, Vec<Mapping>>) -> usize {
    for resource_pair in RESOURCE_CHAIN.into_iter() {
        for mapping in resource_mappings.get(&resource_pair).unwrap().iter() {
            if let Some(new_val) = mapping.map(value) {
                value = new_val;
                break;
            }
        }
    }

    value
}

fn seed_fertilizer_part_1(filename: &str) -> usize {
    let (seeds, resource_mappings) = read_input(filename);

    seeds.iter().map(|value| calc_location(*value, &resource_mappings)).min().unwrap()
}

fn seed_fertilizer_part_2(_filename: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_test() {
        let (seeds, mappings) = read_input("inputs/5_input_example.txt");

        println!("seeds: {seeds:?}");
        for (resource_pair, mappings) in mappings.iter() {
            println!("{resource_pair:?}: {mappings:?}");
        }
    }

    #[test]
    fn part_1_input_example() {
        let answer = seed_fertilizer_part_1("inputs/5_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 35);
    }

    #[test]
    fn part_1_input() {
        let answer = seed_fertilizer_part_1("inputs/5_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 157211394);
    }

    #[test]
    fn part_2_input_example() {
        let answer = seed_fertilizer_part_2("inputs/5_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }

    #[test]
    fn part_2_input() {
        let answer = seed_fertilizer_part_2("inputs/5_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }
}
