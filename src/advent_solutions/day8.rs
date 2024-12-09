use std::collections::{HashMap, HashSet};

use super::{read_input_file, SolveAdvent};

pub struct Day8;

#[derive(Debug, Clone, Copy)]
struct Antenna {
    row: i64,
    col: i64,
}

type OrderedPair = (i64, i64);

impl Antenna {
    fn generate_antinodes_with_distance_bound(&self, other: &Self) -> [OrderedPair;2]  {
        //! Generate the two antinodes for a given pair of antenna that are in line with the two antennas, 
        //! and obey the rule that the antinode must be twice as far from one antennas as the other.
        //! It turns out that simply replicating the delta in the columns/rows between the two antennas will satisfy
        //! the requirements.  
        //! 
        //! Importantly, this function does not check if the antinodes are on or off the map!
        let col_delta = self.col - other.col;
        let row_delta = self.row - other.row;
        let node1 = (self.row + row_delta, self.col + col_delta);
        let node2 = (other.row - row_delta, other.col - col_delta);
        [node1, node2]
    }


}

fn generate_all_possible_antinodes(antennas: &[Antenna]) -> HashSet<OrderedPair> {
    //! Examine all possible pairs of antinodes from the passed in slice of `Antenna`.
    let mut unique_ordered_pairs = HashSet::new();
    for antenna_number in 0..antennas.len() - 1 {
        for other_antenna_number in antenna_number+ 1..antennas.len() {
            let antenna_1 = antennas[antenna_number];
            let antenna_2 = antennas[other_antenna_number];
            unique_ordered_pairs.extend(antenna_1.generate_antinodes_with_distance_bound(&antenna_2));
        }
    }
    unique_ordered_pairs
}

fn construct_antenna_map(city_map: &[Vec<char>]) -> HashMap<char, Vec<Antenna>> {
    //! Construct a HashMap grouping all Antenna's by their frequency.
    let mut antenna_map: HashMap<char, Vec<Antenna>> = HashMap::new();
    for (row_number, row) in city_map.iter().enumerate() {
        for (col_number, character) in row.iter().enumerate() {
            if *character != '.' {
                antenna_map.entry(*character).or_default().push(Antenna {
                    row: row_number as i64,
                    col: col_number as i64,
                });
            }
        }
    }
    antenna_map
}

impl SolveAdvent for Day8 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let city_map = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let antenna_map = construct_antenna_map(&city_map);
        let mut unique_antinodes = HashSet::new();
        for (_frequency, antennas) in antenna_map.iter() {
            let valid_antinodes = generate_all_possible_antinodes(antennas);
            //Need to check and see if the antinode if off the map, in which case we ignore it
            for antinode in valid_antinodes {
                if antinode.0 < 0 || antinode.0 >= city_map.len() as i64 || antinode.1 < 0|| antinode.1 >= city_map[0].len() as i64 {
                    continue;
                }
                unique_antinodes.insert(antinode);
            }            
        }
        println!("There are {} unique antinode positions in the city", unique_antinodes.len());
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}