use std::collections::HashSet;

use super::{read_input_file, SolveAdvent};


pub struct  Day6;

#[derive(Debug, Clone)]
struct Guard {
    ///Guards current row position
    row: i64,
    ///Guards current column position
    col: i64,
    ///Guards current direction
    direction: Direction,
}

impl Guard {
    fn patrol_lab(&mut self, lab_map: &[Vec<char>]) -> anyhow::Result<()> {
        //! Move the guard by 1 step. 
        //! If possible the guard takes 1 step in the same direction they are facing.
        //! Otherwise, rotate 90 degrees until motion is possible. 
        //! Returns an error if the guard walks off the lab map (which is actually the goal)
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
        //! Construct a starting guard from the input lab map.
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
                        })
                    }, 
                    '>' => {
                        return Ok(Guard { 
                            row: row_number, 
                            col: col_number, 
                            direction: Direction::Right})
                    },
                    'v' => {
                        return Ok(Guard{
                            row: row_number, 
                            col: col_number, 
                            direction: Direction::Down})
                    },
                    '<' => {
                        return Ok(Guard { 
                            row: row_number, 
                            col: col_number, 
                            direction: Direction::Left })
                    }
                    _ => {}
                }
            }
        }
        anyhow::bail!("Lab map did not contain a starting guard position!");
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
        //! Let the guard patrol the lab until they walk off the map.
        //! Count the unique spaces traversed.
        let file_contents = read_input_file(path_to_file)?;
        let lab_map = file_contents.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut guard = Guard::try_new(&lab_map)?;
        let mut visit_history = HashSet::new();
        visit_history.insert((guard.row, guard.col));
        //When the guard patrol_lab call returns an error, the guard
        // has walked off the map.
        while guard.patrol_lab(&lab_map).is_ok() {
            //Keep track of unique visits
            visit_history.insert((guard.row, guard.col));
        }
        println!("Guard visited {:?} unique positions in the lab", visit_history.len());
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! A brute-force simulation to find all single obstacles that can
        //! be placed to induce the guard to get stuck in an infinite loop. Not sure
        //! if there is a better way to do this.
        let file_contents = read_input_file(path_to_file)?;
        let mut lab_map = file_contents.lines().map(|row| row.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let guard = Guard::try_new(&lab_map)?;
        let mut successful_guard_loops = 0;
        for row_number in 0..lab_map.len() {
            for col_number in 0..lab_map[0].len() {
                if lab_map[row_number][col_number] == '#' || lab_map[row_number][col_number] == '^' {
                    //We are not allowed to place a new obstacle if there already is an obstacle
                    //or this is the guards starting position
                    continue;
                }
                //Temporarily change row,col to contain an obstacle.
                lab_map[row_number][col_number] = '#';
                let mut guard = guard.clone();
                //An infinite loop occurs if the same row, col and direction are ever repeated.
                //Because the guards progression are deterministic, this gurantees a infinite loop has been reached
                let mut guard_loop_detector = HashSet::new();
                guard_loop_detector.insert((guard.row, guard.col, guard.direction));
                while guard.patrol_lab(&lab_map).is_ok() {
                    if guard_loop_detector.contains(&(guard.row, guard.col, guard.direction)) {
                        successful_guard_loops += 1;
                        break;
                    }
                    guard_loop_detector.insert((guard.row, guard.col, guard.direction));
                }
                //After each simulation we put the current spot back to being empty. 
                lab_map[row_number][col_number] = '.';
            }
        }
        println!("There were {} unique obstacles that generate infinite guard loops", successful_guard_loops);
        Ok(())
    }
}

