use std::collections::{HashMap, HashSet, VecDeque};

use super::{read_input_file, SolveAdvent};

pub struct Day16;

type OrderedPair = (i64, i64);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Left,
    Right, 
    Down
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Down =>Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up =>Direction::Right,
            Direction::Right =>  Direction::Down
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            Direction::Down => Direction::Right,
            Direction::Left =>  Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up
        }
    }

    fn move_one(& self, current_positon: OrderedPair) -> OrderedPair {
        let (row, col) = current_positon;
        match self {
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1)
        }
    }
}

#[derive(Debug, Clone)]
struct MazeRunner<'a> {
    position: OrderedPair,
    direction: Direction,
    visited: HashSet<(OrderedPair, Direction)>, 
    maze: &'a Vec<Vec<char>>,
    running_score: usize
}

impl <'a > std::fmt::Display for MazeRunner<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MazeRunner(position={:?}, direction={:?}, running_score: {})", self.position, self.direction, self.running_score)
    }
}

impl <'a>MazeRunner <'a> {
    fn find_start_position(maze: &Vec<Vec<char>>) -> anyhow::Result<OrderedPair> {
        for (row_number, row) in maze.iter().enumerate() {
            for (col_number, symbol) in row.iter().enumerate() {
                if *symbol == 'S' {
                    return Ok((row_number as i64, col_number as i64));
                }
            }
        }
        anyhow::bail!("No S is present in the maze to start from");

    }
    fn try_new(maze: &'a Vec<Vec<char>>) -> anyhow::Result<MazeRunner<'a>> {
        let start_position = MazeRunner::find_start_position(maze)?;
        Ok(MazeRunner {
            position: start_position,
            direction: Direction::Right,
            visited: HashSet::new(),
            maze,
            running_score: 0
        })
    }

    fn in_cycle(&self) -> bool {
        self.visited.contains(&(self.position, self.direction))
    }

    fn spawn_next(&self) -> [Self; 3] {
        [
            MazeRunner {
                position: self.direction.move_one(self.position),
                direction: self.direction,
                maze: &self.maze,
                visited: self.visited.clone(), 
                running_score: self.running_score + 1
            }, 
            MazeRunner {
                position: self.position,
                direction: self.direction.rotate_clockwise(),
                maze: &self.maze,
                visited: self.visited.clone(),
                running_score: self.running_score + 1_000
            }, 
            MazeRunner {
                position: self.position,
                direction: self.direction.rotate_counterclockwise(),
                maze: &self.maze,
                visited: self.visited.clone(),
                running_score: self.running_score + 1_000
            }, 
        ]
    }


    fn visit(&mut self) {
        self.visited.insert((self.position, self.direction));
    }
    fn destination_reached(&self) -> bool {
        if !self.is_valid_space() {
            return false;
        }
        let (row, col) = self.position;
        if self.maze[row as usize][col as usize] == 'E' {
            return true;
        }
        false
    }
    fn is_valid_space(&self) -> bool {
        let (row, col)  = self.position;
        if row < 0 || col < 0 || row >= self.maze.len() as i64 || col >= self.maze[0].len() as i64 {
            return false;
        }
        if self.maze[row as usize][col as usize] == '#' {
            return false;
        }
        true
    }
}

impl SolveAdvent for Day16 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents =read_input_file(path_to_file)?;
        let maze = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut traversal_queue = VecDeque::new();
        traversal_queue.push_back(MazeRunner::try_new(&maze)?);
        let mut lowest_score = usize::MAX;
        let mut optimizer = HashMap::new();
        while let Some(mut current_maze_runner) = traversal_queue.pop_front() {
            if !current_maze_runner.is_valid_space() || current_maze_runner.in_cycle() {
                continue;
            }
            if current_maze_runner.destination_reached() {
                lowest_score = lowest_score.min(current_maze_runner.running_score);
                continue;
            }
            if let Some(best_score) = optimizer.get_mut(&(current_maze_runner.position, current_maze_runner.direction)) {
                if current_maze_runner.running_score > *best_score {
                    continue;
                } else {
                    *best_score = current_maze_runner.running_score;
                }

            } else {
                optimizer.insert((current_maze_runner.position, current_maze_runner.direction), current_maze_runner.running_score); 
            }
            current_maze_runner.visit();
            traversal_queue.extend(current_maze_runner.spawn_next());
        }
        println!("Minimum score traversing from S to E is {}", lowest_score);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents =read_input_file(path_to_file)?;
        let maze = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut traversal_queue = VecDeque::new();
        traversal_queue.push_back(MazeRunner::try_new(&maze)?);
        let mut lowest_seen_score = usize::MAX;
        let mut optimal_path_position: HashSet<OrderedPair> = HashSet::new();
        let mut optimizer = HashMap::new();
        while let Some(mut current_maze_runner) = traversal_queue.pop_front() {
            if !current_maze_runner.is_valid_space() || current_maze_runner.in_cycle() {
                continue;
            }
            current_maze_runner.visit();
            if current_maze_runner.destination_reached() {
                if current_maze_runner.running_score < lowest_seen_score {
                    lowest_seen_score = current_maze_runner.running_score;
                    optimal_path_position.clear();
                    optimal_path_position.extend(current_maze_runner.visited.iter().map(|(position, _direction)| position));
                } else if current_maze_runner.running_score == lowest_seen_score {
                    optimal_path_position.extend(current_maze_runner.visited.iter().map(|(position, _direction)| position));
                }
                continue;
            }
            if let Some(best_score) = optimizer.get_mut(&(current_maze_runner.position, current_maze_runner.direction)) {
                if current_maze_runner.running_score > *best_score {
                    continue;
                } else {
                    *best_score = current_maze_runner.running_score;
                }

            } else {
                optimizer.insert((current_maze_runner.position, current_maze_runner.direction), current_maze_runner.running_score); 
            }
            traversal_queue.extend(current_maze_runner.spawn_next());
        }
        println!("Unique positions on an optimal path through the maze: {}", optimal_path_position.len());
        Ok(())
    }
}

