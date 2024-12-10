use std::collections::HashSet;

use super::{read_input_file, SolveAdvent};

pub struct Day10;


#[derive(Debug, Clone)]
struct TrailBlazer {
    row: i64,
    col: i64,
    last_number: Option<u32>,
}

impl TrailBlazer {
    fn is_inbounds(&self, trail_map: &[Vec<u32>]) -> bool {
        self.row >= 0 && self.col >= 0 && self.row < trail_map.len() as i64 && self.col < trail_map[0].len() as i64
    }
    fn clone_up(&self) -> Self {
        TrailBlazer {
            row: self.row -1,
            col: self.col,
            last_number: self.last_number,
        }
    }
    fn clone_down(&self) -> Self {
        TrailBlazer {
            row: self.row +1,
            col: self.col,
            last_number: self.last_number,
        }
    }
    fn clone_right(&self) -> Self {
        TrailBlazer {
            row: self.row,
            col: self.col + 1,
            last_number: self.last_number,
        }
    }
    fn clone_left(&self) -> Self {
        TrailBlazer {
            row: self.row,
            col: self.col - 1,
            last_number: self.last_number,
        }
    }
    fn construct_all_trailheads(all_trails: &[Vec<u32>]) -> Vec<TrailBlazer> {
        let mut trail_heads = Vec::new();
        for (row_number, row) in all_trails.iter().enumerate() {
            for (col_number, col) in row.iter().enumerate() {
                if col == &0 {
                    trail_heads.push(TrailBlazer {
                        row: row_number as i64,
                        col: col_number as i64,
                        last_number: None, 
                    })
                }
            }
        }
        trail_heads
    }
}

fn get_trailhead_score(trail_head: TrailBlazer, trail_map: &[Vec<u32>]) -> usize {
    let mut trail_stack = vec![trail_head];
    let mut unique_9s_reached = HashSet::new();
    while let Some(mut trail_blazer) = trail_stack.pop() {
        let current_trail_height = trail_map[trail_blazer.row as usize][trail_blazer.col as usize];
        if let Some(last_height) = trail_blazer.last_number {
            if last_height + 1 != current_trail_height {
                //Kill the path as its invalid
                continue;
            }
        }
        if current_trail_height == 9 {
            unique_9s_reached.insert((trail_blazer.row, trail_blazer.col));
            continue;
        }
        trail_blazer.last_number  = Some(current_trail_height);

        let next_trail_heads = [
            trail_blazer.clone_down(),
            trail_blazer.clone_left(),
            trail_blazer.clone_right(),
            trail_blazer.clone_up()
        ];
        trail_stack.extend(next_trail_heads.into_iter().filter(|trail_blazer| trail_blazer.is_inbounds(trail_map)));
}
unique_9s_reached.len()
}

impl SolveAdvent for Day10 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let trail_map = file_contents.lines().map(|line| line.chars().flat_map(|char| char.to_digit(10)).collect::<Vec<_>>()).collect::<Vec<_>>();
        let trail_heads = TrailBlazer::construct_all_trailheads(&trail_map);
        let mut total_trailhead_score = 0;
        for trail_head in trail_heads {
            total_trailhead_score += get_trailhead_score(trail_head, &trail_map);
        }
        println!("Total trailhead score is {}", total_trailhead_score);
        Ok(())
    }
    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}