use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day18;

type OrderedPair = (i64, i64);

///Represents a single path through the memory
/// maze
#[derive(Debug, Clone)]
struct MemoryTraveler {
    ///Current row position
    row: i64,
    ///Current column position
    col: i64,
    ///This travelers history (to prevent cycles)
    visited: HashSet<OrderedPair>
}

impl MemoryTraveler {
    fn visit(&mut self) {
        //! Add current position to this traveler's
        //! history of visited positions
        self.visited.insert((self.row, self.col));
    }

    fn in_cycle(&self)-> bool {
        //! Whether the probe is in a cycle
        self.visited.contains(&(self.row, self.col))
    }

    fn new(row: i64, col: i64) -> Self {
        MemoryTraveler {
            row,
            col, 
            visited:HashSet::new()
        }
    }
    fn is_inbounds(&self, grid: OrderedPair) -> bool {
        let (max_row, max_col) = grid;
        self.row >= 0 && self.col >= 0 && self.col <= max_col && self.row <= max_row
    }
    fn exit_reached(&self, grid: OrderedPair) -> bool {
        //! Has this probe reached the bottom right corner of the memory space?
        let (max_row, max_col) = grid;
        self.row ==max_row && self.col ==max_col
    }
    fn spawn_next(&self) -> impl IntoIterator<Item = MemoryTraveler> {
        //! Spawn 4 new probes, 1 up, 1 down, 1 left, 1 right.
        [
            MemoryTraveler {
                row: self.row + 1,
                col: self.col, 
                visited: self.visited.clone()
            },
            MemoryTraveler {
                row: self.row - 1,
                col: self.col,
                visited: self.visited.clone()
            },
            MemoryTraveler {
                row: self.row, 
                col: self.col + 1, 
                visited: self.visited.clone()
            },
            MemoryTraveler {
                row: self.row, 
                col: self.col - 1, 
                visited: self.visited.clone()
            }
        ]
    }
}

fn build_corrupted_bytes(input_file: &str, size_limit: usize) -> anyhow::Result<Vec<OrderedPair>> {
    //! Construct a set of the corruped byte positions.
    let mut corruped_bytes = Vec::new();
    for (line_number, line) in input_file.lines().enumerate() {
        if line_number == size_limit {
            break;
        }
        //Input file is col, row but we want row, col
        let [col, row]: [&str; 2] = line.split(',').collect::<Vec<_>>().try_into().map_err(|_| anyhow::anyhow!("Failed to split coordinates"))?;
        let row = row.parse::<i64>()?;
        let col = col.parse::<i64>()?;
        corruped_bytes.push((row, col));
    }
    Ok(corruped_bytes)
}


///Optimizer is a blackbox that helps efficiently
/// kill probes that cannot be on an optimal path.
struct Optimizer {
    best_records: HashMap<OrderedPair, usize>,
}

impl Optimizer {
    fn new() -> Self {
        Optimizer {
            best_records: HashMap::new(),
        }
    }
    fn kill_probe(&mut self, current_explorer: &MemoryTraveler) -> bool {
        //! Whether or not to kill this probe. 
        let current_explorer_position = (current_explorer.row, current_explorer.col); 
        match self.best_records.get_mut(&current_explorer_position) {
            Some(best_record) => {
                if current_explorer.visited.len() >= *best_record {
                    //If another probe has done better or as good as this probe,
                    //then we can kill this probe
                    return true;
                }
                *best_record = current_explorer.visited.len();
            },
            None => {
                self.best_records.insert(current_explorer_position, current_explorer.visited.len());
            }
        }
        false
    }
}

fn find_shortest_path(corrupted_bytes: &HashSet<OrderedPair>, grid_size: OrderedPair) -> Option<usize> {
    //! Depth first traversal of the memory space to find the optimal path (the path with the fewest steps)
    //! The `Optimizer` keeps the runtime from being exponential
    //! If no optimal path exists, then `None` is returned.
    let mut explorer_queue = VecDeque::new();
    explorer_queue.push_back(MemoryTraveler::new(0, 0));
    let mut optimizer = Optimizer::new();
    let mut min_path_taken: Option<usize> = None;
    while let Some(mut current_explorer) = explorer_queue.pop_front() {
        if current_explorer.in_cycle() || corrupted_bytes.contains(&(current_explorer.row, current_explorer.col)) || !current_explorer.is_inbounds(grid_size) {
            continue;
        }
        if optimizer.kill_probe(&current_explorer) {
            continue;
        }
        current_explorer.visit();
        if current_explorer.exit_reached(grid_size) {
            match min_path_taken {
                Some(current_min_path_taken) => min_path_taken = Some(current_min_path_taken.min(current_explorer.visited.len())), 
                None => min_path_taken = Some(current_explorer.visited.len())
            }
        }
        explorer_queue.extend(current_explorer.spawn_next());
    }
    min_path_taken
}


impl SolveAdvent for Day18 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Traverse all possible paths to find the shortest path
        let file_contents = read_input_file(path_to_file)?;
        let corrupted_bytes = build_corrupted_bytes(&file_contents, 1024)?.into_iter().collect::<HashSet<_>>();
        let grid_size = (70, 70);
        let shortest_path = find_shortest_path(&corrupted_bytes, grid_size).ok_or(anyhow!("No path found at all!"))?;
        println!("Minimum path to exit is {} steps", shortest_path - 1);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! For each byte we corrupt, check if this byte makes traversing the 
        //! memory map from start to exit impossible.
        let file_contents = read_input_file(path_to_file)?;
        let grid_size = (70, 70);
        let all_corrupted_bytes = build_corrupted_bytes(&file_contents, file_contents.lines().count())?;
        let mut corrupted_bytes = HashSet::new();
        for corrupted_byte in all_corrupted_bytes  {
            corrupted_bytes.insert(corrupted_byte);
            if find_shortest_path(&corrupted_bytes, grid_size).is_none() {
                //We are done when there is no optimal path at all!
                let (break_row, break_col) = corrupted_byte;
                println!("Coordinates (as col,row) of first byte that prevents exit is: {break_col},{break_row}");
                break;
            }
        }
        Ok(())
    }
}