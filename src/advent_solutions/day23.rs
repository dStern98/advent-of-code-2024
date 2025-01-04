use std::collections::{BTreeSet, HashMap, HashSet};

use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day23;

type ConnectionTopology = HashMap<String, HashSet<String>>;

fn construct_topology(input_file: &str) -> anyhow::Result<ConnectionTopology> {
    //! Construct a bi-directional topology map of computer network connections.
    let mut connection_topology: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input_file.lines() {
        let [computer1, computer2]: [&str; 2] = line.split('-').collect::<Vec<_>>().try_into().map_err(|_| anyhow!("Cannot coerce into length 2 array"))?;
        connection_topology.entry(computer1.to_string()).or_default().insert(computer2.to_string());
        connection_topology.entry(computer2.to_string()).or_default().insert(computer1.to_string());
    }
    Ok(connection_topology)
}

fn traverse_topology_map_recursively(topology_map: &ConnectionTopology, current: &str, mut history: Vec<String>) -> Vec<Vec<String>> {
    //! Recursively traverse the topology tree to collect all length 3 computer connections with at least one computer
    //! starting with the letter 't'. If the `history` is length 3 AND the current computer was the starting computer, then 
    //! we have successfully found a length 3 computer connection.
    if history.len() == 3 && current == history[0] && history.iter().any(|computer| computer.starts_with('t')) {
        return vec![history];
    }
    if history.len()> 3 {
        return vec![];
    }
    history.push(current.to_string());
    let mut collected_histories = Vec::new();
    for next_computer in topology_map.get(current).unwrap() {
        collected_histories.extend(traverse_topology_map_recursively(topology_map, next_computer, history.clone()));
    }
    collected_histories
}


impl SolveAdvent for Day23 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let connection_topology = construct_topology(&file_contents)?;
        //We use a b-tree set because we need to keep track of unique vectors, but vectors are not hash, so they
        //cannot go into a set.
        let mut valid_computer_topologies = BTreeSet::new();
        for computer in connection_topology.keys() {
            let visited_computers = traverse_topology_map_recursively(&connection_topology, computer, Vec::new());
            for mut computer_three_group in visited_computers {
                computer_three_group.sort();
                if !valid_computer_topologies.contains(&computer_three_group) {
                    valid_computer_topologies.insert(computer_three_group);
                }
            }
            
        }
        println!("There are {} unique length-3 computer groups that start with t", valid_computer_topologies.len());
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}