use std::collections::HashSet;

use super::{read_input_file, SolveAdvent};

pub struct Day10;


///Represents a path through the Trail Map.
/// The unique requirement that the height always increase by 1 allows
/// cycles to be avoided without a set to store this `TrailBlazer`'s history, 
/// as is common in these kinds of problems.
#[derive(Debug, Clone)]
struct TrailBlazer {
    row: i64,
    col: i64,
    ///Represents the last_height traversed on this trail path.
    last_height: Option<u32>,
}

///The two key statistics of interest for a trail.
struct TrailResult {
    ///Represents the number of unique paths from height 0 to height 9
    rank: usize,
    ///Represents the number of different height 9's that can 
    /// be reached starting at a given trailhead.
    score: usize
}

impl TrailBlazer {
    fn is_inbounds(&self, trail_map: &[Vec<u32>]) -> bool {
        //! Returns true if the `TrailBlazer` is inbounds according to its
        //! current position
        self.row >= 0 && self.col >= 0 && self.row < trail_map.len() as i64 && self.col < trail_map[0].len() as i64
    }
    fn up_one(&self) -> Self {
        TrailBlazer {
            row: self.row -1,
            col: self.col,
            last_height: self.last_height,
        }
    }
    fn down_one(&self) -> Self {
        TrailBlazer {
            row: self.row +1,
            col: self.col,
            last_height: self.last_height,
        }
    }
    fn right_one(&self) -> Self {
        TrailBlazer {
            row: self.row,
            col: self.col + 1,
            last_height: self.last_height,
        }
    }
    fn left_one(&self) -> Self {
        TrailBlazer {
            row: self.row,
            col: self.col - 1,
            last_height: self.last_height,
        }
    }
    fn construct_all_trailheads(all_trails: &[Vec<u32>]) -> Vec<TrailBlazer> {
        //! Build all of the starting trailheads, which are always at a height of 0.
        let mut trail_heads = Vec::new();
        for (row_number, row) in all_trails.iter().enumerate() {
            for (col_number, col) in row.iter().enumerate() {
                if *col == 0 {
                    trail_heads.push(TrailBlazer {
                        row: row_number as i64,
                        col: col_number as i64,
                        last_height: None, 
                    })
                }
            }
        }
        trail_heads
    }
}


fn get_trailhead_statistics(trail_head: TrailBlazer, trail_map: &[Vec<u32>]) -> TrailResult {
    //! Starting from the passed in `trail_head`, which is at height 0, analyze the trail to generate 
    //! a `TrailResult` using depth first search of the trail map. A successful trail goes from height 0 -> height 9,
    //! with each trail step 1 height greater than the previous step.
    let mut trail_stack = vec![trail_head];
    let mut unique_9s_reached = HashSet::new();
    let mut unique_trail_count = 0;
    while let Some(mut trail_blazer) = trail_stack.pop() {
        let current_trail_height = trail_map[trail_blazer.row as usize][trail_blazer.col as usize];
        if let Some(last_height) = trail_blazer.last_height {
            if last_height + 1 != current_trail_height {
                //Kill the path as its invalid (current height is not 1 greater than last height)
                continue;
            }
        }
        if current_trail_height == 9 {
            unique_9s_reached.insert((trail_blazer.row, trail_blazer.col));
            unique_trail_count += 1;
            continue;
        }
        trail_blazer.last_height  = Some(current_trail_height);

        let next_trail_heads = [
            trail_blazer.down_one(),
            trail_blazer.left_one(),
            trail_blazer.right_one(),
            trail_blazer.up_one()
        ];
        trail_stack.extend(next_trail_heads.into_iter().filter(|trail_blazer| trail_blazer.is_inbounds(trail_map)));
}
TrailResult {
    rank: unique_trail_count,
    score: unique_9s_reached.len()
}
}

impl SolveAdvent for Day10 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Sum all of the trailhead scores.
        let file_contents = read_input_file(path_to_file)?;
        let trail_map = file_contents.lines().map(|line| line.chars().flat_map(|char| char.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let trail_heads = TrailBlazer::construct_all_trailheads(&trail_map);
        let mut total_trailhead_score = 0;
        for trail_head in trail_heads {
            total_trailhead_score += get_trailhead_statistics(trail_head, &trail_map).score;
        }
        println!("Total trailhead score is {}", total_trailhead_score);
        Ok(())
    }
    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Exact same logic as part1 but add the ranks instead of the score.
        let file_contents = read_input_file(path_to_file)?;
        let trail_map = file_contents.lines().map(|line| line.chars().flat_map(|char| char.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let trail_heads = TrailBlazer::construct_all_trailheads(&trail_map);
        let mut total_trailhead_rank = 0;
        for trail_head in trail_heads {
            total_trailhead_rank += get_trailhead_statistics(trail_head, &trail_map).rank;
        }
        println!("Total trailhead rank is {}", total_trailhead_rank);
        Ok(())
    }
}