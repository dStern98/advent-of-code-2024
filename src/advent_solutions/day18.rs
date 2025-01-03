use std::collections::{HashMap, HashSet, VecDeque};

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

fn build_corrupted_bytes(input_file: &str, size_limit: usize) -> anyhow::Result<HashSet<OrderedPair>> {
    //! Construct a set of the corruped byte positions.
    let mut corruped_bytes = HashSet::new();
    for (line_number, line) in input_file.lines().enumerate() {
        if line_number == size_limit {
            break;
        }
        //Input file is col, row but we want row, col
        let [col, row]: [&str; 2] = line.split(',').collect::<Vec<_>>().try_into().map_err(|_| anyhow::anyhow!("Failed to split coordinates"))?;
        let row = row.parse::<i64>()?;
        let col = col.parse::<i64>()?;
        corruped_bytes.insert((row, col));
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


impl SolveAdvent for Day18 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Depth first traversal of the memory space to find the optimal path.
        //! The `Optimizer` keeps the runtime from being exponential
        let file_contents = read_input_file(path_to_file)?;
        let mut optimizer = Optimizer::new();
        let corruped_bytes = build_corrupted_bytes(&file_contents, 1024)?;
        let grid_size = (70, 70);
        let mut explorer_queue = VecDeque::new();
        explorer_queue.push_back(MemoryTraveler::new(0, 0));
        let mut min_path_taken = usize::MAX;
        while let Some(mut current_explorer) = explorer_queue.pop_front() {
            if current_explorer.in_cycle() || corruped_bytes.contains(&(current_explorer.row, current_explorer.col)) || !current_explorer.is_inbounds(grid_size) {
                continue;
            }
            if optimizer.kill_probe(&current_explorer) {
                continue;
            }
            current_explorer.visit();
            if current_explorer.exit_reached(grid_size) {
                min_path_taken = min_path_taken.min(current_explorer.visited.len());
            }
            explorer_queue.extend(current_explorer.spawn_next());
        }
        println!("Minimum path to exit is {} steps", min_path_taken - 1);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}