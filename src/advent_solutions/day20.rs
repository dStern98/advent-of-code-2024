use std::collections::{HashMap, HashSet};

use super::{read_input_file, SolveAdvent};

pub struct Day20;

type OrderedPair = (i64, i64);

struct RaceTrack {
    track: Vec<Vec<char>>
}

fn generate_all_possible_step_combinations(min_steps: i64, max_steps: i64) -> HashSet<OrderedPair> {
    //!Generate all possible unique combinations of steps starting at position `0,0` that is between
    //! `min_steps` and `max_steps` in length, inclusive on both ends.
    let mut step_combinations = HashSet::new();
    let mut memo = HashSet::new();
    collect_all_step_combinations((0, 0), max_steps, &mut step_combinations, &mut memo);
    step_combinations.retain(|(row_step, col_step)| row_step.abs() + col_step.abs() >= min_steps);
    step_combinations
}

fn collect_all_step_combinations(current_position: OrderedPair, remaining_steps: i64, unique_step_combos: &mut HashSet<OrderedPair>, memo : &mut HashSet<(OrderedPair, i64)>) {
    //! Recursively walk all possible step combinations
    //! The `memo` prevents the runtime from being exponential
    if remaining_steps < 0 || memo.contains(&(current_position, remaining_steps)) {
        return;
    }
    unique_step_combos.insert(current_position);
    memo.insert((current_position, remaining_steps));
    let (current_row, current_col) = current_position;
    collect_all_step_combinations((current_row + 1, current_col), remaining_steps - 1, unique_step_combos, memo);
    collect_all_step_combinations((current_row - 1, current_col), remaining_steps - 1, unique_step_combos, memo);
    collect_all_step_combinations((current_row, current_col + 1), remaining_steps - 1, unique_step_combos, memo);
    collect_all_step_combinations((current_row, current_col - 1), remaining_steps - 1, unique_step_combos, memo);
}

impl RaceTrack {
    fn new(track: Vec<Vec<char>>) -> Self {
        RaceTrack {
            track
        }
    }
    fn reached_end(&self, position: OrderedPair) -> bool {
        //! Whether or not the current `position` is at the End of the racetrack (position `E`)
        if !self.is_valid_position(position) {
            return false;
        }
        let (row, col) = position;
        let current_symbol = self.track[row as usize][col as usize];
        current_symbol == 'E'
    }

    fn is_valid_position(&self,  position: OrderedPair) -> bool {
        //! Whether or not the current `position` is a valid spot on the board.
        //! Walls count as invalid positions.
        let (row, col) = position;
        if row < 0 || col < 0 {
           return false
        }
        if let Some(row) = self.track.get(row as usize) {
            if let Some(symbol) = row.get(col as usize) {
                return symbol != &'#'
            }
        }
        false
    }
    fn find_start_position(&self) -> anyhow::Result<OrderedPair> {
        //! Find the start position for the race.
        for (row_number, row) in self.track.iter().enumerate() {
            for (col_number, symbol) in row.iter().enumerate() {
                if symbol == &'S' {
                    return Ok((row_number as i64, col_number as i64));
                }
            } 
        }
        anyhow::bail!("Now Start position (S) found in racetrack!");
    }

    fn index_racetrack(&self) -> anyhow::Result<HashMap<OrderedPair, usize>> {
        //! Records a map of each position on the board to the number of picoseconds
        //! that have occurred. No cheating is allowed, so the path is deterministic
        let racetrack_start = self.find_start_position()?;
        //Traversal queue stores a tuple of `(track position, picoseconds)`
        let mut traversal_queue = vec![(racetrack_start, 0)];
        let mut track_index = HashMap::new();
        while let Some((current_position, picoseconds)) = traversal_queue.pop() {
            if !self.is_valid_position(current_position) || track_index.contains_key(&current_position) {
                continue;
            }
            track_index.insert(current_position, picoseconds);
            if self.reached_end(current_position) {
                //As soon as we reach the end of the track iteration can end.
                break;
            }
            let (current_row, current_col) = current_position;
            traversal_queue.extend([
                ((current_row, current_col + 1), picoseconds + 1),
                ((current_row, current_col - 1), picoseconds + 1),
                ((current_row + 1, current_col), picoseconds + 1),
                ((current_row - 1, current_col), picoseconds + 1)
            ]);
        }
        Ok(track_index)
    }
}

fn index_cheat_shortcuts(indexed_racetrack: &HashMap<OrderedPair, usize>, possible_cheat_jumps: &HashSet<OrderedPair>) -> HashMap<i64,usize> {
    //! Generate a count of all possible unique cheats that save at least 1 picosecond, when compared to the 
    //! none-cheating variant. 
    let mut cheat_shortcuts: HashMap<i64, usize> = HashMap::new();
    //Explore all possible cheats
    for (track_position, picoseconds) in indexed_racetrack.iter() {
        //For any given cheat start position, the seconds saved are the distance between 
        //the seconds it took the non-cheater to get from the old position (before cheating)
        // to the new position (after cheating) minus the seconds the cheater moved during the cheat.
        let (row, col) = *track_position;
        for (row_step, col_step) in possible_cheat_jumps.iter() {
            let cheat_end_position = (row + row_step, col + col_step);
            let cheat_jump_distance = row_step.abs() + col_step.abs();
            if let Some(end_picoseconds) = indexed_racetrack.get(&cheat_end_position) {
                let cheat_picoseconds_saved = *end_picoseconds  as i64  - *picoseconds as i64 - cheat_jump_distance;
                if cheat_picoseconds_saved > 0 {
                    *cheat_shortcuts.entry(cheat_picoseconds_saved).or_default() += 1;
                }
        }
    }
    }
    cheat_shortcuts
}


impl SolveAdvent for Day20 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let input_file = read_input_file(path_to_file)?;
        let racetrack = input_file.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        //First, gather statistics on the racetrack without cheating
        let indexed_racetrack = RaceTrack::new(racetrack).index_racetrack()?;
        //Allowed cheats must take exactly 2 steps
        let possible_cheat_jumps=  generate_all_possible_step_combinations(2, 2);
        //Find all possible cheats that save at least 1 picosecond
        let cheat_shortcuts = index_cheat_shortcuts(&indexed_racetrack, &possible_cheat_jumps);
        let total_cheats_above_threshold = cheat_shortcuts.into_iter().filter_map(|(picoseconds_saved, count)| {
            if picoseconds_saved >= 100 {
                return Some(count)
            }
            None
        }).sum::<usize>();
        println!("There are {} cheats that save at least 100 picoseconds", total_cheats_above_threshold);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Exact same solution as `part1`, but allowing cheats of between
        //! 2 and 20 picoseconds
        let input_file = read_input_file(path_to_file)?;
        let racetrack = input_file.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let indexed_racetrack = RaceTrack::new(racetrack).index_racetrack()?;
        //Allowed cheats must take between 2 and 20 steps (inclusive on both ends)
        let possible_cheat_jumps=  generate_all_possible_step_combinations(2, 20);
        let cheat_shortcuts = index_cheat_shortcuts(&indexed_racetrack, &possible_cheat_jumps);
        let total_cheats_above_threshold = cheat_shortcuts.into_iter().filter_map(|(picoseconds_saved, count)| {
            if picoseconds_saved >= 100 {
                return Some(count)
            }
            None
        }).sum::<usize>();
        println!("There are {} cheats that save at least 100 picoseconds", total_cheats_above_threshold);
        Ok(())
    }
}