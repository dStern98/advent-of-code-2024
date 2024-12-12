use std::collections::HashSet;

use anyhow::anyhow;

use super::{read_input_file, SolveAdvent};

pub struct Day12;

type OrderedPair = (i64, i64);

#[derive(Debug, Clone)]
struct GardenRegionStatistics {
    perimeter: usize,
    area: usize,
    region_elements: HashSet<(i64, i64)>
}

enum Direction {
    Up,
    Down, 
    Right, 
    Left
}

struct PerimeterFenceMarker {
    inbounds: OrderedPair,
    outbounds: OrderedPair,
    jump_direction: Direction
}

impl PerimeterFenceMarker {
    fn try_new(inbounds: OrderedPair, outbounds: OrderedPair) -> anyhow::Result<Self> {
        let jump_direction  =  Direction::from_ordered_pairs(inbounds, outbounds)?;
        Ok(PerimeterFenceMarker {
            inbounds,
            outbounds,
            jump_direction
        }
    )
    }
}

impl Direction {
    fn from_ordered_pairs(pair1: OrderedPair, pair2: OrderedPair) -> anyhow::Result<Self> {
        //! Given two ordered pairs which represent a vector going from `pair1` -> `pair2`, 
        //! mark the cardinal direction of the flow
        let (row1, col1) = pair1;
        let (row2, col2) = pair2;
        if col1== col2 {
            if row1 < row2 {
                return Ok(Direction::Down);
            } else if row1 > row2 {
                return Ok(Direction::Up);
            }
        }
        if row1 == row2 {
            if col1 > col2 {
                return Ok(Direction::Left);
            } else if col1 < col2 {
                return Ok(Direction::Right);
            }
        }
        anyhow::bail!("Could not determine the cardinal direction")
    }
}


fn traverse_garden_plot_part1(starting_row: i64, starting_col: i64, garden_map: &Vec<Vec<char>>) -> anyhow::Result<GardenRegionStatistics> {
    let region_symbol = garden_map.get(starting_row as usize).ok_or(anyhow!("Row {} is out of bounds", starting_row))?.get(starting_col as usize).ok_or(anyhow!("Col {} is out of bounds", starting_col))?;
    let mut region_items = HashSet::new();
    let mut surrounding_items = 0;
    let mut traversal_stack = vec![(starting_row, starting_col)];
    while let Some((current_row, current_col))  = traversal_stack.pop() {
        if current_row < 0 || current_col < 0 || current_row  >= garden_map.len() as i64 || current_col >= garden_map[0].len() as i64 {
            //The traversal has gone off the map, but we still log it as a perimeter (off the map is still technically a boundary)
            surrounding_items += 1;
            continue;
        }
        let current_symbol = garden_map.get(current_row as usize).unwrap().get(current_col as usize).unwrap();
        if current_symbol == region_symbol {
            if region_items.contains(&(current_row, current_col)) {
                //In a cycle, so kill this probe
                continue;
            }
            //Still in the garden plot, so spawn new probes to continue exploring
            region_items.insert((current_row, current_col));
            traversal_stack.extend([
                (current_row, current_col + 1),
                (current_row, current_col - 1),
                (current_row - 1, current_col), 
                (current_row + 1, current_col)
            ]);
        } else {
            surrounding_items += 1;
        }
    }
    Ok(GardenRegionStatistics {
        perimeter: surrounding_items,
        area: region_items.len(),
        region_elements: region_items
    })
}

impl SolveAdvent for Day12 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let garden_map = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut visited = HashSet::new();
        let mut total_fence_price = 0 ;
        for row_number in 0..garden_map.len() {
            for col_number in 0..garden_map[0].len() {
                let col_number = col_number as i64;
                let row_number = row_number as i64;
                if visited.contains(&(row_number, col_number)) {
                    continue;
                }
                let plot_statistics = traverse_garden_plot_part1(row_number, col_number, &garden_map)?;
                total_fence_price += plot_statistics.area * plot_statistics.perimeter;
                visited.extend(plot_statistics.region_elements);
            }
        }
        println!("Total fence cost is {}", total_fence_price);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}