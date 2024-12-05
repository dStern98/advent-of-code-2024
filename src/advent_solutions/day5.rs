
use std::collections::HashMap;

use anyhow::{anyhow, Context};

use super::{read_input_file, SolveAdvent};

pub struct  Day5;

#[derive(Debug, Clone)]
struct SleighLauncher {
    update_rules: Vec<(i64, i64)>, 
    potential_updates: Vec<Vec<i64>>
}

impl SleighLauncher {
    fn is_valid_update(update_rules: &[(i64, i64)], update: &[i64]) -> bool {
        //! Check if a row follows all of the ordering rules.
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
            let is_valid_update = SleighLauncher::is_valid_update(&sleigh_launcher.update_rules, &potential_update);
            if is_valid_update {
                let middle_number = potential_update.get(potential_update.len() / 2).unwrap();
                total_middle_numbers += *middle_number;
            } 
        }
        println!("Sum of all middle page numbers is {}", total_middle_numbers);
        Ok(())
    }

    fn solve_part2(_path_to_file: &str) -> anyhow::Result<()> {
        Ok(())
    }
}