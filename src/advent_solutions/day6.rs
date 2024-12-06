use std::collections::HashSet;

use super::{read_input_file, SolveAdvent};


pub struct  Day6;

#[derive(Debug, Clone)]
struct Guard {
    row: i64,
    col: i64,
    direction: Direction,
    visit_history: HashSet<(i64, i64)>
}

impl Guard {
    fn patrol_lab(&mut self, lab_map: &[Vec<char>]) -> anyhow::Result<()> {
        self.visit_history.insert((self.row, self.col));
        loop {
            let (next_row, next_col) = match self.direction {
                Direction::Right => ( self.row, self.col + 1),
                Direction::Down => (self.row + 1, self.col),
                Direction::Left => (self.row, self.col - 1),
                Direction::Up => (self.row - 1, self.col)
            };
            if next_row < 0 || next_col < 0 || next_row  >= lab_map.len() as i64 || next_col  >= lab_map[0].len() as i64 {
                anyhow::bail!("Guard has walked off the map!");
            }
            let next_space_character = lab_map[next_row as usize][next_col as usize];
            if next_space_character == '.' || next_space_character == '^' {
                //The next space is valid, so we step and return.
                self.col = next_col;
                self.row = next_row;
                return Ok(());
            } else {
                //If we are facing an obstacle, we rotate 90 degrees and try again
                self.direction.right_90_degrees();
            }
        }
    }
    fn try_new(lab_map: &[Vec<char>]) -> anyhow::Result<Self> {
        for (row_number, row) in lab_map.iter().enumerate() {
            for (col_number, col) in row.iter().enumerate() {
                let row_number = row_number as i64;
                let col_number = col_number as i64;
                match col {
                    '^' => {
                        return Ok(Guard {
                            row: row_number, 
                            col: col_number, 
                            direction: Direction::Up,
                            visit_history: HashSet::new()
                        })
                    }, 
                    '>' => {
                        return Ok(Guard { row: row_number, col: col_number, direction: Direction::Right, visit_history: HashSet::new() })
                    },
                    'v' => {
                        return Ok(Guard{row: row_number, col: col_number, direction: Direction::Down, visit_history: HashSet::new()})
                    },
                    '<' => {
                        return Ok(Guard { row: row_number, col: col_number, direction: Direction::Left, visit_history: HashSet::new() })
                    }
                    _ => {}
                }
            }
        }
        anyhow::bail!("Lab map did not contain a starting guard position!");
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up, 
    Down, 
    Left,
    Right
}

impl Direction {
    fn right_90_degrees(&mut self) {
        match self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up
        }
    }
}

impl SolveAdvent for Day6 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let lab_map = file_contents.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut guard = Guard::try_new(&lab_map)?;
        while let Ok(_) = guard.patrol_lab(&lab_map) {}
        println!("Guard visited {:?} unique positions in the lab", guard.visit_history.len());
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}

