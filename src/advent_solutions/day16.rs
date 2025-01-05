use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}};

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
        //! Move 1-step in the direction specified
        let (row, col) = current_positon;
        match self {
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1)
        }
    }
}

///An object exploring the maze
#[derive(Debug, Clone)]
struct MazeRunner<'a> {
    ///Current maze position
    position: OrderedPair,
    ///Current traversal direction
    direction: Direction,
    ///This probe's traversal history
    visited: HashSet<(OrderedPair, Direction)>, 
    ///The maze being traversed
    maze: &'a Vec<Vec<char>>,
    ///Running score along the path. Each turn costs 1000 points,
    /// each step straight costs 1 point
    running_score: usize
}

impl std::fmt::Display for MazeRunner<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MazeRunner(position={:?}, direction={:?}, running_score: {})", self.position, self.direction, self.running_score)
    }
}

impl <'a>MazeRunner <'a> {
    fn find_start_position(maze: &[Vec<char>]) -> anyhow::Result<OrderedPair> {
        //! Find the start position (the first maze runner)
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
        //! Spawn the next 3 possible probes from the current maze runner
        [
            //Option 1: Move 1 step in the same direction, score increments by 1
            MazeRunner {
                position: self.direction.move_one(self.position),
                direction: self.direction,
                maze: self.maze,
                visited: self.visited.clone(), 
                running_score: self.running_score + 1
            }, 
            //Option 2: Rotate Clockwise, which costs 1_000 points
            MazeRunner {
                position: self.position,
                direction: self.direction.rotate_clockwise(),
                maze: self.maze,
                visited: self.visited.clone(),
                running_score: self.running_score + 1_000
            }, 
            //Option 3: Rotate Counterclockwise, which costs 1_000 points
            MazeRunner {
                position: self.position,
                direction: self.direction.rotate_counterclockwise(),
                maze: self.maze,
                visited: self.visited.clone(),
                running_score: self.running_score + 1_000
            }, 
        ]
    }


    fn visit(&mut self) {
        //! Add current position to visit history
        self.visited.insert((self.position, self.direction));
    }
    fn destination_reached(&self) -> bool {
        //! Has the `E` space been reached?
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
        //! Is the `current_position` a valid space (inbounds and not a wall (`#`))
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

///Abstraction of an object that kills probes early as soon as it can 
/// be determined that the probe in question is not optimal
struct Optimizer {
    best_score_tracker: HashMap<(OrderedPair, Direction), usize>,
}

impl Optimizer {
    fn new() -> Self {
        Optimizer {
            best_score_tracker: HashMap::new()
        }
    }

    fn kill_probe(&mut self, current_maze_runner: &MazeRunner) -> bool {
        //! Kill the probe if the `current_maze_runner` cannot possibly be on an optimal path.
        if let Some(previous_best_score) =self.best_score_tracker.get_mut(&(current_maze_runner.position, current_maze_runner.direction)) {
            if current_maze_runner.running_score > *previous_best_score {
                //A previous probe got here first and had a better score, so the current probe should be killed
                return true;
            } else {
                //Update to the new score
                *previous_best_score = current_maze_runner.running_score;
            }

        } else {
            self.best_score_tracker.insert((current_maze_runner.position, current_maze_runner.direction), current_maze_runner.running_score); 
        }
        false
    }
}

impl SolveAdvent for Day16 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Breadth first search of the maze traveling all possible paths from S to E.
        //! The lowest scored path is tracked. A black-box `optimizer` tries to kill probes
        //! as soon as possible to clamp down the programs runtime.
        //! 
        //! The current optimizer can be improved. Current program runtime is round 50 seconds, which 
        //! is obviously not desirable, and can be improved with a better optimizer
        let file_contents =read_input_file(path_to_file)?;
        let maze = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut traversal_queue = VecDeque::new();
        traversal_queue.push_back(MazeRunner::try_new(&maze)?);
        let mut lowest_score = usize::MAX;
        let mut optimizer = Optimizer::new();
        while let Some(mut current_maze_runner) = traversal_queue.pop_front() {
            if !current_maze_runner.is_valid_space() || current_maze_runner.in_cycle() {
                continue;
            }
            if current_maze_runner.destination_reached() {
                lowest_score = lowest_score.min(current_maze_runner.running_score);
                continue;
            }
            if optimizer.kill_probe(&current_maze_runner) {
                continue;
            }
            current_maze_runner.visit();
            traversal_queue.extend(current_maze_runner.spawn_next());
        }
        println!("Minimum score traversing from S to E is {}", lowest_score);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Essentially the exact same algorithm as the solution to part1, but
        //! each unique positions that are on one of the optimial paths is tracked.
        //! Runtime is around 50 seconds, which can be improved.
        let file_contents =read_input_file(path_to_file)?;
        let maze = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut traversal_queue = VecDeque::new();
        traversal_queue.push_back(MazeRunner::try_new(&maze)?);
        let mut lowest_seen_score = usize::MAX;
        //Set to keep track of all unique positions that are part of an optimal path through the maze
        let mut positions_on_optimal_path: HashSet<OrderedPair> = HashSet::new();
        let mut optimizer = Optimizer::new();
        while let Some(mut current_maze_runner) = traversal_queue.pop_front() {
            if !current_maze_runner.is_valid_space() || current_maze_runner.in_cycle() {
                continue;
            }
            current_maze_runner.visit();
            if current_maze_runner.destination_reached() {
                match current_maze_runner.running_score.cmp(&lowest_seen_score) {
                    Ordering::Less => {
                        lowest_seen_score = current_maze_runner.running_score;
                    positions_on_optimal_path.clear();
                    positions_on_optimal_path.extend(current_maze_runner.visited.iter().map(|(position, _direction)| position));
                    },
                    Ordering::Equal => {
                        positions_on_optimal_path.extend(current_maze_runner.visited.iter().map(|(position, _direction)| position));
                    },
                    Ordering::Greater => {}
                }
                continue;
            }
            if optimizer.kill_probe(&current_maze_runner) {
                continue;
            }
            traversal_queue.extend(current_maze_runner.spawn_next());
        }
        println!("Unique positions on an optimal path through the maze: {}", positions_on_optimal_path.len());
        Ok(())
    }
}

