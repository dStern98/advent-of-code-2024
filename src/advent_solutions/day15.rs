use std::{collections::{HashMap, HashSet}};

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
    ///The robot is kept seperate because it generates the movement
    /// of all other board objects.
    robot: BoardObject,
    //Position indexed map of all board objects NOT the robot
    other_objects: HashMap<OrderedPair, BoardObject>,
}

fn collect_boxes_2d(other_objects: & HashMap<OrderedPair, BoardObject>, current_position: OrderedPair, direction: Direction, boxes_to_move: &mut HashSet<OrderedPair>) -> Option<()> {
    //! Collect all affected boxes in the double width scenario, where a robot can potentially push boxes not directly
    //! in the direction of movement due to boxes overlapping.
    //! All collected boxes have their coordinates inserted into the `boxes_to_move` set.
    //! This solution is not perfectly efficient, as it will insert duplicates into the set (which are of course ignored)

    //Climb the ladder (either up or down) as long as necessary to collect all boxes to move. 
    let mut ladder = vec![direction.move_one(current_position)];
    while let Some(object_position) = ladder.pop() {
        let (current_row, current_col) = object_position;
        if let Some(board_object) = other_objects.get(&object_position) {
            if board_object.symbol == ']' {
                //If a right bracket, then we need to include the matching left bracket, which is 1 to the left
                let adjacent_box = (current_row, current_col - 1);
                boxes_to_move.extend([adjacent_box, object_position]);
                //Add to the ladder
                ladder.extend([direction.move_one(adjacent_box), direction.move_one(object_position)]);
            } else if board_object.symbol == '[' {
                //If we have a left bracket, then we automatically add the right bracket
                let adjacent_box = (current_row, current_col + 1);
                boxes_to_move.extend([adjacent_box, object_position]);
                ladder.extend([direction.move_one(adjacent_box), direction.move_one(object_position)]);
            } else if board_object.symbol == '#' {
                //If we hit a wall while iterating, then we return `None`, as the robot can't move anything
                return None;
            }
        }
        }
        //If we reach this point, then we know at least the robot can be moved
        Some(())
    }


