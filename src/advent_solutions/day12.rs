use std::collections::HashSet;
use std::cmp::Ordering;

use super::{read_input_file, SolveAdvent};

pub struct Day12;

type OrderedPair = (i64, i64);

///Represents the key information regarding 
/// a single continous garden region in the plot.
#[derive(Debug, Clone)]
struct GardenRegion {
    ///Represents all perimeter boundaries of the region
    perimeter: Vec<PerimeterFenceMarker>,
    ///The unique elements of the region
    region_elements: HashSet<(i64, i64)>
}

impl GardenRegion {
    fn get_part1_fence_price(&self) -> usize {
        //! Part1 fence price is just area * perimeter
        self.region_elements.len() * self.perimeter.len()
    }
}

///All 4 possible cardinal directions a perimeter
/// could `face`. Diagonals are not allowed here.
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down, 
    Right, 
    Left
}

///In order to comprehensively mark 
/// region perimeters, we mark the inbound position 
/// and the outbound position, as well as the cardinal direction this
/// boundary faces.
#[derive(Debug, Clone, Copy)]
struct PerimeterFenceMarker {
    #[allow(dead_code)]
    inbounds: OrderedPair,
    #[allow(dead_code)]
    outbounds: OrderedPair,
    #[allow(dead_code)]
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
        //! mark the cardinal direction of the flow. This is important for assessing 
        //! when the perimeter edge has changed.
        let (row1, col1) = pair1;
        let (row2, col2) = pair2;
        if col1 == col2 {
            match row1.cmp(&row2) {
                Ordering::Less => return Ok(Direction::Down),
                Ordering::Greater => return Ok(Direction::Up),
                _ => {}
            }
        }
        if row1 == row2 {
            match col1.cmp(&col2) {
                Ordering::Less => return Ok(Direction::Right),
                Ordering::Greater =>return  Ok(Direction::Left),
                _ => {}
            }
        }
        anyhow::bail!("Could not determine the cardinal direction")
    }
}

fn safe_map_read<T>(row: i64, col: i64, garden_map: &[Vec<T>]) -> anyhow::Result<&T> 
{
    //! Safely read an item from the `garden_map` without any of the boiler plate.
    if row < 0 || col < 0 {
        anyhow::bail!("Row or col was negative, but it must be a usize!");
    }
    if let Some(row) = garden_map.get(row as usize) {
        if let Some(item) = row.get(col as usize) {
            return Ok(item);
        }
    }
    anyhow::bail!("Row {} col {} does not exist in passed in garden map", row, col);
}

///A GardenProbe is a convenient data structure used
/// to probe and explore the garden region.
struct GardenProbe {
    ///The previous position this probe just visited, if it exists
    last_position: Option<OrderedPair>,
    ///Current probe position.
    current_position: OrderedPair
}

impl GardenProbe {
    fn out_of_bounds(&self, garden_map: &[Vec<char>]) -> bool {
        //! Is the garden probe's current position out of bounds?
        let (current_row, current_col) = (self.current_position.0, self.current_position.1);
        if current_row < 0 || current_col < 0 || current_row  >= garden_map.len() as i64 || current_col >= garden_map[0].len() as i64 {
            return true
        }
        false
    }

    fn spawn_4(&self) -> impl IntoIterator<Item = Self> {
        //! Returns 4 new probes to continue probing in all 4 cardinal directions
        //! from the current probe's location: Up, down, left, right.
        [
                GardenProbe {
                    last_position: Some(self.current_position),
                    current_position: (self.current_position.0, self.current_position.1 + 1)
                },
                GardenProbe {
                    last_position: Some(self.current_position),
                    current_position: (self.current_position.0, self.current_position.1 - 1)
                },
                GardenProbe {
                    last_position: Some(self.current_position),
                    current_position: (self.current_position.0 + 1, self.current_position.1)
                },
                GardenProbe {
                    last_position: Some(self.current_position),
                    current_position: (self.current_position.0 - 1, self.current_position.1)
                },
            ]
    }
}



fn process_garden_region(starting_row: i64, starting_col: i64, garden_map: &[Vec<char>]) -> anyhow::Result<GardenRegion> {
    //! Analyze a garden region starting from the `starting_row`, `starting_col` position.
    let region_symbol = safe_map_read(starting_row, starting_col, garden_map)?;
    let mut region_elements = HashSet::new(); //All positions in this region being explored
    let mut perimeter_crossings = Vec::new(); // Fences that mark the perimeter of the region.
    let mut traversal_stack = vec![GardenProbe {
        current_position: (starting_row, starting_col),
        last_position: None
    }
    ];
    while let Some(current_probe)  = traversal_stack.pop() {
        if current_probe.out_of_bounds(garden_map) && current_probe.last_position.is_some() {
                //If the current probe is out of bounds, we have identified a perimeter of the region, which we note.
                let fence_marker = PerimeterFenceMarker::try_new(current_probe.last_position.unwrap(), current_probe.current_position)?;
                perimeter_crossings.push(fence_marker);
                continue;
        }
        if safe_map_read(current_probe.current_position.0, current_probe.current_position.1, garden_map)? == region_symbol {
            if region_elements.contains(&current_probe.current_position) {
                //In a cycle, so kill this probe
                continue;
            }
            //We are still inside the region we want to explore, so just continue the search by spawning new probes
            region_elements.insert(current_probe.current_position);
            traversal_stack.extend(current_probe.spawn_4()
            );
        } else if current_probe.last_position.is_some() {
                //If we have reached a position of a different `region_symbol`, then we have left the region to explore, 
                //so we mark it as a fence (perimeter)
                let fence_marker = PerimeterFenceMarker::try_new(current_probe.last_position.unwrap(), current_probe.current_position)?;
                perimeter_crossings.push(fence_marker);
                continue;    
        }
    }
    Ok(GardenRegion {
        perimeter: perimeter_crossings,
        region_elements
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
                let plot_statistics = process_garden_region(row_number, col_number, &garden_map)?;
                total_fence_price += plot_statistics.get_part1_fence_price();
                visited.extend(plot_statistics.region_elements);
            }
        }
        println!("Total fence cost is {}", total_fence_price);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        //The above boilerplate does all the hard work. Completing part2 should not be too hard.
        Ok(())
    }
}