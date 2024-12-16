use std::collections::HashMap;

use anyhow::{anyhow, Context};

use super::{read_input_file, SolveAdvent};

pub struct Day14;

type OrderedPair = (i64, i64);

///Represents a Robot guarding the bathroom
#[derive(Debug, Clone, Copy)]
struct Robot {
    position: OrderedPair,
    velocity: OrderedPair
}

///The position on the map of a given robot
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Quadrant {
    UpperLeft,
    UpperRight,
    LowerLeft,
    LowerRight,
}

impl Quadrant {
    fn assign(robot: &Robot, board_dimensions: OrderedPair) -> Option<Self> {
        //! Try to assign a robot to a quadrant. Any robot in the middle row or column
        //! is not in a quadrant at all according to the problem.
        let middle_row = board_dimensions.0 /2;
        let middle_column = board_dimensions.1 / 2;
        if robot.position.0 < middle_row && robot.position.1 < middle_column {
            return Some(Quadrant::UpperLeft);
        } 
        else if robot.position.0 < middle_row && robot.position.1 > middle_column {
            return Some(Quadrant::UpperRight);
        }
        else if robot.position.0 > middle_row && robot.position.1 < middle_column {
            return Some(Quadrant::LowerLeft);
        }
        else if robot.position.0 > middle_row && robot.position.1 > middle_column {
            return Some(Quadrant::LowerRight);
        }
        None
    }
}


impl Robot {
    fn try_new(line: &str) -> anyhow::Result<Robot> {
        //! Parse the input line into a Robot's initial velocity and position. 
        let [positions, velocities]: [&str; 2] = line.trim().split(' ').collect::<Vec<_>>().try_into().map_err(|_| anyhow!("Could not coerce line into 2 parts"))?;
        let positions: Result<Vec<_>, _> = positions.replace("p=", "").split(',').map(|component| component.parse::<i64>()).collect();
        let positions = positions.context("Failed to parse positions into integers")?;
        let velocities: Result<Vec<_>, _> = velocities.replace("v=", "").split(',').map(|component| component.parse::<i64>() ).collect();
        let velocities = velocities.context("Failed to parse velocities into integers")?;
        anyhow::ensure!(velocities.len() == 2, anyhow!("Velocities vec must be length 2!"));
        anyhow::ensure!(positions.len() == 2, anyhow!("Positions vec must be length 2!"));
        Ok(Robot {
            position:  (positions[1], positions[0]), //Order is swapped from problem input because our convention is row, col not the other way around!
            velocity: (velocities[1], velocities[0])
        })

    }

    fn tick(&mut self, board_dimensions: OrderedPair) {
        //! Advance the robot by a second. Much like Pacman, a robot teleports to the other
        //! side of the board when it runs off the map.
        self.position = ((self.position.0 + self.velocity.0) % board_dimensions.0, (self.position.1 + self.velocity.1) % board_dimensions.1);
        if self.position.0 < 0 {
            self.position.0 += board_dimensions.0
        }
        if self.position.1 < 0 {
            self.position.1 += board_dimensions.1;
        }

    }   
}


impl SolveAdvent for Day14 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let board_dimensions = (103, 101);
        let file_contents = read_input_file(path_to_file)?;
        let mut robots = file_contents.lines().map(Robot::try_new).collect::<Result<Vec<_>, _>>()?;
        //Move the robots for 100 seconds
        for _ in 0..100 {
            for robot in robots.iter_mut() {
                robot.tick(board_dimensions);
            }
        }
        //Divide into quadrants and count how many are in each quadrant
        let mut quadrant_map: HashMap<Quadrant, usize> = HashMap::new();
        for robot in robots {
            if let Some(quadrant) = Quadrant::assign(&robot, board_dimensions) {
                *quadrant_map.entry(quadrant).or_default() += 1;
            }
        }
        let safety_factor = quadrant_map.values().product::<usize>();
        println!("Safety factor on board of dimension {}x{} is {}", board_dimensions.0, board_dimensions.1, safety_factor);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}