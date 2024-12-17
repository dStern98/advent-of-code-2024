use std::collections::HashMap;

use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day15;

type OrderedPair = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right, 
    Down,
    Up
}

///Stores the robot seperate
/// from the rest of the objects in the world
/// Because each object can only be in one place at a time, we
/// can index objects by their current position in the `other_objects` map
struct WarehouseRuntime {
    robot: BoardObject,
    other_objects: HashMap<OrderedPair, BoardObject>,
}

impl WarehouseRuntime {
    #[allow(dead_code)]
    fn draw_board(&self) {
        //! Draw the current board for debugging purposes.
        let max_row = self.other_objects.keys().max_by_key(|position| position.0).unwrap().0;
        let max_col = self.other_objects.keys().max_by_key(|position| position.1).unwrap().1;
        let max_row = max_row.max(self.robot.position.0) as usize;
        let max_col = max_col.max(self.robot.position.1) as usize;
        let mut board = (0..=max_row).map(|_| (0..=max_col).map(|_| '.').collect::<Vec<_>>()).collect::<Vec<_>>();
        board[self.robot.position.0 as usize][self.robot.position.1 as usize] = '@';
        for other_object in self.other_objects.values() {
            let symbol = match other_object.object_type {
                ObjectType::Wall => '#',
                ObjectType::Box => 'O',
                _ => {
                    continue;
                }
            };
            board[other_object.position.0 as usize][other_object.position.1 as usize] = symbol;
        }

        for board_row in board {
            println!("{:?}", board_row.into_iter().collect::<String>());
        }
    }
    fn try_move(&mut self, direction: Direction) {
        //! Try to move the robot, and any boxes that are movable
        let mut boxes_to_move = vec![];
        let mut current_position = self.robot.position;
        loop {
            current_position = direction.move_one(current_position);
            match self.other_objects.get(&current_position) {
                Some(board_object) => {
                    if let ObjectType::Box = board_object.object_type {
                        //Boxes are 'collected' to be moved
                        boxes_to_move.push(board_object.position);
                    } else {
                        //If we encounter a wall during iteration, then its impossible
                        //to move anyone, so return
                        return;
                    }
                }, 
                None => {
                    //We reached an empty space, which means we are done collecting objects
                    break;
                }
            }
        }
        //Reaching this point means we can at the very least move the robot.
        self.robot.position = direction.move_one(self.robot.position);
        //In addition, we can move any boxes that were collected
        let mut boxes_after_move = Vec::new();
        for box_to_move in boxes_to_move {
            let mut removed_box = self.other_objects.remove(&box_to_move).unwrap();
            //Move the associated box
            removed_box.position = direction.move_one(removed_box.position);
            boxes_after_move.push(removed_box);
        };
        //Put the new boxes back in the `other_objects` map
        for moved_box in boxes_after_move {
            self.other_objects.insert(moved_box.position, moved_box);
        };
    }

    fn try_construct(board_objects: Vec<BoardObject>) -> anyhow::Result<Self> {
        //! Construct a `WarehouseRuntime`
        let mut robot = None;
        let mut other_objects = HashMap::new();
        for board_object in board_objects {
            if let ObjectType::Robot = board_object.object_type {
                robot = Some(board_object);
            } else {
                other_objects.insert(board_object.position, board_object);
            }
        }
        let robot = robot.ok_or(anyhow!("No robot was encountered when iterating the board objects"))?;
        Ok(WarehouseRuntime {
            robot,
            other_objects
        })

    }
}

fn parse_input_file(file_contents: &str) -> anyhow::Result<(WarehouseRuntime, Vec<Direction>)> {
    //! Parse the input file into a list of directions for the robot, and the positions
    //! of all objects in the map.
    let demarcation_point = file_contents.find("\r\n\r\n").unwrap();
    let map_input = &file_contents[0..demarcation_point];
    let directions_input = &file_contents[demarcation_point..];
    let directions = Direction::from_input_file(directions_input.trim())?;
    let board_objects = BoardObject::from_input_file(map_input)?;
    let warehouse_runtime = WarehouseRuntime::try_construct(board_objects)?;
    Ok((warehouse_runtime, directions))
}

impl Direction {
    fn from_input_file(file_contents: &str) -> anyhow::Result<Vec<Direction>> {
        //! Takes the second half of the input file, which is the list of directions, 
        //! and parses them into `Direction`.
        let mut directions = Vec::new();
        for line in file_contents.lines() {
            for direction in line.chars() {
                match direction {
                    '<' => directions.push(Direction::Left),
                    '^' => directions.push(Direction::Up),
                    '>' => directions.push(Direction::Right),
                    'v' => directions.push(Direction::Down),
                    other => anyhow::bail!("Encountered illegal direction {}", other)
                }
            }
        }
    Ok(directions)
    }
    fn move_one(&self, current_position: OrderedPair) -> OrderedPair {
        //! Get the new ordered_pair position by moving 1-step
        //! in the `self` direction.
        let (row, col) = current_position;
        match self {
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col -1),
            Direction::Up => (row - 1, col),
            Direction::Right => (row, col + 1)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ObjectType {
    ///The object is the robot. There should only
    /// be one in the map
    Robot, 
    ///The object is a box, which can be moved
    Box, 
    ///The object is a wall, which cannot be moved.
    Wall
}

///Represents a single object in the map at position `position`
/// and type `object_type`
#[derive(Debug, Clone, Copy)]
struct BoardObject {
    position: OrderedPair,
    object_type: ObjectType
}

impl BoardObject {
    fn gps_coordinate(&self) -> Option<i64> {
        //! GPS coordinate is only defined for a box, 
        //! and is 100xrow + col
        if let ObjectType::Box = self.object_type {
            let (row, col) = self.position;
            return Some(100*row + col);
        }
        None
    }
    fn from_input_file(file_contents: &str) -> anyhow::Result<Vec<Self>> {
        //! Parse the first half of the input file into the map
        let mut board_objects = Vec::new();
        for (line_number, line) in file_contents.lines().enumerate() {
            for (col_number, char) in line.chars().enumerate() {
                let row = line_number as i64;
                let col = col_number as i64;
                let object_type = match char {
                    '#' => ObjectType::Wall,
                    'O' => ObjectType::Box,
                    '@' => ObjectType::Robot,
                    '.' => {
                        continue;
                    }
                    other => anyhow::bail!("Encountered illegal map character {}", other)
                };
                board_objects.push(BoardObject {
                    position: (row, col),
                    object_type
                })
            }
        }
        
    Ok(board_objects)
    }
}

impl SolveAdvent for Day15 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let (mut warehouse_runtime, directions) = parse_input_file(&file_contents)?;
        for direction in directions {
            warehouse_runtime.try_move(direction);
        }
        let mut total_gps_score = 0;
        for object in warehouse_runtime.other_objects.values() {
            if let Some(gps_coordinate) = object.gps_coordinate() {
                total_gps_score += gps_coordinate;
            }
        }
        println!("Total GPS Coordinate is {}", total_gps_score);
        Ok(())
    }
    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}