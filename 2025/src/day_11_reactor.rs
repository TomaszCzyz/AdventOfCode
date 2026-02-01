use std::collections::{HashMap, VecDeque};
use std::{fs, iter};

type Neighbors = Vec<usize>;

struct Input {
    neighbors_list: Vec<Neighbors>,
    name_to_index: HashMap<String, usize>,
    index_to_name: HashMap<usize, String>,
}

fn read_input(file_name: &str) -> Input {
    let mut rows_parsed = Vec::<(&str, Vec<&str>)>::new();
    let mut name_to_index = HashMap::new();
    let mut index_to_name = HashMap::new();

    let content = fs::read_to_string(file_name).unwrap();

    content.lines().for_each(|line| {
        let line_elements = line.split(": ").collect::<Vec<_>>();
        let name = line_elements[0];
        let neighbors = line_elements[1].split_whitespace().collect::<Vec<_>>();

        rows_parsed.push((name, neighbors.clone()));
        index_to_name.insert(rows_parsed.len() - 1, name.to_string());
        name_to_index.insert(name.to_string(), rows_parsed.len() - 1);
    });

    index_to_name.insert(rows_parsed.len(), "out".to_string());
    name_to_index.insert("out".to_string(), rows_parsed.len());

    let mut rows = vec![vec![]; rows_parsed.len() + 1];

    for (name, neighbors) in rows_parsed.into_iter() {
        let index = name_to_index[name];
        for neighbor in neighbors {
            rows[index].push(name_to_index[neighbor]);
        }
    }

    Input {
        neighbors_list: rows,
        name_to_index,
        index_to_name,
    }
}

fn part_1(filename: &str, input_vertx_name: &str) -> usize {
    let input = read_input(filename);

    let end_vertex = input.name_to_index["out"];
    let start_vertex = input.name_to_index[input_vertx_name];

    let mut paths_count = 0;
    let mut queue = VecDeque::from([start_vertex]);

    while let Some(vertex) = queue.pop_front() {
        if vertex == end_vertex {
            paths_count += 1;
            continue;
        }

        for neighbor in &input.neighbors_list[vertex] {
            queue.push_back(*neighbor);
        }
    }

    paths_count
}

fn part_2(filename: &str, input_vertx_name: &str) -> i64 {
    let input = read_input(filename);

    // print_input(&input.neighbors_list, &input.index_to_name);

    let end_vertex = input.name_to_index["out"];
    let start_vertex = input.name_to_index[input_vertx_name];

    let dac_vertex = input.name_to_index["dac"];
    let fft_vertex = input.name_to_index["fft"];

    let mut paths_count = 0;
    let mut queue = VecDeque::from([(start_vertex, vec![start_vertex])]);

    while let Some((vertex, path)) = queue.pop_front() {
        // println!(
        //     "visiting vertex: {} - path: {:?}",
        //     vertex,
        //     path.iter()
        //         .map(|&v| &input.index_to_name[&v])
        //         .collect::<Vec<_>>()
        // );

        if vertex == end_vertex {
            if path.contains(&dac_vertex) && path.contains(&fft_vertex) {
                paths_count += 1;
            }

            println!(
                "Path: {:?}",
                path.iter()
                    .map(|&v| &input.index_to_name[&v])
                    .collect::<Vec<_>>()
            );
            continue;
        }

        for neighbor in &input.neighbors_list[vertex] {
            let new_path = path
                .iter()
                .copied()
                .chain(iter::once(*neighbor))
                .collect::<Vec<_>>();

            queue.push_back((*neighbor, new_path));
        }
    }

    paths_count
}

fn print_input(input: &Vec<Neighbors>, translations: &HashMap<usize, String>) {
    for (index, row) in input.iter().enumerate() {
        print!("{}: ", translations[&index]);
        for neighbor in row {
            print!("  {}", translations[neighbor]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_input_example_1() {
        let answer = part_1("inputs/11_input_example_1.txt", "you");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 5);
    }

    #[test]
    fn part_1_input() {
        let answer = part_1("inputs/11_input.txt", "you");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 640);
    }

    #[test]
    fn part_2_input_example_2() {
        let answer = part_2("inputs/11_input_example_2.txt", "you");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part_2_input() {
        let answer = part_2("inputs/11_input.txt", "you");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 111);
    }
}
// myw: vgt gbv wbp dac
// dpf: ojt myw nvw
// kso: ugq xhb axs yky lmw fqi iis sju yma iny nvt dpf ogs zrb lfh coa ivx fef phv hnu
// luw: kso dgp
// yzw: che snn luw
// mem: pmu lcu yzw
// oxe: ntx mem tkx bkg zxt puu
// lvm: exf oxe
// qwu: lvm rfn ifx
// rcy: msn qwu
// fwy: anz rcy
// myd: uer rkw elr jix tgh iup lfl vrj ksy kdn jrm kgz ett cel kzi fwy xwk rmc sgr xop oio tig
// rhg: myd fkk
// lno: rhg she reg
// rvi: lno fpd bgp
// hpx: xpo zdg tuf rvi
// axc: hpx gep hza
// mfc: wvx xjb ndt utx oxi xhs ujk swi ygx ond axc myf qnp jnm bhf jol qqq
// rjo: mfc
