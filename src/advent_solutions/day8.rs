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
    fn generate_antinodes_without_distance_bound(&self, other: &Self, city_map: &[Vec<char>]) -> Vec<OrderedPair> {
        //! Without the antinode distance bound, we generate antinodes forever until we go off the map.
        
        //The closure stops the loops when we go off the edge of the city map
        let is_off_the_map = |row: i64, col: i64| -> bool {
            row < 0 || row >= city_map.len() as i64 || col < 0|| col >= city_map[0].len() as i64
        };
        let mut antinode_locations = Vec::new();
        let col_delta = self.col - other.col;
        let row_delta = self.row - other.row;
        //Generate all possible antinodes based off the position of `self`
        let mut scale_factor1 = 0;
        while !is_off_the_map(self.row + scale_factor1 * row_delta, self.col + scale_factor1 * col_delta) {
            antinode_locations.push((self.row + scale_factor1 * row_delta, self.col + scale_factor1 * col_delta));
            scale_factor1 += 1;
        }
        //Generate all possible antinodes based off the position of `other` in the antenna pair.
        let mut scale_factor2 = 0;
        while !is_off_the_map(other.row - scale_factor2 * row_delta, other.col - scale_factor2 * col_delta) {
            antinode_locations.push((other.row - scale_factor2 * row_delta, other.col - scale_factor2 * col_delta));
            scale_factor2 += 1;
        }
        antinode_locations
    }
    fn generate_antinodes_with_distance_bound(&self, other: &Self, city_map: &[Vec<char>]) -> Vec<OrderedPair>  {
        //! Generate the two antinodes for a given pair of antenna that are in line with the two antennas, 
        //! and obey the rule that the antinode must be twice as far from one antennas as the other.
        //! It turns out that simply replicating the delta in the columns/rows between the two antennas will satisfy
        //! the requirements.  
        //! The returned ordered pairs are guranteed to be in the map!
        let is_off_the_map = |row: i64, col: i64| -> bool {
            row < 0 || row >= city_map.len() as i64 || col < 0|| col >= city_map[0].len() as i64
        };
        let col_delta = self.col - other.col;
        let row_delta = self.row - other.row;
        //With the distance bound in place, the two possible antinodes are as simple as this to express!
        let node1 = (self.row + row_delta, self.col + col_delta);
        let node2 = (other.row - row_delta, other.col - col_delta);
        [node1, node2].into_iter().filter(|(row, col)| !is_off_the_map(*row, *col)).collect::<Vec<_>>()
    }

}



fn generate_all_possible_antinodes(antennas: &[Antenna], city_map: &[Vec<char>], apply_distance_bound: bool) -> HashSet<OrderedPair> {
    //! Examine all possible pairs of antinodes from the passed in slice of `Antenna`. The `city_map` is used to check
    //! if the antinode position is inside the city map, and also to stop the antinode generation when there is no distance bound.
    let mut unique_ordered_pairs = HashSet::new();
    for antenna_number in 0..antennas.len() - 1 {
        for other_antenna_number in antenna_number+ 1..antennas.len() {
            let antenna_1 = antennas[antenna_number];
            let antenna_2 = antennas[other_antenna_number];
            if apply_distance_bound {
                unique_ordered_pairs.extend(antenna_1.generate_antinodes_with_distance_bound(&antenna_2, city_map));
            }
            else {
                unique_ordered_pairs.extend(antenna_1.generate_antinodes_without_distance_bound(&antenna_2, city_map));
            }
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
        //! Count unique antinodes with the distance rule in place (the antinode must be exactly twice the distance from
        //! one antenna as the other).
        let file_contents = read_input_file(path_to_file)?;
        let city_map = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let antenna_map = construct_antenna_map(&city_map);
        let mut unique_antinodes = HashSet::new();
        for antenna_group in antenna_map.values() {
            unique_antinodes.extend( generate_all_possible_antinodes(antenna_group, &city_map, true));           
        }
        println!("There are {} unique antinode positions in the city", unique_antinodes.len());
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        //! Generate unique antinodes without any distance constraint.
        let file_contents = read_input_file(path_to_file)?;
        let city_map = file_contents.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        let antenna_map = construct_antenna_map(&city_map);
        let mut unique_antinodes = HashSet::new();
        for antenna_group in antenna_map.values(){
            unique_antinodes.extend(generate_all_possible_antinodes(antenna_group, &city_map, false));           
        }
        println!("There are {} unique antinode positions in the city", unique_antinodes.len());
        Ok(())
    }
}