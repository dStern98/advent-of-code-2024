use std::collections::HashMap;

use super::{read_input_file, SolveAdvent};

pub struct Day20;

type OrderedPair = (i64, i64);

struct RaceTrack {
    track: Vec<Vec<char>>
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


impl SolveAdvent for Day20 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let input_file = read_input_file(path_to_file)?;
        let racetrack = input_file.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let racetrack = RaceTrack::new(racetrack);
        let indexed_racetrack = racetrack.index_racetrack()?;
        let mut cheat_shortcuts: HashMap<i64, usize> = HashMap::new();
        //Explore all possible cheats
        for (track_position, picoseconds) in indexed_racetrack.iter() {
            //For any given cheat start position, the seconds saved are the distance between 
            //the seconds it took the non-cheater to get from the old position (before cheating)
            // to the new position (after cheating) minus the seconds the cheater moved during the cheat.
            let (row, col) = *track_position;
            let possible_cheats=  [
                (row + 1, col + 1),
                (row - 1, col + 1),
                (row + 1, col - 1),
                (row - 1, col - 1),
                (row + 2, col),
                (row - 2, col), 
                (row, col + 2), 
                (row, col - 2)
            ];
            for possible_cheat in possible_cheats {
                if let Some(cheat_picoseconds) = indexed_racetrack.get(&possible_cheat) {
                    let cheat_picoseconds_saved = *cheat_picoseconds  as i64  - *picoseconds as i64 - 2;
                    if cheat_picoseconds_saved > 0 {
                        *cheat_shortcuts.entry(cheat_picoseconds_saved).or_default() += 1;
                    }
                }
            }
        }
        let total_cheats_above_threshold = cheat_shortcuts.into_iter().filter_map(|(picoseconds_saved, count)| {
            if picoseconds_saved >= 100 {
                return Some(count)
            }
            None
        }).sum::<usize>();
        println!("There are {} cheats that save at least 100 picoseconds", total_cheats_above_threshold);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}