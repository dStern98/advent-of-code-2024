
use std::collections::HashMap;

use anyhow::{anyhow, Context};

use super::{read_input_file, SolveAdvent};

pub struct  Day5;

#[derive(Debug, Clone)]
struct SleighLauncher {
    ///The ordering rules of the problem.
    update_rules: Vec<(i64, i64)>, 
    ///the parsed updates from the input file. Note that these updates have
    /// not been checked to see if they are valid.
    potential_updates: Vec<Vec<i64>>
}

impl SleighLauncher {
    fn is_valid_update(update_rules: &[(i64, i64)], update: &[i64]) -> bool {
        //! Check if a row follows all of the ordering rules.
        
        //The inverted_map is a hashmap of value to index. In essence, when the problem discusses proper ordering all we
        //need to know if whether the index position of the lower number is less than the index of the upper number.
        //Otherwise a violation in the rules has occured.
        let inverted_map = update.iter().enumerate().map(|(idx, value)| (*value, idx)).collect::<HashMap<_, _>>();
        for (lower, upper) in update_rules.iter() {
            if let Some(lower_idx) = inverted_map.get(lower) {
                if let Some(upper_idx) = inverted_map.get(upper) {
                    if upper_idx < lower_idx {
                        return false
                    }
                }
            }
        }
    true
    }

    fn maybe_swap(inverted_map: &HashMap<i64, usize>, lower: &i64, upper: &i64) -> Option<(usize, usize)> {
        //! If a swap is needed in order to obey the update rules, returns the lower/upper indexes
        //! to be swapped. This shortens the immutable borrow of the `inverted_map` so that the map 
        //! itself can then be updated.
        if let Some(lower_idx) = inverted_map.get(lower) {
            if let Some(upper_idx) = inverted_map.get(upper) {
                if upper_idx < lower_idx {
                    return Some((*lower_idx, *upper_idx))
                }
            }
        }
        None
    }

    fn fix_bad_update(update_rules: &[(i64, i64)], update: &mut [i64]) {
        //! So long as the update is not valid, continue to swap any pair of numbers that 
        //! violate a rule. The outer while loop gurantees that if the function returns, the `update`
        //! is guranteed to be valid. It is not obvious to me whether or not this loop could potentially be infinite
        //! but it works for the problem input so hopefully it works in all possible cases without becoming infinite!
        let mut inverted_map = update.iter().enumerate().map(|(idx, value)| (*value, idx)).collect::<HashMap<_, _>>();
        while !SleighLauncher::is_valid_update(update_rules, update) {
            for (lower, upper) in update_rules.iter() {
                if let Some((lower_idx, upper_idx)) = SleighLauncher::maybe_swap(&inverted_map, lower, upper) {
                    //If a `lower/upper` index needs to be swapped, then swap them in the `update` array.
                    //In addition, update the index values in the `inverted_map`
                    update.swap(lower_idx, upper_idx);
                    inverted_map.insert(*lower, upper_idx);
                    inverted_map.insert(*upper, lower_idx);
                };
            }
        }
    }
}

fn parse_and_preprocess_input(file_contents: &str) -> anyhow::Result<SleighLauncher> {
    //! Preprocess the input file contents into a `SleighLauncher`
    let mut finished_rules = false;
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for line in file_contents.lines() {
        if line.is_empty() {
            finished_rules = true;
            continue;
        }
        if !finished_rules {
            let [num1, num2]: [&str; 2] = line.trim().split('|').collect::<Vec<_>>().try_into().map_err(|_err| anyhow!("Failed to coerce into array of length 2"))?;
            let num1 = num1.parse::<i64>()?;
            let num2 = num2.parse::<i64>()?;
            rules.push((num1, num2));
        } else {
            let page_updates: Result<Vec<_>, _> = line.trim().split(',').map(|num| num.parse::<i64>()).collect();
            let page_updates = page_updates.context("Failed to process row into list of page updates")?;
            updates.push(page_updates);

        }
    }
    Ok(SleighLauncher{
        update_rules: rules,
        potential_updates: updates
    })
}

impl SolveAdvent for Day5 {
    fn solve_part1(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let sleigh_launcher = parse_and_preprocess_input(&file_contents)?;
        let mut total_middle_numbers = 0;
        for potential_update in sleigh_launcher.potential_updates {
            if SleighLauncher::is_valid_update(&sleigh_launcher.update_rules, &potential_update) {
                total_middle_numbers += *potential_update.get(potential_update.len() / 2).unwrap();
            } 
        }
        println!("Sum of all middle page numbers is {}", total_middle_numbers);
        Ok(())
    }

    fn solve_part2(path_to_file: &str) -> anyhow::Result<()> {
        let file_contents = read_input_file(path_to_file)?;
        let mut sleigh_launcher = parse_and_preprocess_input(&file_contents)?;
        let mut total_middle_numbers = 0;
        for potential_update in sleigh_launcher.potential_updates.iter_mut() {
            if !SleighLauncher::is_valid_update(&sleigh_launcher.update_rules, potential_update) {
                SleighLauncher::fix_bad_update(&sleigh_launcher.update_rules, potential_update);
                total_middle_numbers += *potential_update.get(potential_update.len() / 2).unwrap();
            } 
        }
        println!("Sum of middle page numbers after fix is {}", total_middle_numbers);
        Ok(())
    }
}