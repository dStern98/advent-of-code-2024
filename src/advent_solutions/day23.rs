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

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let mut connection_topology = construct_topology(&file_contents)?;
        for (computer, connected_computers) in connection_topology.iter_mut() {
            connected_computers.insert(computer.to_string());
        }

        let mut optimizer = HashSet::new();
        let mut largest_totally_connected_network = HashSet::new();
        for starting_computer in connection_topology.keys() {
            let largest_network = find_largest_connected_network( &mut optimizer, &starting_computer, &connection_topology);
            if largest_network.len() > largest_totally_connected_network.len() {
                largest_totally_connected_network = largest_network;
            }
        }
        let mut lan_network_password = largest_totally_connected_network.into_iter().collect::<Vec<_>>();
        lan_network_password.sort();
        println!("LAN Password is {:?}", lan_network_password.join(","));
        Ok(())
    }
}


fn find_largest_connected_network( optimizer: &mut HashSet<(String, String)>, starting_position: &str, network: &ConnectionTopology) -> HashSet<String> {
    let mut largest_connected_network = HashSet::new();
    let mut traversal_queue = vec![NetworkTraveler::new(starting_position.to_string(), HashSet::new(), network)];
    while let Some(mut current_traveler) = traversal_queue.pop() {
        let mut visited =  current_traveler.visited.clone().into_iter().collect::<Vec<_>>();
        visited.sort();
        let optimizer_insert = (visited.join(","),current_traveler.current.clone());
        if current_traveler.in_cycle() || optimizer.contains(&optimizer_insert)  {
            continue;    
        }
        current_traveler.visit();
        optimizer.insert(optimizer_insert);
        let perfectly_connnected = current_traveler.perfectly_connected();
        if !perfectly_connnected {
            continue;
        }
        if perfectly_connnected && current_traveler.visited.len() > largest_connected_network.len() {
            largest_connected_network = current_traveler.visited.clone();
        }
        traversal_queue.extend(current_traveler.spawn_next());

    }
    largest_connected_network
}

struct NetworkTraveler<'a> {
    visited: HashSet<String>,
    current: String,
    network: &'a ConnectionTopology
}

impl <'a>NetworkTraveler <'a> {
    fn new(current: String, visited: HashSet<String>, network: &'a ConnectionTopology) -> Self {
        NetworkTraveler {
            current, 
            visited,
            network
        }
    }
    fn in_cycle(&self) -> bool {
        self.visited.contains(&self.current)
    }
    fn visit(&mut self) {
        self.visited.insert(self.current.clone());
    }
    fn spawn_next(&self) -> Vec<Self> {
        self.network.get(&self.current).unwrap().iter().map(|next_computer| NetworkTraveler::new(next_computer.clone(),  self.visited.clone(), &self.network )).collect::<Vec<_>>()
    }

    fn perfectly_connected(&self) -> bool {
        for computer in self.visited.iter() {
            if !self.visited.is_subset(self.network.get(computer).unwrap()) {
                return false;
            }
        }
        true
    }
}