fn collect_boxes_1d(other_objects: & HashMap<OrderedPair, BoardObject>, mut current_position: OrderedPair, direction: Direction, boxes_to_move: &mut HashSet<OrderedPair>) -> Option<()> {
    //In the 1d case, we just collect all boxes as we move directly in the `direction`
    loop {
        current_position = direction.move_one(current_position);
        match other_objects.get(&current_position) {
            Some(board_object) => {
                if let ObjectType::Box = board_object.object_type {
                    //Boxes are 'collected' to be moved
                    boxes_to_move.insert(board_object.position);
                } else {
                    //If we encounter a wall during iteration, then its impossible
                    //to move anyone, so return
                    return None;
                }
            }, 
            None => {
                //We reached an empty space, which means we are done collecting objects
                return Some(());
            }
        }
    }
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
        board[self.robot.position.0 as usize][self.robot.position.1 as usize] = self.robot.symbol;
        for other_object in self.other_objects.values() {
            board[other_object.position.0 as usize][other_object.position.1 as usize] = other_object.symbol;
        }

        for board_row in board {
            println!("{:?}", board_row.into_iter().collect::<String>());
        }
    }

 
    fn try_move(&mut self, direction: Direction, double_width: bool) {
        //! Try to move the robot, and any boxes that are movable
        //! If `double_width` is True, then for `Up`| `Down` motion, the `2d` collection version
        //! is used.
        
        //Step 1: Collect all boxes that can be safely moved. By convention, if nothing can be moved (including the robot),
        //then None is returned.
        let mut boxes_to_move = HashSet::new();
        let current_position = self.robot.position;
        let box_collection_outcome = match direction {
            //Only up/down motion needs to use the 2d method, because the boxes are 2x wide but never 2x tall
            Direction::Down | Direction::Up if double_width => collect_boxes_2d(&self.other_objects, current_position, direction, &mut boxes_to_move),
            _=> collect_boxes_1d(&self.other_objects, current_position, direction, &mut boxes_to_move),
        };
        if box_collection_outcome.is_none() {
            //None returned means we can't move anything so we are done!
            return;
        }
        //2. Move the robot and all objects in the `boxes_to_move` set 1 step in the `direction`
        //Reaching this point means we can at the very least move the robot.
        self.robot.position = direction.move_one(self.robot.position);
        //3. In addition, we can move any boxes that were collected
        let mut boxes_after_move = Vec::new();
        for box_to_move in boxes_to_move {
            if let Some(mut removed_box) = self.other_objects.remove(&box_to_move) {
                //Move the associated box
                removed_box.position = direction.move_one(removed_box.position);
                boxes_after_move.push(removed_box);
            }
        };
        //Put the new boxes back in the `other_objects` map. 
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

fn parse_input_file(file_contents: &str, double_width: bool) -> anyhow::Result<(WarehouseRuntime, Vec<Direction>)> {
    //! Parse the input file into a list of directions for the robot, and the positions
    //! of all objects in the map.
    let demarcation_point = file_contents.find("\r\n\r\n").ok_or(anyhow!("Input file did not contain an empty line seperating map from directions"))?;
    let map_input = &file_contents[0..demarcation_point];
    let directions_input = &file_contents[demarcation_point..];
    let directions = Direction::from_input_file(directions_input.trim())?;
    let mut board_objects = BoardObject::from_input_file(map_input)?;
    if double_width {
        board_objects = BoardObject::build_double_width_objects(&mut board_objects);
    }
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
    Wall, 
}

///Represents a single object in the map at position `position`
/// and type `object_type`
/// For Part2, we use the `symbol` as a defacto 
/// second enum `ObjectType`. But its preferable to not change the `ObjectType`
/// enum directly, as that will break the part1 solution.
#[derive(Debug, Clone, Copy)]
struct BoardObject {
    position: OrderedPair,
    object_type: ObjectType, 
    ///The char representing the object.
    /// This is key in part2, where `[` represents
    /// a left box, and `]` represents a right box
    symbol: char
}

impl BoardObject {
    fn gps_coordinate_double_width(&self) -> Option<i64> {
         //! GPS coordinate is only defined for a box, 
        //! and is 100xrow + col
        if self.symbol == '[' {
            let (row, col) = self.position;
            return Some(100*row + col);
        }
        None
    }
    fn gps_coordinate(&self) -> Option<i64> {
        //! GPS coordinate is only defined for a box, 
        //! and is 100xrow + col
        if let ObjectType::Box = self.object_type {
            let (row, col) = self.position;
            return Some(100*row + col);
        }
        None
    }

    fn build_double_width_objects(objects: &mut Vec<Self>) -> Vec<Self> {
        //!Map all of the original objects into the new double width warehouse:
        //! In the double width variants, the `symbol` becomes key to represent the left `[`
        //! and right `]` side of the box
        let mut new_objects = Vec::new();
        while let Some(old_object) = objects.pop() {
            let (old_row, old_col) = old_object.position;
            //Note that now col numbers are always doubled
            match old_object.object_type {
                ObjectType::Box => {
                    //A Box now becomes a double width box (`[]`)
                    new_objects.extend(
                        [
                            BoardObject {
                                position: (old_row, old_col * 2),
                                object_type: ObjectType::Box,
                                symbol: '['
                            },
                            BoardObject {
                                position: (old_row, old_col * 2 + 1),
                                object_type: ObjectType::Box,
                                symbol: ']'
                            }
                        ]
                    );
                },
                ObjectType::Robot => {
                    //Robot is a robot plus an empty space, which we do not create an
                    //object for
                    new_objects.push(
                            BoardObject {
                                position: (old_row, old_col * 2),
                                object_type: ObjectType::Robot,
                                symbol: '@'
                            }
                        
                    );
                },
                ObjectType::Wall => {
                    //Wall simply becomes two walls
                    new_objects.extend(
                        [
                            BoardObject {
                                position: (old_row, old_col * 2),
                                object_type: ObjectType::Wall,
                                symbol: '#'
                            },
                            BoardObject {
                                position: (old_row, old_col * 2 + 1),
                                object_type: ObjectType::Wall,
                                symbol: '#'
                            }
                        ]
                    );
                },
            }
        }
        new_objects
    }

    fn from_input_file(file_contents: &str) -> anyhow::Result<Vec<Self>> {
        //! Parse the first half of the input file into the map
        let mut board_objects = Vec::new();
        for (line_number, line) in file_contents.lines().enumerate() {
            for (col_number, symbol) in line.chars().enumerate() {
                let row = line_number as i64;
                let col = col_number as i64;
                let object_type = match symbol {
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
                    object_type, 
                    symbol
                })
            }
        }
        
    Ok(board_objects)
    }
}

impl SolveAdvent for Day15 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        //! Apply all of the directions, moving any eligible boxes
        let file_contents = read_input_file(path_to_file)?;
        let (mut warehouse_runtime, directions) = parse_input_file(&file_contents, false)?;
        for direction in directions {
            warehouse_runtime.try_move(direction, false);
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
    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Apply all of the directions, moving any eligible boxes.
        let file_contents = read_input_file(path_to_file)?;
        let (mut warehouse_runtime, directions) = parse_input_file(&file_contents, true)?;
        for direction in directions {
            warehouse_runtime.try_move(direction, true);
        }
        let mut total_gps_score = 0;
        for object in warehouse_runtime.other_objects.values() {
            if let Some(gps_coordinate) = object.gps_coordinate_double_width() {
                total_gps_score += gps_coordinate;
            }
        }
        println!("Total GPS Coordinates with double width boxes is {}", total_gps_score);
        Ok(())
    }
}